# CU Integration — Compatibility Assessment

## Scope
This assessment records a read-only inspection of the current CU application source to determine a safe Windows desktop integration boundary. No files in the CU source repository were changed.

## Confirmed source characteristics
- The application is built with React, TypeScript, Vite, and TanStack Start.
- The default web build is SSR + PWA and uses the Node server output outside the hosted build sandbox.
- A dedicated `BUILD_TARGET=mobile` build already exists. It enables TanStack SPA mode, disables Nitro, and emits a static `index.html` plus hashed client assets for Capacitor.
- The mobile build is explicitly coupled to Capacitor and its native runtime conventions. The Capacitor configuration uses `dist/client` as its web directory.
- The source has a central Capacitor runtime helper that currently recognizes Capacitor Android/iOS and the configured native-shell hostnames. It does not yet define a Tauri/Windows desktop runtime.
- The app root contains mobile-specific startup logic and PWA launch handling. Desktop needs an explicit runtime identity rather than being misclassified as mobile or ordinary browser/PWA.
- The current CU package has no Tauri dependencies or Tauri configuration.

## Integration decision
**Do not reuse the current mobile build as the Windows desktop build.**

Create a separate desktop build target in the CU source repository only after an explicit, reviewed change is approved there. The target should:
1. Emit a static SPA bundle suitable for a locally bundled WebView.
2. Use a dedicated desktop build marker, separate from `VITE_SYM_MOBILE_BUILD`.
3. Preserve the existing default web SSR/PWA build unchanged.
4. Preserve Android/iOS build behavior unchanged.
5. Introduce a narrowly scoped desktop runtime detector and route only desktop-specific behavior through it.
6. Keep Capacitor-only plugins and bridges guarded so they are never invoked as if Windows were Android or iOS.

## Runtime boundary
The Tauri shell should identify itself through a documented, explicit desktop runtime signal. Do not rely only on a user-agent substring. The shared CU runtime helper should distinguish at least:
- Web browser / PWA
- Capacitor Android
- Capacitor iOS
- Tauri Windows desktop

Existing `isNativeAppRuntime()` semantics must not be broadened casually: several current flows intentionally mean Capacitor Android/iOS, not any embedded WebView. Add a separate desktop predicate and update only call sites whose behavior is verified for Windows.

## WebView and service constraints to validate
- Local bundled asset routing and SPA fallback for Tanstack Router.
- Supabase Auth persistence, OAuth/callback handling, and custom URL/deep links.
- Supabase Realtime lifecycle when the main window is hidden or minimized.
- Notification delivery and click-through routing.
- LiveKit microphone, camera, speaker selection, and call lifecycle.
- File chooser, downloads, and drag/drop permissions.
- External links and secure navigation boundaries.
- CSP, storage origin, and migration behavior between desktop versions.

## Private source delivery
The CU source repository is private while this repository is public. Never copy the CU source into this repository or publish it as a public submodule, artifact, cache, or workflow log.

For local development, the developer may clone the private source repository separately and point the desktop shell at a local built asset directory. For CI, use a protected, narrowly scoped credential or an approved private artifact handoff. The workflow must pin an exact source commit and must never expose the token or source in public artifacts.

## Go / no-go
- **Go:** proceed with a minimal Tauri shell and a separate desktop build integration spike.
- **No-go:** do not yet claim full CU compatibility, ship a Windows installer, or change the production CU build.

## Source inspection note
Findings are based on the current main-branch source layout and build configuration inspected during this planning pass. Revalidate against the exact source commit pinned for the first compatibility build.