#version 330 es
uniform mat4 camera, world_matrix;

layout (location = 0) in vec3 pos;

void main() {
    vec4 world = world_matrix * vec4(pos, 1);
    vec4 screen = camera * world;

    gl_Position = screen;
}