# Local Development — Private CU Source

## Repository layout
Keep the private CU source checkout separate from this public repository. A recommended local workspace is:
```text
workspace/
  CU-desktop-/       # this repository
  cu-source/         # private CU source checkout; never copied or pushed here
```

## Rules
- Never add the private source checkout as a folder, submodule, subtree, or copied snapshot inside this public repository.
- Never commit `.env`, local configuration, credentials, generated private assets, or source bundles.
- Keep local build output in ignored directories such as `.cu-build/`.
- Pin and record the exact private source commit used for every compatibility test.
- The desktop build must use a dedicated desktop target in the CU source, not the Android/iOS Capacitor build target.

## Current implementation gate
The CU source currently has a web SSR/PWA build and a Capacitor SPA build. A dedicated Windows desktop build marker/runtime seam has not yet been implemented. Until that reviewed source change exists, do not claim that the full CU frontend is ready to load in Tauri.

## Planned local flow
1. Clone the private CU source into a sibling directory using the owner's normal GitHub access.
2. Checkout the exact source commit selected for the desktop compatibility spike.
3. After the dedicated desktop build target is approved and implemented, build the static desktop assets locally.
4. Point the Tauri shell at the local generated asset directory.
5. Run Windows-specific auth, Realtime, notification, file, and call tests.
6. Keep the private checkout and generated bundle out of this public repository and its public artifacts.

## CI flow
CI may access the private source only through a protected, least-privilege credential or an explicitly approved private artifact delivery mechanism. Pin the source commit, redact logs, and ensure public artifacts contain only the distributable desktop app.