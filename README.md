# CU Desktop

**CU Desktop** is the Windows desktop application for the SeeYouMore / CU ecosystem, part of 321 CENTURY.

It combines the full CU experience with a dedicated desktop Work mode.

## Product direction
- Full CU experience: chats, calls, contacts, channels, CU News, travel services, AI, translation, and existing modules.
- Work mode: desktop workspace for multitasking, multiple work areas, side-by-side conversations and content, files, and contextual AI.
- Windows integration: background operation, system tray, native notifications, startup controls, file flows, and signed updates.

## Current build pipeline
- The private CU repository contains the dedicated `BUILD_TARGET=desktop` SPA build.
- This public repository contains the Tauri 2 / Rust Windows shell and its Windows packaging workflow.
- On a trusted push to `main` or manual workflow dispatch, GitHub Actions checks out a **pinned commit** from the private CU repository using the `CU_SOURCE_READ_TOKEN` secret, builds `dist/client`, stages the generated frontend into Tauri's `dist`, and creates a Windows NSIS installer.
- The private CU source is never committed or copied into this public Git repository. Only the compiled frontend is included in the installer artifact.
- Pull-request checks do not receive or use the private-source token.

## One-time GitHub configuration
Add these Actions repository secrets in **Settings → Secrets and variables → Actions → New repository secret**:

Required:
- `CU_SOURCE_READ_TOKEN` — fine-grained token with **Contents: Read-only** access to `CUWorldru/journey-structure-import-82fde408` only.
- `CU_SUPABASE_PUBLISHABLE_KEY` — CU's existing Supabase publishable/anon key.
- `CU_SUPABASE_PROJECT_ID` — CU's Supabase project reference.

Optional client configuration (features that use these providers will be unavailable without them):
- `CU_MAPBOX_TOKEN`
- `CU_FIREBASE_API_KEY`
- `CU_FIREBASE_AUTH_DOMAIN`
- `CU_FIREBASE_PROJECT_ID`
- `CU_FIREBASE_STORAGE_BUCKET`
- `CU_FIREBASE_MESSAGING_SENDER_ID`
- `CU_FIREBASE_APP_ID`
- `CU_FIREBASE_MEASUREMENT_ID`
- `CU_FIREBASE_VAPID_KEY`

Only publishable client configuration belongs in these build secrets. Never add service-role keys, database passwords, AI provider keys, Telegram bot tokens, or other server secrets.

The private CU source ref is pinned in `.github/workflows/windows-ci.yml`. Update that SHA deliberately when a newer CU commit is approved for the Windows build.

## Getting the first installer
1. Configure the required secrets above.
2. Open **Actions → CU Desktop — Windows CI → Run workflow** on `main`.
3. Wait for the `Build installable CU Desktop (Windows x64)` job.
4. Download the `CU-Desktop-Windows-x64` artifact and run the generated `.exe` installer on Windows.

The first build is a test build, not a signed production release. Windows SmartScreen may show an unknown-publisher warning until a code-signing certificate is configured.

## Architecture and decisions
- [Architecture](docs/ARCHITECTURE.md)
- [CU compatibility assessment](docs/CU-COMPATIBILITY.md)
- [Desktop UX](docs/DESKTOP-UX.md)
- [Security baseline](docs/SECURITY.md)
- [Roadmap](docs/ROADMAP.md)
- [Release process](docs/RELEASES.md)
- [Build and validation status](docs/BUILD-STATUS.md)
- [ADR 0001: separate desktop build boundary](docs/adr/0001-desktop-build-boundary.md)

## Repository boundary
This repository contains desktop-specific code and documentation. The existing CU application and backend remain authoritative. Never commit private CU source, credentials, API keys, environment files, signing certificates, or production secrets here.
