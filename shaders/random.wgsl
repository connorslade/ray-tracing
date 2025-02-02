var<private> seed: u32 = 0u;

fn rand() -> f32 {
    seed = seed * 747796405u + 2891336453u;
    let f = f32(seed >> 9u) / f32(1u << 23u);
    return fract(f);
}

fn rand_unit_vector() -> vec3f {
    return normalize(vec3(
        rand() * 2.0 - 1.0,
        rand() * 2.0 - 1.0,
        rand() * 2.0 - 1.0
    ));
}
