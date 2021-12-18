uniform mat4 camera, world_matrix;
attribute vec3 pos, norm;

void main() {
    vec4 world = world_matrix * vec4(pos, 1);
    vec4 screen = camera * world;

    gl_Position = screen;
}