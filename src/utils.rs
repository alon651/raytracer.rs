const EPSILON: f32 = 0.0001;
pub fn cmp_f32(x: f32, y: f32) -> bool {
    (x - y).abs() < EPSILON
}
