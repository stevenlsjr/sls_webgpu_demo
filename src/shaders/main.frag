// simple.frag
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec4 varying_color;
layout(location = 1) in vec2 varying_uv;
layout(location = 2) in vec4 varying_pos;

layout(location = 0) out vec4 output_color;

vec4 lookup_tex(vec2 uv){
    return vec4(sin(uv.x), cos(uv.y), 1.0, 1.0);
}

void main() {
    output_color = lookup_tex(varying_uv);
}