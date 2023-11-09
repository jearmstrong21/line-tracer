#version 330

in vec2 pos;

out vec4 fc;

uniform vec2 screenSize;
uniform int frameCount;
uniform float systemTime;

#define PI 3.1415
#define TWO_PI (2.0 * PI)

#include random
#include zucconi6
#include scene

void main() {
    Random rng = seed(uvec3(pos * screenSize, frameCount));
    int N = 5;
    int DEPTH = 3;
    vec3 C = vec3(0);
    for (int i = 0; i < N; i++) {
//        vec2 x = pos * screenSize;
//        EdgeSample l = sampleLights(rng);
//        Ray r = Ray(x, normalize(vec2(l.p - x)));
//        HitRes hr = hitScene(r);
//        if (hr.hit && hr.p.x > 500.0) {
//            C += vec3(100.0 / hr.t);
//        }

        float wl = RAND_FLOAT * 300.0 + 400.0;
//        vec3 c = vec3(10.0);
        vec3 c = 10.0 * spectral_zucconi6(wl);
        float totalT = 0.0;

//        float theta = (i + RAND_FLOAT) / N * TWO_PI;
        float theta = RAND_FLOAT * TWO_PI;
        Ray r = Ray(pos * screenSize, vec2(cos(theta), sin(theta)));

        for (int j = 0; j < DEPTH; j++) {
            HitRes hr = hitScene(r);
            if (hr.hit) {
                totalT += hr.t;
                if (hr.p.x > 500.0 && hr.p.x < screenSize.x - 10.0 && hr.p.y > 10.0 && hr.p.y < screenSize.y - 10.0) {
                    C += c;
                    break;
                } else {
                    float eta = 1.0 / 1.5 * 700.0 / wl;
                    if (j == 1) {
                        eta = 1.0 / eta;
                    }
                    vec2 n = refract(r.d, hr.n, eta);
                    r = Ray(hr.p, n);
//                    r = Ray(hr.p, RAND_HEMI(hr.n));
//                    r = Ray(hr.p, reflect(r.d, hr.n));
                }
            } else {
                break;
            }
        }
    }
    fc = vec4(C / N, 1.0);
}