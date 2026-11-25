# CU Desktop — Roadmap

## Phase 0 — Foundation
- [x] Confirm separate repository and product boundary.
- [x] Record architecture, UX, security, and release principles.
- [x] Inspect the CU source build configuration without modifying the source repository.
- [x] Confirm the current CU app has a dedicated Capacitor SPA build but no Tauri/desktop target.
- [x] Record the decision not to reuse the mobile build unchanged.

## Phase 1 — Desktop compatibility seam
- [ ] Agree and implement a separate desktop build target in the private CU source repository through an explicit reviewed change.
- [ ] Add a desktop-specific build marker and runtime detector; do not broaden Capacitor-only `isNativeAppRuntime()` semantics.
- [ ] Keep web SSR/PWA, Android, and iOS build paths unchanged.
- [ ] Confirm the private-source delivery approach and pin an exact source revision.
- [ ] Build a local static desktop bundle and verify TanStack Router asset/fallback behavior.
- [ ] Validate auth persistence, OAuth/callbacks, Supabase Realtime, hidden-window lifecycle, and deep links.
- [ ] Validate LiveKit microphone, camera, speaker selection, and call lifecycle on Windows.

## Phase 2 — Tauri shell
- [x] Bootstrap the initial Tauri 2 + Rust + Vite shell in this repository.
- [ ] Load the locally bundled CU desktop assets (not the production website URL); the current frontend is only a shell smoke-test screen.
- [x] Implement initial close-to-tray, restore, and explicit Quit behavior.\n- [ ] Add single-instance behavior and test lifecycle on Windows.
- [x] Add the native notification plugin and a permission/send smoke-test control.\n- [ ] Validate notification delivery and click-through routing on Windows.
- [ ] Implement startup controls and secure file open/save flows.

## Phase 3 — Work mode
- [ ] Define workspace and panel interaction model.
- [ ] Add multiple work areas without duplicating CU data stores.
- [ ] Add contextual AI entry points with explicit context and permission boundaries.

## Phase 4 — Release readiness
- [ ] Windows x64 installer and code signing.
- [ ] Signed updater and rollback validation.
- [ ] CI build, automated checks, and Windows smoke tests.
- [ ] Security, accessibility, performance, and regression checks.

## Scope guardrails
Do not modify the main CU repository as part of this desktop-repository work. Any shared CU build/runtime changes require a separate, explicit, reviewed change in the private source repository. Android and iOS behavior must remain unchanged.