#version 330 core
uniform mat4 camera, world_matrix;

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 norm;

out vec3 world_pos;
out vec3 out_norm;

void main() {
    vec4 world = world_matrix * vec4(pos, 1);
    vec4 screen = camera * world;

    world_pos = world.xyz;
    out_norm = norm;
    gl_Position = screen;
}