//// simple.frag
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec4 varying_color;
layout(location = 1) in vec2 varying_uv_0;
layout(location = 2) in vec2 varying_uv_1;
layout(location = 3) in vec4 varying_pos;
layout(location = 0) out vec4 output_color;


layout(set=1, binding=0) uniform texture2D diffuse_tex;
layout(set=1, binding=1) uniform sampler diffuse;

layout(set=1, binding=0)
uniform Light {
    vec3 position;
    vec3 color;
};

void main() {

    output_color= vec4(1.0, 1.0, 1.0, 1.0);
}
