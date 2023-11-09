#version 330 core

in vec2 uv;

uniform sampler2D tex;
uniform vec2 screenSize;

out vec4 fc;

void main() {
    vec3 c = texelFetch(tex, ivec2(uv*screenSize),0).xyz;
    if (c.x > 1.0 || c.y > 1.0 || c.z > 1.0) {
        c /= max(c.x,max(c.y,c.z));
    }
    fc=vec4(c,1.0);
//    ivec2 p = ivec2(uv * screenSize);
//    vec3 xmi = texelFetch(tex, ivec2(p.x - 1, p.y), 0).xyz;
//    vec3 xpl = texelFetch(tex, ivec2(p.x + 1, p.y), 0).xyz;
//    vec3 ymi = texelFetch(tex, ivec2(p.x, p.y - 1), 0).xyz;
//    vec3 ypl = texelFetch(tex, ivec2(p.x, p.y + 1), 0).xyz;
//    vec3 c = texelFetch(tex, p, 0).xyz;
////    vec3 res = c;
//    vec3 res = (xmi + xpl + ymi + ypl + c) / 5.0;
//    res.b *= 1.5;
//    float d = dot(res, vec3(0.5, 0, 1));
//    res = mix(res, vec3(0.5, 0, 1), 0.1 * d);
//    float m = max(res.x, max(res.y, res.z));
//    if (m >= 1.0) {
//        res /= m;
//    }
//    res = res / (res + vec3(1.0));
//    fc = vec4(c, 1.0);
}