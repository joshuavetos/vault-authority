# BUILD_LOCK: Vault Authority v1.2.0

## Verified Toolchain
- **Node.js**: 20.x (LTS)
- **Rust**: 1.75.0
- **Tauri CLI**: 1.5.8
- **Platform**: Cross-platform (Windows/Mac/Linux)

## Invariant Check
This UI is a Read-Only Observer. The build process is configured to strip any auto-generated IPC handlers that are not explicitly defined. No communication from UI -> Rust is permitted in the tauri.conf.json allowlist.
