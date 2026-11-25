use crate::companion_identity::{
    companion_identity_get, companion_identity_sign, delete_credential, read_session,
    write_credential, CompanionIdentityInfo, StoredCompanionSession,
};
use chrono::{DateTime, Utc};
use reqwest::{header, Client, Method, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tauri::command;
use zeroize::Zeroize;

const API_BASE: &str = "https://seeyoumore.ru/api/companion/v1";
const SESSION_TARGET: &str = "CU.Desktop.Companion.Session.v1";
const SESSION_USER: &str = "CU Desktop Companion session";
const MAX_RESPONSE_BYTES: usize = 8 * 1024;
static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();
static SESSION_REFRESH_LOCK: Mutex<()> = Mutex::const_new(());

fn client() -> Result<&'static Client, String> {
    if let Some(client) = HTTP_CLIENT.get() {
        return Ok(client);
    }
    let built = Client::builder()
        .https_only(true)
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(std::time::Duration::from_secs(8))
        .timeout(std::time::Duration::from_secs(15))
        .user_agent("CU-Desktop-Companion/1")
        .build()
        .map_err(|_| "COMPANION_HTTP_UNAVAILABLE".to_string())?;
    let _ = HTTP_CLIENT.set(built);
    HTTP_CLIENT.get().ok_or_else(|| "COMPANION_HTTP_UNAVAILABLE".into())
}

async fn request_json<T: DeserializeOwned>(
    method: Method,
    path: &str,
    body: Option<Value>,
    authorization: Option<String>,
) -> Result<T, String> {
    let url = format!("{API_BASE}{path}");
    let mut request = client()?.request(method, url)
        .header(header::ACCEPT, "application/json")
        .header(header::CACHE_CONTROL, "no-store");
    if let Some(value) = authorization {
        request = request.header(header::AUTHORIZATION, value);
    }
    if let Some(value) = body {
        request = request.header(header::CONTENT_TYPE, "application/json").json(&value);
    }
    let mut response = request.send().await.map_err(|_| "COMPANION_HTTP_TRANSPORT_FAILED".to_string())?;
    if response.content_length().is_some_and(|length| length > MAX_RESPONSE_BYTES as u64) {
        return Err("COMPANION_HTTP_RESPONSE_INVALID".into());
    }
    let status = response.status();
    let mut bytes = Vec::with_capacity(1024);
    while let Some(chunk) = response.chunk().await
        .map_err(|_| "COMPANION_HTTP_TRANSPORT_FAILED".to_string())? {
        if bytes.len().saturating_add(chunk.len()) > MAX_RESPONSE_BYTES {
            bytes.zeroize();
            return Err("COMPANION_HTTP_RESPONSE_INVALID".into());
        }
        bytes.extend_from_slice(&chunk);
    }
    if !status.is_success() {
        bytes.zeroize();
        return Err(match status {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => "COMPANION_UNAUTHORIZED",
            StatusCode::TOO_MANY_REQUESTS => "COMPANION_RATE_LIMITED",
            StatusCode::SERVICE_UNAVAILABLE => "COMPANION_API_DISABLED_OR_UNAVAILABLE",
            _ => "COMPANION_REQUEST_REJECTED",
        }.into());
    }
    let parsed = serde_json::from_slice::<T>(&bytes)
        .map_err(|_| "COMPANION_HTTP_RESPONSE_INVALID".to_string());
    bytes.zeroize();
    parsed
}

fn expiry_unix(value: &str) -> Result<i64, String> {
    let parsed = DateTime::parse_from_rfc3339(value)
        .map_err(|_| "COMPANION_RESPONSE_INVALID".to_string())?;
    let timestamp = parsed.with_timezone(&Utc).timestamp();
    if timestamp <= Utc::now().timestamp() || timestamp > 9_007_199_254_740_991 {
        return Err("COMPANION_RESPONSE_INVALID".into());
    }
    Ok(timestamp)
}

#[derive(Deserialize)]
struct ChallengeCreated {
    challenge_id: String,
    challenge_secret: String,
    key_fingerprint: String,
    server_nonce: String,
    expires_at: String,
    poll_after_seconds: u64,
}

#[derive(Serialize)]
pub struct PairingQrPayload {
    protocol: &'static str,
    challenge_id: String,
    challenge_secret: String,
    expires_at: String,
    device_name: String,
}

#[derive(Serialize)]
pub struct PairingStartResult {
    qr_payload: PairingQrPayload,
    key_fingerprint: String,
    poll_after_seconds: u64,
}

#[command]
pub async fn companion_pairing_start(device_name: String) -> Result<PairingStartResult, String> {
    let name = device_name.trim();
    if name.is_empty() || name.len() > 80 {
        return Err("COMPANION_DEVICE_NAME_INVALID".into());
    }
    let identity: CompanionIdentityInfo = companion_identity_get()?;
    let created: ChallengeCreated = request_json(
        Method::POST,
        "/pairing-challenges",
        Some(json!({"device_name": name, "public_key": identity.public_key})),
        None,
    ).await?;
    expiry_unix(&created.expires_at)?;
    if created.challenge_id.is_empty() || created.challenge_secret.is_empty()
        || created.server_nonce.is_empty() || created.key_fingerprint != identity.key_fingerprint {
        return Err("COMPANION_RESPONSE_INVALID".into());
    }
    Ok(PairingStartResult {
        qr_payload: PairingQrPayload {
            protocol: "CU-COMPANION-PAIR-V1",
            challenge_id: created.challenge_id,
            challenge_secret: created.challenge_secret,
            expires_at: created.expires_at,
            device_name: name.to_string(),
        },
        key_fingerprint: created.key_fingerprint,
        poll_after_seconds: created.poll_after_seconds.clamp(1, 10),
    })
}

#[derive(Deserialize, Serialize)]
pub struct PairingStatus {
    pub status: Option<String>,
    pub device_name: Option<String>,
    pub expires_at: Option<String>,
    pub approved_at: Option<String>,
    #[serde(default)]
    pub proof_verified: bool,
}

fn validate_pairing_input(challenge_id: &str, challenge_secret: &str) -> Result<(), String> {
    if challenge_id.len() != 36 || challenge_secret.len() != 43
        || !challenge_id.bytes().all(|b| b.is_ascii_hexdigit() || b == b'-')
        || !challenge_secret.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-') {
        return Err("COMPANION_PAIRING_INPUT_INVALID".into());
    }
    Ok(())
}

#[command]
pub async fn companion_pairing_poll(
    challenge_id: String,
    challenge_secret: String,
) -> Result<PairingStatus, String> {
    validate_pairing_input(&challenge_id, &challenge_secret)?;
    request_json(
        Method::GET,
        &format!("/pairing-challenges/{challenge_id}"),
        None,
        Some(format!("Companion-Pairing {challenge_secret}")),
    ).await
}

#[command]
pub async fn companion_pairing_submit_proof(
    challenge_id: String,
    challenge_secret: String,
    server_nonce: String,
    key_fingerprint: String,
    expires_at: String,
) -> Result<PairingStatus, String> {
    validate_pairing_input(&challenge_id, &challenge_secret)?;
    let status: PairingStatus = request_json(
        Method::GET,
        &format!("/pairing-challenges/{challenge_id}"),
        None,
        Some(format!("Companion-Pairing {challenge_secret}")),
    ).await?;
    if status.status.as_deref() != Some("approved") {
        return Err("COMPANION_PAIRING_NOT_APPROVED".into());
    }
    let expiry = expiry_unix(&expires_at)?;
    let identity: CompanionIdentityInfo = companion_identity_get()?;
    if identity.key_fingerprint != key_fingerprint {
        return Err("COMPANION_PROOF_KEY_MISMATCH".into());
    }
    let signature = companion_identity_sign(
        challenge_id.clone(), expiry, key_fingerprint, server_nonce
    )?;
    let _: Value = request_json(
        Method::POST,
        &format!("/pairing-challenges/{challenge_id}/proof"),
        Some(json!({"algorithm": "Ed25519", "signature": signature})),
        Some(format!("Companion-Pairing {challenge_secret}")),
    ).await?;
    request_json(
        Method::GET,
        &format!("/pairing-challenges/{challenge_id}"),
        None,
        Some(format!("Companion-Pairing {challenge_secret}")),
    ).await
}

#[derive(Deserialize)]
struct ExchangeResponse {
    access_token: String,
    access_expires_at: String,
    refresh_token: String,
    refresh_expires_at: String,
    session_id: String,
    device: ExchangeDevice,
    generation: u64,
}
#[derive(Deserialize)]
struct ExchangeDevice {
    device_id: String,
    device_name: String,
}
#[derive(Deserialize)]
struct RefreshResponse {
    access_token: String,
    access_expires_at: String,
    refresh_token: String,
    refresh_expires_at: String,
    session_id: String,
    device_id: String,
    generation: u64,
}

fn persist_session(
    access_token: String,
    access_expires_at: String,
    refresh_token: String,
    refresh_expires_at: String,
    session_id: String,
    device_id: String,
    device_name: String,
    generation: u64,
) -> Result<(), String> {
    expiry_unix(&access_expires_at)?;
    expiry_unix(&refresh_expires_at)?;
    if access_token.len() != 43 || refresh_token.len() != 43
        || session_id.len() != 36 || device_id.len() != 36
        || device_name.trim().is_empty() || device_name.len() > 80 {
        return Err("COMPANION_RESPONSE_INVALID".into());
    }
    let session = StoredCompanionSession {
        access_token,
        access_expires_at,
        refresh_token,
        refresh_expires_at,
        session_id,
        device_id,
        device_name,
        generation,
    };
    let mut encoded = serde_json::to_vec(&session)
        .map_err(|_| "COMPANION_SESSION_STORE_WRITE_FAILED".to_string())?;
    let result = write_credential(SESSION_TARGET, SESSION_USER, &encoded);
    encoded.zeroize();
    result
}

#[command]
pub async fn companion_pairing_exchange(
    challenge_id: String,
    challenge_secret: String,
) -> Result<(), String> {
    validate_pairing_input(&challenge_id, &challenge_secret)?;
    let response: ExchangeResponse = request_json(
        Method::POST,
        "/sessions/exchange",
        Some(json!({"challenge_id": challenge_id, "challenge_secret": challenge_secret})),
        None,
    ).await?;
    persist_session(
        response.access_token, response.access_expires_at,
        response.refresh_token, response.refresh_expires_at,
        response.session_id, response.device.device_id,
        response.device.device_name, response.generation,
    )
}

#[command]
pub async fn companion_session_refresh() -> Result<(), String> {
    let _guard = SESSION_REFRESH_LOCK.lock()
        .map_err(|_| "COMPANION_SESSION_LOCK_FAILED".to_string())?;
    let mut current = read_session()?.ok_or_else(|| "COMPANION_SESSION_NOT_FOUND".to_string())?;
    let refresh = current.refresh_token.clone();
    let response = request_json::<RefreshResponse>(
        Method::POST,
        "/sessions/refresh",
        Some(json!({"refresh_token": refresh})),
        None,
    ).await;
    current.access_token.zeroize();
    current.refresh_token.zeroize();
    let response = match response {
        Ok(value) => value,
        Err(error) => {
            // Refresh tokens are single-use. A transport or parse failure is ambiguous:
            // discard local credentials rather than ever retrying the old token.
            let _ = delete_credential(SESSION_TARGET);
            return Err(error);
        }
    };
    let stored = persist_session(
        response.access_token, response.access_expires_at,
        response.refresh_token, response.refresh_expires_at,
        response.session_id, response.device_id,
        current.device_name.clone(), response.generation,
    );
    if stored.is_err() {
        let _ = delete_credential(SESSION_TARGET);
    }
    stored
}

#[command]
pub async fn companion_session_revoke() -> Result<(), String> {
    let _guard = SESSION_REFRESH_LOCK.lock()
        .map_err(|_| "COMPANION_SESSION_LOCK_FAILED".to_string())?;
    let mut current = read_session()?.ok_or_else(|| "COMPANION_SESSION_NOT_FOUND".to_string())?;
    let token = current.access_token.clone();
    let result: Result<Value, String> = request_json(
        Method::DELETE,
        "/sessions/current",
        None,
        Some(format!("Bearer {token}")),
    ).await;
    current.access_token.zeroize();
    current.refresh_token.zeroize();
    match result {
        Ok(_) => delete_credential(SESSION_TARGET),
        Err(error) => Err(error),
    }
}
