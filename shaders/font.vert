#version 330 core

layout (location=0) in vec2 inPos;

out vec2 uv;

uniform vec2 coords;

uniform vec2 spos;
uniform vec2 ssize;

void main() {
    gl_Position = vec4(spos + ssize * inPos, 0, 1);
    uv = inPos / 16.0 + coords / 16.0;
//    uv = inPos;
}