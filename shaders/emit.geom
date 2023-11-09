#version 330 core

layout (points) in;
layout (line_strip, max_vertices = 20) out;

uniform vec2 screenSize;
uniform int frameCount;
uniform float systemTime;

uniform vec2 mouseCoords;

#define PI 3.1415
#define TWO_PI (2.0 * PI)

#include random
#include geom/mod
//include scene/emit_caustic_focus
#include scene/emit_demo
#include zucconi6

out vec3 color;

void point(vec2 p) {
    gl_Position = vec4(
    0.5 *
    (p / screenSize * 2.0 - 1.0), 0, 1);
    EmitVertex();
}

void main() {
    float spaced = gl_in[0].gl_Position.x;
    Random rng = seed(uvec3(spaced * screenSize.x, 0, 0 * frameCount));
    EdgeSample es = sampleLightsSpaced(rng, spaced);

    float wl = sampleWavelength(rng, spaced);

    color = spectral_zucconi6(wl) * 0.25;
//    color = vec3(1);

    Ray r = Ray(es.p
//    / 1.5
    , es.n);
    point(r.o);
    bool inMedium = false;
    for (int i = 0; i < 40; i++) {
        HitRes hr = hitScene(r);
        Ray nr;
        if (hr.hit) {
            point(r.o + r.d * hr.t);
//            EndPrimitive();
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
            r = Ray(hr.p, n);
//            color *= 0.5;
        } else {
//            point(r.o);
            point(r.o + r.d * dot(screenSize, screenSize));
//            EndPrimitive();
            break;
        }
    }
    EndPrimitive();
}