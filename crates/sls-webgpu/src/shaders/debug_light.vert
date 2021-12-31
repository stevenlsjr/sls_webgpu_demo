#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 vertex_position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 uv;
layout(location = 3) in vec2 uv_1;
layout(location = 4) in vec4 normal;
layout(location = 5) in vec3 tangent;
layout(location = 6) in vec3 bitangent;


// model matrix for instance
layout(location = 7) in vec4 instance_model_x;
layout(location = 8) in vec4 instance_model_y;
layout(location = 9) in vec4 instance_model_z;
layout(location = 10) in vec4 instance_model_w;


layout(location = 0) out vec4 varying_color;
layout(location = 1) out vec2 varying_uv_0;
layout(location = 2) out vec2 varying_uv_1;
layout(location = 3) out vec4 varying_pos;

layout(set=0, binding=0) uniform UniformBufferObject {
    mat4 view_projection;
} ubo;

layout(set=1, binding=0)
uniform Light {
    vec3 position;
    vec3 color;
} light_ubo;


void main() {
    mat4 model_mat = mat4(
        instance_model_x,
        instance_model_y,
        instance_model_z,
        instance_model_w
    );
    varying_uv_0 = uv;
    varying_uv_1 = uv_1;
    varying_color = normal;
    varying_pos = model_mat * vec4(vertex_position, 1.0);
    gl_Position = ubo.view_projection * varying_pos;
}