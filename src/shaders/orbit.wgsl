// dithered earth with a capsule doing one orbit per minute

struct Uniforms {
    res: vec2f,
    time: f32,
    orbit: f32, // 0..1 through the current minute
    view_lon: f32,
    dark: f32,
    srgb: f32,
    _pad: f32,
};

@group(0) @binding(0) var<uniform> u: Uniforms;
// see scripts/gen_earth_mask.py
@group(0) @binding(1) var land_mask: texture_2d<f32>;
@group(0) @binding(2) var land_sampler: sampler;

const TAU: f32 = 6.2831853;
const PI: f32 = 3.1415927;
const R: f32 = 0.24;
const RX: f32 = 0.44;
const RY: f32 = 0.12;
const TILT: f32 = -0.3;
const VIEW_LAT: f32 = 0.35;

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4f {
    let p = vec2f(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4f(p * 2.0 - 1.0, 0.0, 1.0);
}

fn bayer2(p: vec2u) -> u32 {
    return ((p.x ^ p.y) & 1u) * 2u + (p.y & 1u);
}

fn bayer4(p: vec2u) -> f32 {
    let m = bayer2(p & vec2u(1u)) * 4u + bayer2((p >> vec2u(1u)) & vec2u(1u));
    return (f32(m) + 0.5) / 16.0;
}

fn hash(p: vec2u) -> f32 {
    var h = p.x * 1664525u + p.y * 1013904223u;
    h ^= h >> 16u;
    h *= 0x7feb352du;
    h ^= h >> 15u;
    h *= 0x846ca68bu;
    h ^= h >> 16u;
    return f32(h) / 4294967295.0;
}

fn from_orbit(q: vec2f) -> vec2f {
    let c = cos(TILT);
    let s = sin(TILT);
    return vec2f(c * q.x - s * q.y, s * q.x + c * q.y);
}

@fragment
fn fs_main(@builtin(position) frag: vec4f) -> @location(0) vec4f {
    let px = vec2u(frag.xy);
    let scale = min(u.res.x, u.res.y);
    let pix = 1.0 / scale;
    var p = (frag.xy - u.res * 0.5) / scale;
    p.y = -p.y;

    let threshold = bayer4(px);
    var ink = 0.0;
    var accent = 0.0;

    let r = length(p);
    let on_planet = r < R;

    // stars
    if (!on_planet) {
        let h = hash(px);
        if (h > 0.996) {
            ink = 0.5 + 0.5 * sin(u.time * 1.3 + h * 1000.0);
        }
    }

    let dark = u.dark > 0.5;

    // earth
    if (on_planet) {
        let n = vec3f(p / R, sqrt(max(0.0, 1.0 - r * r / (R * R))));
        let cl = cos(VIEW_LAT);
        let sl = sin(VIEW_LAT);
        let g = vec3f(n.x, n.y * cl + n.z * sl, -n.y * sl + n.z * cl);
        let lon = atan2(g.x, g.z) + u.view_lon;
        let lat = asin(clamp(g.y, -1.0, 1.0));

        let uv = vec2f(lon / TAU + 0.5, 0.5 - lat / PI);
        let land = smoothstep(0.3, 0.7, textureSampleLevel(land_mask, land_sampler, uv, 0.0).r);

        let bright = mix(0.28, 0.95, land);

        // invert on the light theme
        ink = select(1.0 - bright, bright, dark);

        if (r > R - 1.2 * pix) {
            ink = 1.0;
        }
    }

    // orbit
    let c = cos(TILT);
    let s = sin(TILT);
    let q = vec2f(c * p.x + s * p.y, -s * p.x + c * p.y);
    let e = q / vec2f(RX, RY);
    let el = max(length(e), 1e-4);
    let grad = max(length(e / vec2f(RX, RY)) / el, 1e-4);
    let d = abs(el - 1.0) / grad;
    let theta = atan2(e.y, e.x);

    let hidden = e.y > 0.0 && on_planet;

    // starts at the top, goes clockwise
    let cap_theta = PI * 0.5 - u.orbit * TAU;

    if (!hidden) {
        let on_line = d < 0.8 * pix;
        let dash = fract(theta * 64.0 / TAU) < 0.5;

        let behind_cap = fract((theta - cap_theta) / TAU) * TAU;
        let trail = 1.0 - behind_cap / 1.6;

        if (on_line && trail > 0.0) {
            accent = trail;
        } else if (on_line && dash) {
            ink = 0.9;
        }

        let tick = abs(fract((PI * 0.5 - theta) / TAU * 12.0 + 0.5) - 0.5);
        if (tick < 0.012 && d < 3.0 * pix) {
            ink = 1.0;
        }
    }

    // capsule
    let cap = from_orbit(vec2f(RX * cos(cap_theta), RY * sin(cap_theta)));
    let cap_hidden = sin(cap_theta) > 0.0 && on_planet;
    if (!cap_hidden) {
        let dc = abs(p - cap) / pix;
        let cheb = max(dc.x, dc.y);
        if (cheb < 1.6) {
            accent = 1.0;
        }
        if (cheb >= 5.0 && cheb < 6.0 && min(dc.x, dc.y) > 2.5) {
            accent = 1.0;
        }
    }

    let bg = select(vec3f(1.0), vec3f(0.051), dark);
    let fg = select(vec3f(0.102), vec3f(0.878), dark);
    let orange = vec3f(1.0, 0.42, 0.208);

    var col = bg;
    if (ink > threshold) {
        col = fg;
    }
    if (accent > threshold) {
        col = orange;
    }
    if (u.srgb > 0.5) {
        col = pow(col, vec3f(2.2));
    }
    return vec4f(col, 1.0);
}
