struct Uniforms {
    width : u32,
    height : u32,
}
struct Sphere {
    center : vec3f,
    radius : f32,
}
fn intersect_sphere(ray : Ray, sphere : Sphere) -> f32 {
    let v = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let b = dot(v, ray.direction);
    let c = dot(v, v) - sphere.radius * sphere.radius;

    let d = b * b - a * c;
    if d < 0. {
        return - 1.;
    }

    let sqrt_d = sqrt(d);
    let recip_a = 1. / a;
    let mb = -b;
    let t = (mb - sqrt_d) * recip_a;
    if t > 0. {
        return t;
    }
    return (mb + sqrt_d) * recip_a;
}

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
alias TriangleVertices = array<vec2f, 6>;
var<private> vertices : TriangleVertices = TriangleVertices(
vec2f(-1.0, 1.0),
vec2f(-1.0, -1.0),
vec2f(1.0, 1.0),
vec2f(1.0, 1.0),
vec2f(-1.0, -1.0),
vec2f(1.0, -1.0),
);

struct Ray {
    origin : vec3f,
    direction : vec3f,
}

@vertex fn display_vs(@builtin(vertex_index) vid : u32) -> @builtin(position) vec4f {
    return vec4f(vertices[vid], 0.0, 1.0);
}

fn sky_color(ray : Ray) -> vec3f {
    let t = 0.5 * (normalize(ray.direction).y + 1.0);
    return (1. - t) * vec3(1.) + t * vec3(0.3, 0.5, 1.0);
}
@fragment fn display_fs(@builtin(position) pos : vec4f) -> @location(0) vec4f {
    let aspect_ratio = f32(uniforms.width) / f32(uniforms.height);
    var uv = pos.xy / vec2f(f32(uniforms.width - 1u), f32(uniforms.height - 1u));
    uv = (uv / 2 - vec2(0.5)) * vec2(aspect_ratio, -1.);
    let origin = vec3(0.);
    let focus_distance = 1.;
    let direction = vec3(uv, -focus_distance);
    let ray = Ray(origin, direction);
    let sphere = Sphere(vec3(0., 0., -1), 0.2);
    if intersect_sphere(ray, sphere) > 0. {
        return vec4(1., 0.76, 0.03, 1.);
    }
    return vec4(sky_color(ray), 1.);
}
