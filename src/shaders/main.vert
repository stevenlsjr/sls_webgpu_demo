#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 vertex_position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 uv;
layout(location = 3) in vec4 normal;
layout(location = 4) in vec3 tangent;
layout(location = 5) in vec3 bitangent;


// model matrix for instance
layout(location = 6) in vec4 instance_model_x;
layout(location = 7) in vec4 instance_model_y;
layout(location = 8) in vec4 instance_model_z;
layout(location = 9) in vec4 instance_model_w;


layout(location = 0) out vec4 varying_color;
layout(location = 1) out vec2 varying_uv;
layout(location = 2) out vec4 varying_pos;

layout(binding=0) uniform UniformBufferObject {
    mat4 view_projection;
} ubo;

void main() {
    mat4 model_mat = mat4(
        instance_model_x,
        instance_model_y,
        instance_model_z,
        instance_model_w
    );
    varying_uv = uv;
    varying_color = normal;
    varying_pos = model_mat * vec4(vertex_position, 1.0);
    gl_Position = ubo.view_projection * varying_pos;
}