struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    switch in_vertex_index {
        case 0u, 5u: {
        out.coord = vec2<f32>(-1.0, -1.0);
                    }
        case 1u: {
        out.coord = vec2<f32>(1.0, -1.0);
                }
        case 2u, 3u: {
            out.coord = vec2<f32>(1.0, 1.0);
        }
        case 4u: {
            out.coord = vec2<f32>(-1.0, 1.0);
        }
        default: {
            out.coord = vec2<f32>(0.0, 0.0);
        }
    }
    
    out.position = vec4<f32>(out.coord, 0.0, 1.0);
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.coord, 0.0, 1.0);
}