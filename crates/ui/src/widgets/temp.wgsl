struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

    var positions = array<vec2<f32>, 3>(
        vec2(-1.0, -3.0),
        vec2(3.0, 1.0),
        vec2(-1.0, 1.0)
    );

    let pos = positions[vertex_index];
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.2, 0.6, 1.0, 1.0);
}
