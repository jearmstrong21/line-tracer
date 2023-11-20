#include geom/ray

struct Circle {
    vec2 p;
    float r;
    int m;
};

#define MAX_CIRCLE_COUNT 20
uniform int circle_count;

uniform vec2 circle_p[MAX_CIRCLE_COUNT];
uniform float circle_r[MAX_CIRCLE_COUNT];
uniform int circle_m[MAX_CIRCLE_COUNT];

EdgeSample sampleCircle(RAND_PARAM, Circle c) {
    float theta = RAND_FLOAT * TWO_PI;
    vec2 d = vec2(cos(theta), sin(theta));
    return EdgeSample(c.p + c.r * d, d);
}

#define H_CIRCLE_EPS 1.0
void hitCircle(Ray r, Circle s, inout HitRes hr) {
    vec2 L = r.o - s.p;
    float b = 2.0 * dot(r.d, L);
    float c = dot(L, L) - s.r * s.r;
    float disc = b * b - 4.0 * c;
    if (disc >= 0.0) {
        float e = sqrt(disc);
        float t = (-b - e) / 2.0;
        if (t > H_CIRCLE_EPS && t < hr.t) {
            hr.t = t;
            hr.r = r;
            hr.p = r.o + t * r.d;
            hr.n = (L + t * r.d) / s.r;
            hr.m = s.m;
            //            hr.m = s.m;
            //            hr.inside = dot(hr.p - s.p, hr.p - s.p) < s.r * s.r;
        }
        t = (-b + e) / 2.0;
        if (t > H_CIRCLE_EPS && t < hr.t) {
            hr.t = t;
            hr.r = r;
            hr.p = r.o + t * r.d;
            hr.n = (L + t * r.d) / s.r;
            hr.m = s.m;
            //            hr.m = s.m;
            //            hr.inside = dot(hr.p - s.p, hr.p - s.p) < s.r * s.r;
        }
    }
}

void hitCircles(Ray r, inout HitRes hr) {
    for(int i = 0; i < circle_count; i++) {
        Circle c;
        c.p = circle_p[i];
        c.r = circle_r[i];
        c.m = circle_m[i];
        hitCircle(r, c, hr);
    }
}