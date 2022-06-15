const K_EPSILON: f64 = 0.00001;

fn is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < K_EPSILON
}
