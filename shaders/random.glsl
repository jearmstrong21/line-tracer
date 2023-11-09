#define RNGL_HIGH_QUALITY
struct Random { uint s0; uint s1; };

float unorm(uint n) { return float(n) * (1.0 / float(0xffffffffU)); }
uint uhash(uint a, uint b) {
    uint x = ((a * 1597334673U) ^ (b * 3812015801U));
    #ifdef RNGL_HIGH_QUALITY
    x = x ^ (x >> 16u);
    x = x * 0x7feb352du;
    x = x ^ (x >> 15u);
    x = x * 0x846ca68bu;
    x = x ^ (x >> 16u);
    #else
    x = x * 0x7feb352du;
    x = x ^ (x >> 15u);
    x = x * 0x846ca68bu;
    #endif
    return x;
}
Random seed(uint s) { return Random(s, uhash(0x1ef7c663u, s)); }
Random seed(uvec3 v) { return seed(uhash(v.x, uhash(v.y, v.z))); }
uint urandom(inout Random rng) {
    uint last = rng.s1;
    uint next = uhash(rng.s0, rng.s1);
    rng.s0 = rng.s1; rng.s1 = next;
    return last;
}
float random__(inout Random rng) { return unorm(urandom(rng)); }
vec2 random_hemi__(inout Random rng, vec2 v) {
    float theta = atan(v.y, v.x) + (random__(rng) - 0.5) * PI;
    return vec2(cos(theta), sin(theta));
}

#define RAND_PARAM inout Random rng
#define RAND_FLOAT (random__(rng))
#define RAND_SIGN (random__(rng) < 0.5 ? 1.0 : -1.0)
#define RAND_VEC2 (vec2(RAND_FLOAT, RAND_FLOAT))
#define RAND_VEC3 (vec3(RAND_FLOAT, RAND_FLOAT, RAND_FLOAT))
#define RAND_HEMI(v) (random_hemi__(rng, (v)))