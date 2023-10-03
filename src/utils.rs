
use std::sync::atomic::{AtomicUsize, Ordering};

const EPSILON: f32 = 0.0001;
pub fn cmp_f32(x: f32, y: f32) -> bool {
    (x - y).abs() < EPSILON
}

static IDS: AtomicUsize = AtomicUsize::new(0);

pub fn generate_id() -> usize {
    IDS.fetch_add(1, Ordering::SeqCst)
}

