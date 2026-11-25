# CU Desktop — Roadmap

## Phase 0 — Foundation
- [x] Confirm separate repository and product boundary.
- [x] Record architecture, UX, security, and release principles.
- [x] Inspect the CU source build configuration without modifying mobile behavior.
- [x] Record the decision not to reuse the Capacitor mobile build unchanged.
- [x] Define the native desktop UI architecture in [DESKTOP-UI-ARCHITECTURE.md](./DESKTOP-UI-ARCHITECTURE.md).

## Phase 1 — Desktop compatibility seam
- [x] Add a separate desktop build target in the private CU source repository through a reviewed change merged to main.
- [x] Add a desktop-specific build marker/runtime detector without broadening Capacitor-only `isNativeAppRuntime()` semantics.
- [x] Keep web SSR/PWA, Android, and iOS build paths isolated from the desktop target.
- [x] Pin the private CU source revision used by the Windows packaging workflow.
- [x] Build the static desktop SPA in CI and stage its assets into the Tauri bundle.
- [ ] Validate TanStack Router navigation and asset fallback in an installed Windows app.
- [ ] Validate auth persistence, OAuth/callbacks, Supabase Realtime, hidden-window lifecycle, and deep links on Windows.
- [ ] Validate LiveKit microphone, camera, speaker selection, and call lifecycle on Windows.

## Phase 2 — Tauri shell and first Windows package
- [x] Bootstrap the Tauri 2 + Rust shell.
- [x] Pass Windows x64 CI for frontend checks and Rust compilation.
- [x] Package the locally bundled CU frontend (not the production website URL).
- [x] Implement close-to-tray, explicit Quit, and restore from the tray menu or left-click.
- [x] Add the native notification plugin and a permission/send smoke-test control.
- [x] Produce the first Windows x64 NSIS installer artifact in GitHub Actions.
- [ ] Add single-instance behavior and test lifecycle on Windows.
- [ ] Validate notification delivery and click-through routing in an installed Windows build.
- [ ] Implement startup controls and secure file open/save flows.

## Phase 3 — Native desktop UI
- [x] Define the application frame, CU mode / Work mode split, pane model, breakpoints, interaction principles, and acceptance criteria.
- [ ] Implement desktop-only app rail, navigation pane, workspace canvas, inspector, and desktop design tokens in the private CU source repository.
- [ ] Adapt Chats as the first reference module with a desktop three-pane layout.
- [ ] Adapt Channels and CU News for list/detail reading layouts.
- [ ] Adapt Planner, trips, services, and excursions for wide-screen workflows.
- [ ] Adapt AI and Translator for side-by-side work surfaces.
- [ ] Implement Work mode workspaces, split panes, and local layout preference persistence.

## Phase 4 — Release readiness
- [ ] Code-sign the Windows installer.
- [ ] Add signed updater and rollback validation.
- [ ] Add CI desktop UI checks and automated Windows smoke tests.
- [ ] Complete security, accessibility, performance, and regression checks.

## Scope guardrails
The actual React desktop UI belongs in the private CU source repository, which owns the shared application and its modules. This public repository owns the Tauri/Rust shell, Windows integrations, packaging, and desktop-level documentation/tests. Do not copy private CU source into this repository. Any shared CU UI changes must be made through an explicit reviewed change in the private repository. Android and iOS behavior must remain unchanged.
