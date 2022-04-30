//// simple.frag
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec4 varying_color;
layout(location = 1) in vec2 varying_uv_0;
layout(location = 2) in vec2 varying_uv_1;
layout(location = 3) in vec4 varying_pos;
layout(location = 4) in vec3 varying_normal;


layout(location = 0) out vec4 output_color;


layout(set=1, binding=0) uniform texture2D diffuse_tex;
layout(set=1, binding=1) uniform sampler diffuse;

layout(set=1, binding=0)
uniform Light {
    vec3 position;
    vec3 color;
} light;

vec3 ambient = vec3(0.1, 0.1, 0.0);


void main() {
    vec3 norm = normalize(varying_normal);
    vec3 light_dir = normalize(light.position - varying_pos.xyz);
    float diffuse_factor = max(dot(norm, light_dir), 0.0);
    vec4 object_albedo = texture(sampler2D(diffuse_tex, diffuse), varying_uv_0);
    vec3 diffuse= diffuse_factor * light.color;
    vec3 ambient_diffuse = (diffuse + ambient) * object_albedo.xyz;

    output_color = vec4(ambient_diffuse, object_albedo.w);


}
