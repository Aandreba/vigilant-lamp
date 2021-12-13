#version 330 core
out vec4 final_color;

struct Material {
  vec4 color;
  uint texture;
};

uniform Material material;

void main() {
  final_color = material.color;
}
