#version 330 core

in vec2 uv;

uniform sampler2D source;

out vec4 fc;

void main() {
    fc = texture(source, uv);
}