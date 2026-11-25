use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ring::{
    digest::{digest, SHA256},
    rand::SystemRandom,
    signature::{Ed25519KeyPair, KeyPair},
};
use std::{ffi::c_void, ptr, sync::Mutex};
use zeroize::Zeroize;
use tauri::command;
use windows_sys::Win32::{
    Foundation::{GetLastError, ERROR_NOT_FOUND},
    Security::Credentials::{
        CredDeleteW, CredFree, CredReadW, CredWriteW, CREDENTIALW, CRED_PERSIST_LOCAL_MACHINE,
        CRED_TYPE_GENERIC,
    },
};

const CREDENTIAL_TARGET: &str = "CU.Desktop.Companion.Ed25519.v1";
const CREDENTIAL_USER: &str = "CU Desktop Companion identity";
const SESSION_TARGET: &str = "CU.Desktop.Companion.Session.v1";
const SESSION_USER: &str = "CU Desktop Companion session";
const UUID_CHARS: &[u8] = b"0123456789abcdef-";
const HEX_CHARS: &[u8] = b"0123456789abcdef";
static IDENTITY_LOCK: Mutex<()> = Mutex::new(());

#[derive(serde::Serialize)]
pub struct CompanionIdentityInfo {
    public_key: String,
    key_fingerprint: String,
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

fn read_private_key() -> Result<Option<Vec<u8>>, String> {
    let target = wide(CREDENTIAL_TARGET);
    let mut credential: *mut CREDENTIALW = ptr::null_mut();
    let ok = unsafe { CredReadW(target.as_ptr(), CRED_TYPE_GENERIC, 0, &mut credential) };
    if ok == 0 {
        let error = unsafe { GetLastError() };
        if error == ERROR_NOT_FOUND {
            return Ok(None);
        }
        return Err("COMPANION_IDENTITY_STORE_READ_FAILED".into());
    }

    let result = unsafe {
        let item = &*credential;
        if item.CredentialBlob.is_null() || item.CredentialBlobSize == 0 {
            Err("COMPANION_IDENTITY_STORE_INVALID".to_string())
        } else {
            Ok(std::slice::from_raw_parts(
                item.CredentialBlob,
                item.CredentialBlobSize as usize,
            )
            .to_vec())
        }
    };
    unsafe { CredFree(credential as *const c_void) };
    result.map(Some)
}

fn write_private_key(pkcs8: &[u8]) -> Result<(), String> {
    if pkcs8.is_empty() || pkcs8.len() > 2048 {
        return Err("COMPANION_IDENTITY_KEY_INVALID".into());
    }
    let target = wide(CREDENTIAL_TARGET);
    let username = wide(CREDENTIAL_USER);
    let mut credential: CREDENTIALW = unsafe { std::mem::zeroed() };
    credential.Flags = 0;
    credential.Type = CRED_TYPE_GENERIC;
    credential.TargetName = target.as_ptr() as *mut u16;
    credential.Comment = ptr::null_mut();
    credential.LastWritten = unsafe { std::mem::zeroed() };
    credential.CredentialBlobSize = pkcs8.len() as u32;
    credential.CredentialBlob = pkcs8.as_ptr() as *mut u8;
    credential.Persist = CRED_PERSIST_LOCAL_MACHINE;
    credential.AttributeCount = 0;
    credential.Attributes = ptr::null_mut();
    credential.TargetAlias = ptr::null_mut();
    credential.UserName = username.as_ptr() as *mut u16;
    let ok = unsafe { CredWriteW(&credential, 0) };
    if ok == 0 {
        Err("COMPANION_IDENTITY_STORE_WRITE_FAILED".into())
    } else {
        Ok(())
    }
}

fn load_or_create_keypair() -> Result<Ed25519KeyPair, String> {
    let _guard = IDENTITY_LOCK.lock().map_err(|_| "COMPANION_IDENTITY_LOCK_FAILED")?;
    if let Some(mut pkcs8) = read_private_key()? {
        let keypair = Ed25519KeyPair::from_pkcs8(&pkcs8)
            .map_err(|_| "COMPANION_IDENTITY_STORE_INVALID".to_string());
        pkcs8.zeroize();
        return keypair;
    }

    let rng = SystemRandom::new();
    let generated = Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(|_| "COMPANION_IDENTITY_GENERATION_FAILED")?;
    let pkcs8 = generated.as_ref();
    write_private_key(pkcs8)?;
    Ed25519KeyPair::from_pkcs8(pkcs8)
        .map_err(|_| "COMPANION_IDENTITY_GENERATION_FAILED".into())
}

fn identity_info(key: &Ed25519KeyPair) -> CompanionIdentityInfo {
    let public_key = key.public_key().as_ref();
    CompanionIdentityInfo {
        public_key: URL_SAFE_NO_PAD.encode(public_key),
        key_fingerprint: hex(&digest(&SHA256, public_key).as_ref()),
    }
}

fn hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn valid_uuid(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                b == b'-'
            } else {
                UUID_CHARS.contains(&b) && b != b'-'
            }
        })
        && value.as_bytes()[14..15].iter().all(|b| (b'1'..=b'8').contains(b))
        && matches!(value.as_bytes()[19], b'8' | b'9' | b'a' | b'b')
}

fn valid_fingerprint(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|b| HEX_CHARS.contains(&b))
}

fn valid_nonce(value: &str) -> bool {
    if value.len() != 43 {
        return false;
    }
    URL_SAFE_NO_PAD
        .decode(value)
        .map(|decoded| decoded.len() == 32 && URL_SAFE_NO_PAD.encode(decoded) == value)
        .unwrap_or(false)
}

#[command]
pub fn companion_identity_get() -> Result<CompanionIdentityInfo, String> {
    let key = load_or_create_keypair()?;
    Ok(identity_info(&key))
}

#[command]
pub fn companion_identity_sign(
    challenge_id: String,
    expires_at: i64,
    key_fingerprint: String,
    server_nonce: String,
) -> Result<String, String> {
    if !valid_uuid(&challenge_id)
        || expires_at <= 0
        || expires_at > 9_007_199_254_740_991
        || !valid_fingerprint(&key_fingerprint)
        || !valid_nonce(&server_nonce)
    {
        return Err("COMPANION_PROOF_INVALID".into());
    }

    let key = load_or_create_keypair()?;
    let own_fingerprint = hex(&digest(&SHA256, key.public_key().as_ref()).as_ref());
    if own_fingerprint != key_fingerprint {
        return Err("COMPANION_PROOF_KEY_MISMATCH".into());
    }

    // RFC 8785 JCS for the fixed primitive-only transcript: lexicographic key order.
    let transcript = format!(
        "{{\"challenge_id\":\"{}\",\"expires_at\":{},\"key_fingerprint\":\"{}\",\"protocol_version\":1,\"purpose\":\"CU-COMPANION-PAIR-V1\",\"server_nonce\":\"{}\"}}",
        challenge_id, expires_at, key_fingerprint, server_nonce
    );
    Ok(URL_SAFE_NO_PAD.encode(key.sign(transcript.as_bytes()).as_ref()))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct StoredCompanionSession {
    access_token: String,
    access_expires_at: String,
    refresh_token: String,
    refresh_expires_at: String,
    session_id: String,
    device_id: String,
    device_name: String,
    generation: u64,
}

fn valid_opaque_token(value: &str) -> bool {
    URL_SAFE_NO_PAD.decode(value)
        .map(|bytes| bytes.len() == 32 && URL_SAFE_NO_PAD.encode(bytes) == value)
        .unwrap_or(false)
}

fn write_credential(target_name: &str, username_value: &str, blob: &[u8]) -> Result<(), String> {
    if blob.is_empty() || blob.len() > 2048 {
        return Err("COMPANION_SESSION_INVALID".into());
    }
    let target = wide(target_name);
    let username = wide(username_value);
    let mut credential: CREDENTIALW = unsafe { std::mem::zeroed() };
    credential.Type = CRED_TYPE_GENERIC;
    credential.TargetName = target.as_ptr() as *mut u16;
    credential.CredentialBlobSize = blob.len() as u32;
    credential.CredentialBlob = blob.as_ptr() as *mut u8;
    credential.Persist = CRED_PERSIST_LOCAL_MACHINE;
    credential.UserName = username.as_ptr() as *mut u16;
    if unsafe { CredWriteW(&credential, 0) } == 0 {
        return Err("COMPANION_SESSION_STORE_WRITE_FAILED".into());
    }
    Ok(())
}

fn delete_credential(target_name: &str) -> Result<(), String> {
    let target = wide(target_name);
    let ok = unsafe { CredDeleteW(target.as_ptr(), CRED_TYPE_GENERIC, 0) };
    if ok == 0 {
        let error = unsafe { GetLastError() };
        if error == ERROR_NOT_FOUND { return Ok(()); }
        return Err("COMPANION_SESSION_STORE_DELETE_FAILED".into());
    }
    Ok(())
}

#[command]
pub fn companion_session_store(
    access_token: String,
    access_expires_at: String,
    refresh_token: String,
    refresh_expires_at: String,
    session_id: String,
    device_id: String,
    device_name: String,
    generation: u64,
) -> Result<(), String> {
    if !valid_opaque_token(&access_token) || !valid_opaque_token(&refresh_token)
        || !valid_uuid(&session_id) || !valid_uuid(&device_id)
        || access_expires_at.len() > 40 || refresh_expires_at.len() > 40
        || device_name.trim().is_empty() || device_name.len() > 80
    {
        return Err("COMPANION_SESSION_INVALID".into());
    }
    let session = StoredCompanionSession {
        access_token, access_expires_at, refresh_token, refresh_expires_at,
        session_id, device_id, device_name: device_name.trim().to_string(), generation,
    };
    let mut encoded = serde_json::to_vec(&session)
        .map_err(|_| "COMPANION_SESSION_INVALID".to_string())?;
    let result = write_credential(SESSION_TARGET, SESSION_USER, &encoded);
    encoded.zeroize();
    result
}

#[command]
pub fn companion_session_clear() -> Result<(), String> {
    delete_credential(SESSION_TARGET)
}

#[command]
pub fn companion_session_is_present() -> Result<bool, String> {
    let target = wide(SESSION_TARGET);
    let mut credential: *mut CREDENTIALW = ptr::null_mut();
    let ok = unsafe { CredReadW(target.as_ptr(), CRED_TYPE_GENERIC, 0, &mut credential) };
    if ok == 0 {
        let error = unsafe { GetLastError() };
        if error == ERROR_NOT_FOUND { return Ok(false); }
        return Err("COMPANION_SESSION_STORE_READ_FAILED".into());
    }
    unsafe { CredFree(credential as *const c_void); }
    Ok(true)
}
