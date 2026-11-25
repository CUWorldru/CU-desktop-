# CU Desktop — Releases

## Initial target
- Platform: Windows 10/11, x64 first.
- Packaging: installer format to be selected after the Tauri compatibility spike.
- ARM64: evaluate after x64 is stable.

## Release requirements
- Build from a reviewed, pinned source revision.
- Run automated checks and Windows smoke tests.
- Sign the application and updater metadata.
- Verify update signatures and installation/rollback behavior.
- Publish release notes and checksums.

## Secrets
Signing certificates, private keys, updater credentials, and private-source access tokens must be stored as protected secrets outside this public repository. Never place secret values in workflow YAML or logs.

## Release status
No production release pipeline is enabled yet. Do not represent an unsigned development build as a production release.