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
- Use the dedicated desktop build target from the private CU source review branch; do not use the Android/iOS Capacitor build target.

## Current integration gate
The private CU source desktop build is available on draft PR [#145](https://github.com/CUWorldru/journey-structure-import-82fde408/pull/145), branch `feat/cu-desktop-build-target`. It is not merged into private `main` yet, and its private Actions validation has not completed successfully. Keep the Tauri shell on its diagnostic placeholder until that source change is reviewed and validated.

## Planned local flow
1. Clone the private CU source into a sibling directory using the owner's normal GitHub access.
2. Checkout the exact desktop source commit approved for the compatibility spike.
3. Build the private source with `npm run build:desktop` after the desktop target is approved.
4. Point the Tauri shell at the generated local static assets without copying them into this public repository.
5. Run Windows-specific auth, Realtime, notification, file, and call tests.
6. Keep the private checkout and generated bundle out of this public repository and its public artifacts.

## CI flow
CI may access the private source only through a protected, least-privilege credential or an explicitly approved private artifact delivery mechanism. Pin the source commit, redact logs, and ensure public artifacts contain only the distributable desktop app.
