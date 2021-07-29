// simple.frag
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec4 varying_color;
layout(location = 1) in vec2 varying_uv;
layout(location = 2) in vec4 varying_pos;

layout(location = 0) out vec4 output_color;

vec4 lookup_tex(vec2 uv){
    if(!gl_FrontFacing){
        return vec4(1.0, 0.0, 1.0, 1.0);
    }
    float value = cos(uv.x * 3.14 * 10.f) + atan((uv.y + 0.003) * 3.14 * 2.f);
    value = smoothstep(0.8, 1.f, value);
    // Output to screen
    return vec4(value, value, value,1.0);
}

void main() {

    output_color = lookup_tex(varying_uv);
}