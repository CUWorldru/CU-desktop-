# Build and Validation Status

## Implemented
- Vite + TypeScript diagnostic shell.
- Tauri 2 + Rust Windows application shell.
- Rust system tray with Open and Quit actions.
- Closing the main window hides it instead of terminating the process.
- Tauri notification plugin registration and a frontend permission/send smoke-test control.
- Tauri HTTP plugin with a capability allowlist limited to `https://seeyoumore.ru/_serverFn/**`.
- Dedicated private CU desktop SPA build target merged into private CU `main`.
- Public Windows workflow that checks out a pinned private CU commit, builds the CU desktop SPA, stages it into Tauri, and packages an NSIS installer.
- Pull-request checks are isolated from the private-source token; only trusted pushes to `main` and manual dispatch can access private source.

## Confirmed validation

- Desktop messenger workspace now gives the selected conversation the full remaining width; the conversation list has a keyboard-accessible drag splitter (220–520 px), and the context inspector is collapsed by default and can be toggled from the chat toolbar. Source change: private CU commit `48526ad53bb81e5b5077063fedcac8c1458565cd`.
- The Tauri shell previously passed Windows x64 frontend typecheck/build and Rust `cargo check --locked` in GitHub Actions.
- Private CU PR #145 was merged to private `main` at commit `2175e7d5327a4d45854d5d8a1b80c964ab537c54`.
- The private repository's Actions validation could not allocate a runner before merge; therefore the private CU desktop build has not yet been confirmed by CI.
- The first end-to-end Windows installer workflow is now configured but still requires the one-time repository secrets and a successful packaging run.

## Required one-time configuration
See [README](../README.md#one-time-github-configuration) for the exact secret names and minimum permissions.

## First build procedure
1. Add the required Actions secrets to the public `CU-desktop-` repository.
2. Open **Actions → CU Desktop — Windows CI**.
3. Run the workflow on `main`.
4. Download the `CU-Desktop-Windows-x64` artifact from the successful run.

## Not yet verified on a Windows machine
- First full CU installer build and install.
- Login, OAuth/deep links, Supabase Auth/Realtime, calls and media.
- Windows notification delivery, tray lifecycle, background behavior, and restart.
- Code signing and automatic updates.

## Important
The first installer is a test build. It is not signed and should not be treated as a production release. The compiled client is included in the installer; private source files are not committed to this public repository.
