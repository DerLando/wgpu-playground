// Default output of verstex shader, do not change!
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.coord, 0.0, 1.0);
}
