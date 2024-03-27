#version 330 core

layout (std140) uniform;
layout (points) in;
layout (line_strip, max_vertices = 20) out;

uniform vec2 screen_size;
uniform int frame_count;
uniform float system_time;

uniform vec2 mouse_coords;

uniform float bounce_diminish;

uniform float brightness_scale;
uniform float ray_count;

#define PI 3.1415
#define TWO_PI (2.0 * PI)

#include random
#include geom/mod
//include scene/emit_caustic_focus
#include scene/emit_demo
#include zucconi6

out vec3 color;


#define H_SCENE_EPS 1E10
HitRes hitScene(Ray r) {
    HitRes hr;
    hr.t = H_SCENE_EPS;

    hitArcs(r, hr);
    hitCircles(r, hr);
    hitLines(r, hr);

    hr.hit = hr.t < H_SCENE_EPS;
    if (hr.hit) {
        hr.n = normalize(hr.n);
        if (dot(hr.n, r.d) > 0.0) {
            hr.n *= -1.0;
        }
    }
    return hr;
}

void point(vec2 p) {
    gl_Position = vec4((p / screen_size * 2.0 - 1.0), 0, 1);
    EmitVertex();
}

uniform float jitterGeom;

void main() {
    float spaced = gl_in[0].gl_Position.x + jitterGeom - 0.5;
    Random rng = seed(uvec3(spaced * screen_size.x, 0, 1 * frame_count));
    EdgeSample es = sampleLightsSpaced(rng, spaced);

    float wl = sampleWavelength(rng, spaced);

    color = spectral_zucconi6(wl) * 0.25 * brightness_scale * 10000.0 / ray_count;

    Ray r = Ray(es.p, es.n);
//    r.o += spaced * 100.0;
    point(r.o);
    bool inMedium = false;
    for (int i = 0; i < 40; i++) {
        HitRes hr = hitScene(r);
        Ray nr;
        if (hr.hit) {
            vec2 n;
            if (hr.m == M_DIFF) {
                n = RAND_HEMI(hr.n);
            } else if (hr.m == M_REFL) {
                n = reflect(r.d, hr.n);
            } else if (hr.m == M_REFR) {
                if (RAND_FLOAT < 0.) {
                    n = reflect(r.d, hr.n);
                } else {
                    float eta = getEta(wl);
                    //             TODO fix xray through prism + blending + arc-shaped reflector
                    if (inMedium) {
                        eta = 1.0 / eta;
                    }
                    n = refract(r.d, hr.n, eta);
                    if (length(n) < 0.1) {
                        n = reflect(r.d, hr.n);
                    }
                    inMedium = !inMedium;
                }
            } else {
                color = vec3(1, 0, 0);
            }
            vec2 hitpos = r.o + r.d * hr.t;
            r = Ray(hr.p, n);
            color *= bounce_diminish;
            point(hitpos);
        } else {
            point(r.o + r.d * dot(screen_size, screen_size));
            break;
        }
    }
    EndPrimitive();
}