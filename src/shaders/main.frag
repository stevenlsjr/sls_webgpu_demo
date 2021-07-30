// simple.frag
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec4 varying_color;
layout(location = 1) in vec2 varying_uv;
layout(location = 2) in vec4 varying_pos;

layout(set=1, binding=0) uniform texture2D diffuse_tex;
layout(set=1, binding=1) uniform sampler2D diffuse;

layout(location = 0) out vec4 output_color;

vec4 lookup_tex(vec2 uv){
    if(!gl_FrontFacing){
        return vec4(1.0, 0.0, 1.0, 1.0);
    }

    return texture(diffuse, uv);
}

void main() {

    output_color = lookup_tex(varying_uv);
}