struct Ray {
    vec2 o;
    vec2 d;
};

#define M_DIFF 1
#define M_REFL 2
#define M_REFR 3

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