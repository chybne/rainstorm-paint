struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

struct Uniform {
    projection: mat4x4<f32>,
    transformation: mat4x4<f32>,
}

@group(1) @binding(0)
var<uniform> uni: Uniform;

@vertex
fn vs_main(
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = uni.projection * uni.transformation * vec4<f32>(model.position, 0.0, 1.0);
    return out;
}

/* fragment */

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // return vec4<f32>(0.2, 0.6, 1.0, 1.0);
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
