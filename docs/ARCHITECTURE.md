# CU Desktop — Architecture

## Product
CU Desktop is the Windows desktop application for the SeeYouMore / CU ecosystem within 321 CENTURY.

It is one product with two integrated experiences:
- **CU mode:** the existing CU experience: messaging, calls, contacts, channels, CU News, routes, excursions, AI, translation, and other existing modules.
- **Work mode:** a desktop-oriented workspace for multitasking, multiple work areas, side-by-side conversations and content, files/documents, and contextual AI.

Both modes share one identity, authentication session, user state, and backend. They are not separate applications.

## Required Windows behavior
- Background operation and a system tray.
- Closing the main window hides the app to the tray by default; an explicit Quit command exits it.
- Native notifications with navigation to the relevant conversation or item.
- Calls, microphone/camera access, and audio-device handling, subject to Windows validation.
- File open/save flows.
- User-controlled startup behavior.
- Signed application updates.

## Proposed desktop shell
The initial candidate is **Tauri 2 + Rust + WebView2**, reusing the existing React and TypeScript CU interface. This is a proposal, not a compatibility certification.

Before implementation, validate the actual CU SPA build, asset routing, authentication callbacks, deep links, hidden-window Realtime behavior, Windows notifications, LiveKit calls and media devices, file handling, permissions, and updater signing. Do not assume browser/native modules work unchanged in WebView2.

## Backend and source of truth
The existing CU backend remains authoritative. Reuse Supabase Auth, Database, Storage, Realtime, existing APIs and server-side authorization, LiveKit infrastructure, and canonical message/inbox/channel/notification models.

Do not create parallel user stores, message pipelines, chat caches, or authorization paths for desktop.

## Repository boundary and source integration
This repository owns the Windows shell, Rust integration, desktop-specific UX and Work mode, Windows build/package/sign/release automation, and desktop tests/docs.

The existing CU repository remains the source of the shared application. Shared-interface changes must be proposed and reviewed there explicitly. Never silently copy private CU source into this public repository.

The integration must pin an exact CU revision. Because the CU source repository is private and this repository is public, CI access requires a narrowly scoped protected secret or another explicitly approved private-source delivery mechanism. Never publish private source or credentials in this repository, build artifacts, or logs.

## Security
- No credentials, API keys, environment files, signing certificates, or private source in this public repository.
- Use least-privilege Tauri capabilities.
- Keep IPC commands narrow, validated, and explicitly authorized.
- Restrict navigation and external URL handling.
- Validate deep links and notification payloads before acting on them.
- Verify updater signatures.
- Keep secrets in protected GitHub Actions secrets or an approved secret manager.
- Preserve server-side authorization and RLS; the desktop UI is not a security boundary.

## Delivery stages
1. Repository and architecture foundation.
2. Compatibility spike: build the existing CU UI locally and render it in a minimal Tauri/WebView2 shell.
3. Window lifecycle, tray, background behavior, and notifications.
4. Deep links, file handling, and media/call validation.
5. Work mode foundation.
6. Windows x64 packaging, signing, and update pipeline.
7. Security, regression, and release validation.

## Explicit non-goals
- Do not modify the main CU repository as part of this foundation work.
- Do not change Android or iOS behavior.
- Do not replace the existing backend.
- Do not fork or duplicate the full CU application.
- Do not claim compatibility until features have been tested on Windows.
