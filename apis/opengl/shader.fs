#version 330 core
out vec4 final_color;

struct Material {
  color: uint;
  texture: uint;
};

uniform Material material;

void main() {
  final_color = vec4(1.0, 0.5, 0.2, 1.0);
}
