# CU Desktop

**CU Desktop** is the Windows desktop application for the SeeYouMore / CU ecosystem, part of 321 CENTURY.

It combines the full CU experience with a dedicated desktop Work mode.

## Product direction
- Full CU experience: chats, calls, contacts, channels, CU News, travel services, AI, translation, and existing modules.
- Work mode: desktop workspace for multitasking, multiple work areas, side-by-side conversations and content, files, and contextual AI.
- Windows integration: background operation, system tray, native notifications, startup controls, file flows, and signed updates.

## Current status
Foundation, compatibility assessment, and an initial Tauri 2 shell are committed. The shell includes a Rust system-tray menu, close-to-tray behavior, and a Windows notification smoke-test control. Its current Vite screen is only a diagnostic placeholder: the private CU frontend is not connected yet. The current CU source has a Capacitor SPA build for Android/iOS, but no dedicated Windows/Tauri build target. The mobile bundle will not be reused unchanged. The private CU desktop build/runtime patch is available in draft PR [#145](https://github.com/CUWorldru/journey-structure-import-82fde408/pull/145); it is not merged. The Tauri shell has a narrowly scoped HTTP bridge for the CU server-function endpoint. Windows x64 CI has passed frontend typecheck/build and Rust compilation, but the full CU frontend is not connected yet.

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
This repository contains desktop-specific code and documentation. The existing CU application and backend remain authoritative. The private CU source must never be copied into this public repository.

## Security notice
This repository is public. Never commit credentials, API keys, environment files, signing certificates, private CU source, or production secrets.