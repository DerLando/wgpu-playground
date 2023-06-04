fn circle(center: vec2<f32>, sample: vec2<f32>, radius: f32) -> f32 {
    // If the sample point is inside the interior sign needs to be positive
    return radius - length(sample - center);
}
