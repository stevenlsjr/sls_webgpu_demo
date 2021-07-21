#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 vertex_position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec4 normal;


layout(location = 0) out vec4 varying_color;
layout(location = 1) out vec4 varying_pos;

layout(binding=0) uniform UniformBufferObject {
    mat4 view_projection;
} ubo;

void main() {
    varying_color = normal;
    varying_pos = vec4(vertex_position, 1.0);
    gl_Position = ubo.view_projection * vec4(vertex_position, 1.0);
}