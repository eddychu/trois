pub const K_EPSILON: f32 = 0.00001;

pub fn is_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < K_EPSILON
}
pub fn srgb_to_linear(c : f32) -> f32 {
    c.powf(2.2)
}

pub fn linear_to_srgb(c : f32) -> f32 {
    c.powf(1.0 / 2.2)
}
