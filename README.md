# CU Desktop

**CU Desktop** is the Windows desktop application for the SeeYouMore / CU ecosystem, part of 321 CENTURY.

It combines the full CU experience with a dedicated desktop Work mode.

## Product direction
- Full CU experience: chats, calls, contacts, channels, CU News, travel services, AI, translation, and existing modules.
- Work mode: desktop workspace for multitasking, multiple work areas, side-by-side conversations and content, files, and contextual AI.
- Windows integration: background operation, system tray, native notifications, startup controls, file flows, and signed updates.

## Architecture
See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for product boundaries, proposed technology, security requirements, and delivery stages.

## Status
Foundation and compatibility-planning phase. Tauri 2 + Rust + WebView2 is the proposed shell and must pass a compatibility spike before being treated as final.

## Repository boundary
This repository contains desktop-specific code and documentation. The existing CU application and backend remain authoritative. The private CU source must never be copied into this public repository.

## Security notice
This repository is public. Never commit credentials, API keys, environment files, signing certificates, private CU source, or production secrets.