@group(0) @binding(0) var<uniform> ctx: Uniform;
@group(0) @binding(1) var<storage, read_write> accumulation: array<vec3f>;

// Vertex Shader //

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(1) uv: vec2<f32>
};

@vertex
fn vert(
    @location(0) pos: vec4<f32>,
    @location(1) uv: vec2<f32>,
) -> VertexOutput {
    return VertexOutput(pos, uv);
}

// Fragment Shader //

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel = vec2u(vec2f(in.uv.x, 1.0 - in.uv.y) * vec2f(ctx.window));
    let pixel_idx = pixel.y * ctx.window.x + pixel.x;

    return vec4(accumulation[pixel_idx], 1.0);
}
