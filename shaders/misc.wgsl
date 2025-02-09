fn background_color(ray_dir: vec3f) -> vec3f {
    let a = 0.5 * (ray_dir.y + 1.0);
    return (1.0 - a) * vec3(1.0, 1.0, 1.0) + a * vec3(0.5, 0.7, 1.0);
}

fn sample_rgb(texture: u32, uv: vec2f) -> vec3f {
    return textureSampleLevel(textures[texture], texture_sampler, uv, 0.0).xyz;
}

fn tangent_space(normal: vec3<f32>, sample: vec3<f32>) -> vec3<f32> {
    var arbitrary = vec3f(1.0, 0.0, 0.0);
    if abs(normal.x) > 0.9 { arbitrary = vec3f(0.0, 1.0, 0.0); }

    let tangent = normalize(cross(arbitrary, normal));
    let bitangent = cross(normal, tangent);
    return sample.x * tangent + sample.y * bitangent + sample.z * normal;
}
