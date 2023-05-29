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

fn sd_circle(position: vec2<f32>, radius: f32) -> f32 {
    return length(position) - radius;
}
 
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let out_color = vec3<f32>(0.9, 0.6, 0.3);
    let in_color = vec3<f32>(0.65, 0.85, 1.0);
    
    //var sign = sd_triangle_equi(in.vert_pos);
    var sign = sd_circle(in.coord, 0.5);
    var color = vec3<f32>(1.0);
    if sign > 0.0 {
        color = out_color;
    }
    else{
        color = in_color;
    }
    
        // Iso curves
    color *= 1.0 - exp(-6.0*abs(sign));
    color *= 0.8 + 0.2*cos(150.0 * sign);
    color = mix(color, vec3<f32>(1.0), 1.0 - smoothstep(0.0, 0.01, abs(sign)));
        
    return vec4<f32>(color, 1.0);
}