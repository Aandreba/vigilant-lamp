#version 330 core
out vec4 final_color;

struct Material {
  vec4 color;
  sampler2D texture;
};

struct Light {
  vec4 color;
  float intensity;
};

uniform Material material;
uniform Light ambient;

void main() {
  final_color = material.color + (ambient.color * ambient.intensity);
}
