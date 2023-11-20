#version 330 core

in vec3 color;

out vec4 fc;

//uniform float colors[3];

void main() {
//    fc=vec4(1,0,0,1);
    fc=vec4(color,1)*0.01/0.25;
//    fc=vec4(1.-colors[0],1.-colors[1],1.-colors[2],1.0);
//    fc = vec4(abs(vec3(color.x-color.y,color.y-color.z,color.z-color.x))*1000.0, 1);// * 0.01 / 0.25;
//    fc.w=1.0;
}