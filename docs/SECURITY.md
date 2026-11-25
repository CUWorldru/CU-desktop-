# CU Desktop — Security Baseline

## Public repository rules
- Never commit `.env` files, tokens, API keys, passwords, private keys, signing certificates, production data, or private CU source.
- Treat all issues, pull requests, logs, and build artifacts as public.
- Use protected GitHub Actions secrets for CI credentials; grant the minimum required repository and workflow permissions.

## Tauri security
- Start with minimal Tauri capabilities and allowlists.
- Expose only narrowly scoped Rust commands; validate all arguments and authorization context.
- Do not expose arbitrary filesystem, shell, process, or network access to the web frontend.
- The Tauri HTTP plugin is limited to `https://seeyoumore.ru/_serverFn/**`; do not widen this scope to the whole host or arbitrary URLs without a reviewed need.
- Restrict navigation to trusted application origins; handle external URLs through explicit validation.
- Validate deep links and notification payloads before routing or performing actions.
- Keep WebView content security policy restrictive and review every required exception.

## Identity and backend
- Supabase and existing CU services remain the source of truth.
- Enforce authorization on the server and preserve RLS policies.
- Never treat frontend state or desktop IPC as an authorization boundary.
- Do not introduce desktop-only copies of credentials, user records, or chat/message data.

## Updates and releases
- Sign release artifacts and updater metadata.
- Verify update signatures before installation.
- Protect signing material outside the public repository.
- Publish checksums and release notes; do not publish secrets in logs or artifacts.

## Required review before release
Threat-model IPC, deep links, notification routing, file access, external navigation, authentication callbacks, update delivery, and telemetry.