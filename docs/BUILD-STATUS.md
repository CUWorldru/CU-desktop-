# Build and Validation Status

## Implemented in the repository
- Vite + TypeScript frontend scaffold.
- Tauri 2 + Rust application scaffold.
- Windows-oriented main window configuration.
- Rust system tray with Open and Quit actions.
- Closing the main window hides it instead of terminating the process.
- Tauri notification plugin registration and a frontend permission/send smoke-test control.
- Minimal Tauri capability file granting core defaults and notification defaults.

## Not yet verified
- A Windows GitHub Actions check workflow has been added. No workflow run has been observed yet, so compilation is not confirmed.\n- No Windows build has been run in this environment.
- No `package-lock.json` or `Cargo.lock` has been generated yet; the initial CI currently uses `npm install` and is not yet lockfile-reproducible.
- The current Vite page is a diagnostic placeholder, not the CU application.
- The notification smoke test has not yet been run on Windows. The Tauri notification plugin documents that Windows notifications work for installed apps; development mode is not a reliable end-to-end validation target.
- Tray behavior, process lifecycle, single-instance behavior, startup, media, and calls require Windows testing.
- Installer bundling is intentionally disabled until the approved CU branding/icon assets and release-signing plan are ready.

## Local prerequisites
- Windows 10/11 x64 for the first target.
- Node.js 22 or newer and npm.
- Rust stable toolchain (the current notification plugin requires Rust 1.90 or newer).
- Microsoft WebView2 Runtime / Evergreen Runtime.

## Local shell commands
```powershell
npm install
npm run tauri -- dev
```

To check the frontend independently:
```powershell
npm run build
```

Do not treat a successful frontend-only build as proof that the Rust shell or Windows integrations work. Record the exact toolchain versions and test results when the first Windows build is run.