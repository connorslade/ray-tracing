fn background_color(ray_dir: vec3f) -> vec3f {
    let a = 0.5 * (ray_dir.y + 1.0);
    return (1.0 - a) * vec3(1.0, 1.0, 1.0) + a * vec3(0.5, 0.7, 1.0);
}

fn sample_rgb(texture: u32, uv: vec2f) -> vec3f {
    return textureSampleLevel(textures[texture], texture_sampler, uv, 0.0).xyz;
}
