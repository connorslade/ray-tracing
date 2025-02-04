fn hit_sphere(sphere: Sphere, ray: Ray) -> Hit {
    let oc = sphere.position - ray.pos;
    let h = dot(ray.dir, oc);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = h * h - c;

    if discriminant < 0 { return default_hit(); }

    let t = h - sqrt(discriminant);
    let position = ray.pos + ray.dir * t;
    let normal = normalize(position - sphere.position);
    return Hit(position, normal, t);
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

    if det < 1e-6 || dst < 0.0 || u < 0.0 || v < 0.0 || w < 0.0 {
        return default_hit();
    }

    let hit_pos = ray.pos + ray.dir * dst;
    let hit_normal = normalize(triangle.n0 * w + triangle.n1 * u + triangle.n2 * v);
    return Hit(hit_pos, hit_normal, dst);
}

fn hit_bounding_box(bounds: BoundingBox, ray: Ray) -> f32 {
    let tmin = (bounds.min - ray.pos) * ray.inv_dir;
    let tmax = (bounds.max - ray.pos) * ray.inv_dir;

    let t1 = min(tmin, tmax);
    let t2 = max(tmin, tmax);

    let tnear = max(max(t1.x, t1.y), t1.z);
    let tfar = min(min(t2.x, t2.y), t2.z);

    if tfar >= tnear && tfar > 0.0 { return tnear; }
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

fn get_scattered_direction(ray: Ray, trace: TraceResult) -> ScatterResult {
    let is_specular = f32(rand() < trace.material.specular_probability);
    let smoothness = 1.0 - trace.material.roughness;

    let diffuse = rand_hemisphere_vector(trace.hit.normal);
    let specular = reflect(ray.dir, trace.hit.normal);

    return ScatterResult(
        mix(diffuse, specular, smoothness * is_specular),
        mix(trace.material.diffuse_color, trace.material.specular_color, is_specular)
    );
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
