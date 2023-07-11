const EPSILON: f32 = 0.0001;
pub fn cmp_f32(x: f32, y: f32) -> bool {
    (x - y).abs() < EPSILON
}

use std::sync::atomic::{AtomicUsize, Ordering};

static Ids: AtomicUsize = AtomicUsize::new(0);

pub fn generateId() -> usize {
    Ids.fetch_add(1, Ordering::SeqCst)
}