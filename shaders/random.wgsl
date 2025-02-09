var<private> seed: u32 = 0u;

fn rand() -> f32 {
    seed = seed * 747796405u + 2891336453u;
    let f = f32(seed >> 9u) / f32(1u << 23u);
    return fract(f);
}

fn rand_unit_vector() -> vec3f {
    var z = rand() * 2.0 - 1.0;
    var a = rand() * 2.0 * PI;
    var r = sqrt(1.0 - z * z);
    var x = r * cos(a);
    var y = r * sin(a);
    return vec3(x, y, z);
}

fn rand_cosine_hemisphere_vector(normal: vec3<f32>) -> vec3<f32> {
    let r = sqrt(rand());
    let theta = 2.0 * PI * rand();

    let sample = vec3f(
        r * cos(theta),
        r * sin(theta),
        sqrt(1.0 - r * r)
    );

    return tangent_space(normal, sample);
}
