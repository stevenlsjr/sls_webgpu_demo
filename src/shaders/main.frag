// simple.frag
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec4 varying_color;
layout(location = 1) in vec4 varying_pos;
layout(location = 0) out vec4 output_color;

void main() {
    output_color = vec4(
        sin(varying_pos.x * 10.0) + 0.5,
        cos(varying_pos.y * 10.0) + 0.5,
        sin(varying_pos.z * 10.f) + 0.5, 1.0);
}