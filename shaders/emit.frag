#version 330 core

in vec3 color;

out vec4 fc;

void main() {
    fc = vec4(color, 1) * 0.01 / 0.25;
}