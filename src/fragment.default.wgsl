// Default output of verstex shader, do not change!
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> time: u32; // Ellapsed time in ms

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.coord, sin(time), 1.0);
}
