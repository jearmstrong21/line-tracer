#version 330

layout (location=0) in vec2 inPos;

out vec2 pos;

void main() {
    gl_Position = vec4(inPos * 2.0 - 1.0, 0, 1);
    pos = inPos;
}