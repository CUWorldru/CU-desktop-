# CU Desktop — Roadmap

## Phase 0 — Foundation
- [x] Confirm separate public repository and product boundary.
- [x] Record architecture and security principles.
- [ ] Confirm private CU source integration strategy and pinned revision workflow.

## Phase 1 — Compatibility spike
- [ ] Inspect the current CU repository build instructions and platform boundaries.
- [ ] Identify the exact browser/SPA build entry point suitable for desktop.
- [ ] Build a minimal Tauri 2 + WebView2 shell locally.
- [ ] Validate authentication, routing, assets, and session persistence.
- [ ] Validate Supabase Realtime and lifecycle behavior while the window is hidden.
- [ ] Validate LiveKit calls, microphone, camera, and audio routing.

## Phase 2 — Windows integration
- [ ] System tray and close-to-tray lifecycle.
- [ ] Native notifications and deep-link navigation.
- [ ] Startup controls and single-instance behavior.
- [ ] File open/save flows.

## Phase 3 — Work mode
- [ ] Define workspace and panel interaction model.
- [ ] Add multiple work areas without duplicating CU data stores.
- [ ] Add contextual AI entry points with explicit context and permission boundaries.

## Phase 4 — Release readiness
- [ ] Windows x64 build and installer.
- [ ] Code signing and signed updater.
- [ ] CI build and smoke tests.
- [ ] Security, accessibility, performance, and regression checks.

## Scope guardrails
Do not modify the main CU repository, Android, or iOS as part of this desktop foundation work. Shared CU changes require a separate, explicit review in the source repository.