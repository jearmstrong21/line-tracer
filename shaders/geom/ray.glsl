struct Ray {
    vec2 o;
    vec2 d;
};

struct HitRes {
    bool hit;
    float t;
    Ray r;
    vec2 p;
    vec2 n;
    int m;
};

struct EdgeSample {
    vec2 p;
    vec2 n;
};