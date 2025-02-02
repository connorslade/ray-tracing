fn hit_sphere(sphere: Sphere, ray: Ray) -> Hit {
    let oc = sphere.position - ray.pos;
    let h = dot(ray.dir, oc);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = h * h - c;

    if discriminant < 0 { return default_hit(); }

    let t = h - sqrt(discriminant);
    let position = ray.pos + ray.dir * t;
    return Hit(
        position,
        normalize(position - sphere.position),
        t
    );
}

fn hit_triangle(triangle: Triangle, ray: Ray) -> Hit {
    let edge_ab = triangle.v1 - triangle.v0;
    let edge_ac = triangle.v2 - triangle.v0;
    let normal = cross(edge_ab, edge_ac);
    let ao = ray.pos - triangle.v0;
    let dao = cross(ao, ray.dir);

    let det = -dot(ray.dir, normal);
    let inv_det = 1.0 / det;

    let dst = dot(ao, normal) * inv_det;
    let u = dot(edge_ac, dao) * inv_det;
    let v = -dot(edge_ab, dao) * inv_det;
    let w = 1.0 - u - v;

    let hit_pos = ray.pos + ray.dir * dst;
    let hit_normal = normalize(triangle.n0 * w + triangle.n1 * u + triangle.n2 * v);
    return Hit(hit_pos, hit_normal, dst);
}

fn ray_direction(pos: vec2f) -> vec3f {
    let forward = camera_direction();
    let right = normalize(cross(vec3f(0, 1, 0), forward));
    let up = normalize(cross(forward, right));

    let fov_scale = tan(ctx.camera.fov * 0.5);
    let uv = pos * vec2(ctx.camera.aspect, 1.0) * fov_scale;

    return normalize(forward + right * uv.x + up * uv.y);
}

fn get_scattered_direction(ray: Ray, trace: TraceResult) -> vec3f {
    let specular_dir = reflect(ray.dir, trace.hit.normal);
    let diffuse_dir = rand_hemisphere_vector(trace.hit.normal);
    return mix(specular_dir, diffuse_dir, trace.material.roughness);
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
