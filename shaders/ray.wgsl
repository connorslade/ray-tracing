fn hit_sphere(center: vec3f, radius: f32, ray_origin: vec3f, ray_dir: vec3f) -> f32 {
    let oc = center - ray_origin;
    let h = dot(ray_dir, oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = h * h - c;

    if discriminant < 0 { return -1.0; }
    return (h - sqrt(discriminant));
}

fn hit_triangle(v0: vec3f, v1: vec3f, v2: vec3f, ray_origin: vec3f, ray_dir: vec3f) -> f32 {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let pvec = cross(ray_dir, edge2);
    let det = dot(edge1, pvec);

    if abs(det) < 1e-6 { return -1.0; }

    let inv_det = 1.0 / det;
    let tvec = ray_origin - v0;

    let u = dot(tvec, pvec) * inv_det;
    if u < 0.0 || u > 1.0 { return -1.0; }

    let qvec = cross(tvec, edge1);
    let v = dot(ray_dir, qvec) * inv_det;
    if v < 0.0 || u + v > 1.0 { return -1.0; }

    let t = dot(edge2, qvec) * inv_det;
    if t >= 1e-6 { return t; }
    return -1.0;
}

fn ray_direction(pos: vec2f) -> vec3f {
    let forward = camera_direction();
    let right = normalize(cross(vec3f(0, 1, 0), forward));
    let up = normalize(cross(forward, right));

    let fov_scale = tan(ctx.camera.fov * 0.5);
    let uv = pos * vec2(ctx.camera.aspect, 1.0) * fov_scale;

    return normalize(forward + right * uv.x + up * uv.y);
}

fn get_scattered_direction(ray_dir: vec3f, hit: Hit) -> vec3f {
    let specular_dir = reflect(ray_dir, hit.normal);
    let diffuse_dir = rand_hemisphere_vector(hit.normal);
    return mix(specular_dir, diffuse_dir, hit.material.roughness);
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
