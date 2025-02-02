var<private> seed: u32 = 0u;

fn rand() -> f32 {
    seed = seed * 747796405u + 2891336453u;
    let f = f32(seed >> 9u) / f32(1u << 23u);
    return fract(f);
}

fn rand_normal() -> f32 {
    let theta = 2.0 * PI * rand();
    let rho = sqrt(-2.0 * log(rand()));
    return rho * cos(theta);
}

fn rand_unit_vector() -> vec3f {
    return vec3(
        rand_normal(),
        rand_normal(),
        rand_normal()
    );
}

fn rand_hemisphere_vector(normal: vec3f) -> vec3f {
    let unit = rand_unit_vector();
    return unit * sign(dot(unit, normal));
}
