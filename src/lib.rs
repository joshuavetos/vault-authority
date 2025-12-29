// Expose modules for use in main.rs and tests/
pub mod remediation;

/// Global Application State for Hot-Reloading (INV-2: Atomic)
pub struct AppState {
    pub signing_key: Vec<u8>,
}
