# Build and Validation Status

## Implemented in the repository
- Vite + TypeScript frontend scaffold.
- Tauri 2 + Rust application scaffold.
- Windows-oriented main window configuration.
- Rust system tray with Open and Quit actions.
- Closing the main window hides it instead of terminating the process.
- Tauri notification plugin registration and a frontend permission/send smoke-test control.
- Tauri HTTP plugin registration with an allowlist limited to the CU server-function endpoint.
- Minimal Tauri capability file granting core defaults and notification defaults.

## Confirmed validation
- GitHub Actions Windows x64 run **passed** on 2026-11-25 for commit `79bf9672d1b050ae124198ffd2c234aaef060454`.
- The successful run completed `npm ci`, TypeScript check + Vite production build, and Rust `cargo check --locked`.
- `package-lock.json` and `src-tauri/Cargo.lock` are committed, so dependency resolution is reproducible.
- [Locked Windows CI run](https://github.com/CUWorldru/CU-desktop-/actions/runs/45536345904)

This confirms that the current frontend scaffold and Rust shell compile on the Windows runner. It does not yet prove that the full CU application works inside the shell.

## Not yet verified
- The private CU source desktop build target is in draft PR [#145](https://github.com/CUWorldru/journey-structure-import-82fde408/pull/145); it has not been merged into the private source `main`.
- The Tauri shell still displays a diagnostic placeholder, not the CU application.
- The notification smoke test has not yet been run interactively on a Windows machine.
- Tray behavior, process lifecycle, single-instance behavior, startup, media, and calls require hands-on Windows testing.
- A temporary CU icon is present to satisfy Windows resource compilation; replace it with final approved branding before release.
- Installer bundling is intentionally disabled until the release-signing plan is ready.

## Local prerequisites
- Windows 10/11 x64 for the first target.
- Node.js 22 or newer and npm.
- Rust stable toolchain (the current notification plugin requires Rust 1.90 or newer).
- Microsoft WebView2 Runtime / Evergreen Runtime.

## Local shell commands
```powershell
npm ci
npm run tauri -- dev
```

To check the frontend independently:
```powershell
npm run build
```

Do not treat a successful frontend-only build or `cargo check` as proof that the complete CU application, Windows notifications, tray lifecycle, media, or calls work end-to-end.
