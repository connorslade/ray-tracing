fn ray_direction(pos: vec2f) -> vec3f {
    let forward = camera_direction();
    let right = normalize(cross(vec3f(0, 1, 0), forward));
    let up = normalize(cross(forward, right));

    let fov_scale = tan(ctx.camera.fov * 0.5);
    let uv = pos * vec2(ctx.camera.aspect, 1.0) * fov_scale;

    return normalize(forward + right * uv.x + up * uv.y);
}

fn get_scattered_direction_metal(ray: Ray, normal: vec3f, material: MetalMaterial) -> ScatterResult {
    let is_specular = f32(rand() < material.specular_probability);
    let smoothness = 1.0 - material.roughness;

    let diffuse = rand_hemisphere_vector(normal);
    let specular = reflect(ray.dir, normal);

    return ScatterResult(
        mix(diffuse, specular, smoothness * is_specular),
        mix(material.diffuse_color, material.specular_color, is_specular)
    );
}

fn get_scattered_direction_dielectric(ray: Ray, trace: Intersection, material: DielectricMaterial) -> vec3f {
    let normal = faceForward(trace.normal, trace.normal, ray.dir);

    var refractive_index = material.refractive_index;
    if trace.front_face { refractive_index = 1.0 / material.refractive_index; }

    let cos_theta = min(dot(-ray.dir, normal), 1.0);
    let sin_theta = sqrt(1.0 - cos_theta * cos_theta);

    let must_reflect = refractive_index * sin_theta > 1.0;
    let reflect_prob = schlick_approximation(cos_theta, refractive_index);
    let reflect = must_reflect || reflect_prob > rand();

    if reflect { return reflect(ray.dir, normal); }
    else { return refract(ray.dir, normal, refractive_index); }
}

fn camera_direction() -> vec3f {
    var pitch = ctx.camera.pitch;
    var yaw = ctx.camera.yaw;

    return normalize(vec3(
        cos(yaw) * cos(pitch),
        sin(pitch),
        sin(yaw) * cos(pitch)
    ));
}
