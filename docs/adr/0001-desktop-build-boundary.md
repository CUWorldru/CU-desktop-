# ADR 0001 — Separate Windows Desktop Build Boundary

- Status: Accepted for the desktop foundation
- Date: 2026-11-25

## Context
The existing CU application has a browser SSR/PWA build and a separate Capacitor SPA build for Android/iOS. The mobile build carries platform markers and Capacitor-specific assumptions. The Windows desktop app needs a bundled frontend but must not inherit mobile runtime behavior or disturb web/mobile builds.

## Decision
Use a separate `desktop` build target in the CU source repository, paired with a standalone Tauri 2 + Rust + WebView2 shell in this repository.

The desktop target must have its own build marker and runtime detection. Existing web, Android, and iOS targets remain behaviorally unchanged. Tauri-specific integration stays in this repository unless a small, explicitly reviewed shared-runtime seam is required in the CU source repository.

## Consequences
- A reviewed change in the private CU source repository will be needed before the full CU interface can be built as a desktop bundle.
- The desktop shell can be developed independently against a local asset directory while the private source remains private.
- CI must obtain private source through a protected, least-privilege mechanism and pin an exact commit.
- Desktop-specific tests must prove that web, Android, and iOS build paths remain unchanged.
- The Tauri shell must not treat every Capacitor plugin as available on Windows.

## Rejected alternatives
- Wrapping the production website URL: rejected because it depends on network availability and does not bundle the application.
- Reusing the Capacitor mobile bundle unchanged: rejected because it carries mobile-specific runtime behavior and plugins.
- Copying the full CU source into this public repository: rejected because it exposes private code and creates a divergent fork.