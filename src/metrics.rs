use once_cell::sync::Lazy;
use prometheus::{Registry, Counter, IntCounterVec, register_int_counter_vec_with_registry};

pub struct VaultMetrics {
    pub total_remediations: IntCounterVec,
    pub active_rejections: IntCounterVec,
}

pub static METRICS: Lazy<VaultMetrics> = Lazy::new(|| {
    let registry = Registry::new();
    VaultMetrics {
        total_remediations: register_int_counter_vec_with_registry!(
            "vault_remediations_total",
            "Total number of remediation attempts",
            &["failure_id", "outcome"],
            registry
        ).unwrap(),
        active_rejections: register_int_counter_vec_with_registry!(
            "vault_rejections_total",
            "Total number of invariant rejections (INV-1 through INV-4)",
            &["reason"],
            registry
        ).unwrap(),
    }
});
