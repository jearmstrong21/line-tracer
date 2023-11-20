#include geom/ray

struct Line {
    vec2 a;
    vec2 b;
    int m;
};

#define MAX_LINE_COUNT 20
uniform int line_count;

uniform vec2 line_a[MAX_LINE_COUNT];
uniform vec2 line_b[MAX_LINE_COUNT];
uniform int line_m[MAX_LINE_COUNT];

EdgeSample sampleLine(RAND_PARAM, Line l) {
    vec2 d = l.b - l.a;
    return EdgeSample(l.a + d * RAND_FLOAT, normalize(vec2(-d.y, d.x)) * RAND_SIGN);
}

float hitLine_helper(float a, float b, float c, float d) {
    return a * d - b * c;
}

#define H_LINE_EPS 1.0
void hitLine(Ray r, Line l, inout HitRes hr) {
    float x1 = r.o.x;
    float y1 = r.o.y;
    float x2 = r.o.x + r.d.x;
    float y2 = r.o.y + r.d.y;

    float x3 = l.a.x;
    float y3 = l.a.y;
    float x4 = l.b.x;
    float y4 = l.b.y;

    float d = hitLine_helper(x1 - x2, x3 - x4, y1 - y2, y3 - y4);
    if (abs(d) < 1E-5) return;
    float t = hitLine_helper(x1 - x3, x3 - x4, y1 - y3, y3 - y4) / d;
    float u = hitLine_helper(x1 - x3, x1 - x2, y1 - y3, y1 - y2) / d;

    if (t > H_LINE_EPS && t < hr.t && 0.0 <= u && u <= 1.0) {
        hr.t = t;
        hr.r = r;
        hr.p = r.o + t * r.d;
        hr.n = normalize(vec2(l.a.y - l.b.y, l.b.x - l.a.x));
        hr.m = l.m;
        //            hr.m = l.m;
        //            hr.inside = false;
    }
}

void hitLines(Ray r, inout HitRes hr) {
    for(int i = 0; i < line_count; i++) {
        Line l;
        l.a = line_a[i];
        l.b = line_b[i];
        l.m = line_m[i];
        hitLine(r, l, hr);
    }
}