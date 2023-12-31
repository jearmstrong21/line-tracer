struct Arc {
    vec2 p;
    float r;
    float ta;
    float tb;
    int m;
};

#define MAX_ARC_COUNT 20
uniform int arc_count;

uniform vec2 arc_p[MAX_ARC_COUNT];
uniform float arc_r[MAX_ARC_COUNT];
uniform float arc_ta[MAX_ARC_COUNT];
uniform float arc_tb[MAX_ARC_COUNT];
uniform int arc_m[MAX_ARC_COUNT];

float hitArc_helper(vec2 p, Arc s) {
    return atan(p.y - s.p.y, p.x - s.p.x) * 180.0 / PI;
}

#define H_ARC_EPS 1.0
void hitArc(Ray r, Arc s, inout HitRes hr) {
    vec2 L = r.o - s.p;
    float b = 2.0 * dot(r.d, L);
    float c = dot(L, L) - s.r * s.r;
    float disc = b * b - 4.0 * c;
    if (disc >= 0.0) {
        float e = sqrt(disc);
        float t = (-b - e) / 2.0;
        float theta = hitArc_helper(r.o + t * r.d, s);
        bool goodTheta;
        if (s.ta <= s.tb) {
            goodTheta = s.ta <= theta && theta <= s.tb;
        } else {
            goodTheta = s.ta <= theta || theta <= s.tb;
        }
        if (t > H_CIRCLE_EPS && t < hr.t && goodTheta) {
            hr.t = t;
            hr.r = r;
            hr.p = r.o + t * r.d;
            hr.n = (L + t * r.d) / s.r;
            hr.m = s.m;
            return;
        }
        t = (-b + e) / 2.0;
        theta = hitArc_helper(r.o + t * r.d, s);
        if (s.ta <= s.tb) {
            goodTheta = s.ta <= theta && theta <= s.tb;
        } else {
            goodTheta = s.ta <= theta || theta <= s.tb;
        }
        if (t > H_CIRCLE_EPS && t < hr.t && goodTheta) {
            hr.t = t;
            hr.r = r;
            hr.p = r.o + t * r.d;
            hr.n = (L + t * r.d) / s.r;
            hr.m = s.m;
        }
    }
}

void hitArcs(Ray r, inout HitRes hr) {
    for(int i = 0; i < arc_count; i++) {
        Arc a;
        a.p = arc_p[i];
        a.r = arc_r[i];
        a.ta = arc_ta[i];
        a.tb = arc_tb[i];
        a.m = arc_m[i];
        hitArc(r, a, hr);
    }
}