#version 330 core

in vec2 uv;

uniform sampler2D base;
uniform sampler2D new_frame;

out vec4 fc;

void main() {
    vec4 b = texture(base, uv);
    vec4 n = texture(new_frame, uv);
    fc = vec4((b.xyz * b.w + n.xyz) / (b.w + 1.0), b.w + 1.0);
}