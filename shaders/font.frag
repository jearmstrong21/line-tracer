#version 330 core

in vec2 uv;

out vec4 fc;

uniform sampler2D font_texture;

void main() {
    fc = texture(font_texture, uv).xxxx;
//    fc = vec4(uv, 0, 1);
}