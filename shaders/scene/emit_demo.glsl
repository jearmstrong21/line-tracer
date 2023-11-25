#include geom/mod

float sampleWavelength(RAND_PARAM, float t) {
    return RAND_FLOAT * 300.0 + 400.0;
//    return t * 300.0 + 400.0;
}

uniform vec2 light_origin;
uniform float light_angle;

EdgeSample sampleLightsSpaced(RAND_PARAM, float t) {
    vec2 origin = light_origin;
    vec2 target = origin + vec2(cos(light_angle), sin(light_angle));
    vec2 direction=normalize(target-origin);
    vec2 left=vec2(-direction.y,direction.x);
    return EdgeSample(origin + (t-0.5)*left * 5.0, direction);
}

uniform float eta_a;
uniform float eta_b;

float getEta(float wl) {
    wl /= 700.0;
    return eta_a + eta_b / (wl * wl);
}