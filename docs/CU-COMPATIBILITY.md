# CU Integration — Compatibility Assessment

## Scope
This assessment records a read-only inspection of the current CU application source and the isolated Windows desktop integration work. The private CU source `main` has not been changed by this public repository.

## Confirmed source characteristics
- The application is built with React, TypeScript, Vite, and TanStack Start.
- The default web build is SSR + PWA and uses the Node server output outside the hosted build sandbox.
- A dedicated `BUILD_TARGET=mobile` build exists for Capacitor Android/iOS.
- The mobile bundle is explicitly coupled to Capacitor and its native runtime conventions; it must not be reused unchanged for Windows.
- The private source now has a separate desktop build/runtime implementation on draft PR [#145](https://github.com/CUWorldru/journey-structure-import-82fde408/pull/145), branch `feat/cu-desktop-build-target`.
- That review branch adds a static desktop SPA target, separate desktop marker/runtime predicates, Tauri HTTP transport for server-function calls, and desktop startup/PWA guards.
- PR #145 is still draft and its private-repository Actions validation is failing before runner allocation (no steps executed). Therefore the source build has not yet been validated by that workflow or merged into `main`.

## Integration decision
**Do not connect the public Tauri shell to the private CU bundle until the private source change is reviewed and merged.**

The reviewed desktop target is intended to:
1. Emit a static SPA bundle suitable for a locally bundled WebView.
2. Use a dedicated desktop build marker, separate from `VITE_SYM_MOBILE_BUILD`.
3. Preserve the existing default web SSR/PWA build.
4. Preserve Android/iOS build behavior.
5. Introduce a separate desktop runtime predicate without broadening Capacitor-only semantics.
6. Route desktop server-function requests through the Tauri HTTP plugin with a narrowly scoped capability.

## Runtime boundary
The application must distinguish:
- Web browser / PWA
- Capacitor Android
- Capacitor iOS
- Tauri Windows desktop

Existing `isNativeAppRuntime()` semantics must not be broadened casually: several current flows intentionally mean Capacitor Android/iOS, not any embedded WebView. Use a separate desktop predicate and update only call sites whose behavior is verified for Windows.

## WebView and service constraints to validate
- Local bundled asset routing and SPA fallback for TanStack Router.
- Supabase Auth persistence, OAuth/callback handling, and custom URL/deep links.
- Supabase Realtime lifecycle when the main window is hidden or minimized.
- Notification delivery and click-through routing.
- LiveKit microphone, camera, speaker selection, and call lifecycle.
- File chooser, downloads, and drag/drop permissions.
- External links and secure navigation boundaries.
- CSP, storage origin, and migration behavior between desktop versions.

## Private source delivery
The CU source repository is private while this repository is public. Never copy the CU source into this repository or publish it as a public submodule, artifact, cache, or workflow log.

For local development, clone the private source repository separately and build from the reviewed desktop branch only after the source change is approved. Keep generated private assets outside this public repository. For CI, use a protected, narrowly scoped credential or an approved private artifact handoff, pin an exact source commit, redact logs, and ensure public artifacts contain only the distributable desktop app.

## Go / no-go
- **Go:** continue improving and validating the isolated Tauri shell.
- **Conditional go:** connect the CU frontend only after PR #145 is reviewed, merged, and its desktop build is validated.
- **No-go:** do not claim full CU compatibility, ship a Windows installer, or change the production CU build yet.
