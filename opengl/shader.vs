#version 330 core
uniform mat4 world_matrix;

layout (location = 0) in vec3 pos;

void main() {
    vec4 world = world_matrix * vec4(pos, 1);
    gl_Position = world;
}