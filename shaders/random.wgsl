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

fn rand_hemisphere_vector(normal: vec3f) -> vec3f {
    let unit = rand_unit_vector();
    if dot(unit, normal) > 0.0 { return unit; }
    else { return -unit; }
}
