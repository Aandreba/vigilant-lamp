#version 330 core
const float SPEC_STRENGTH = 64;

struct Material {
  vec4 color;
  sampler2D texture;
};

struct Light {
  vec4 color;
  float intensity;
};

struct PointLight {
  vec3 position;
  Light light;
};

vec4 diffuse (in vec3 pos, in vec3 norm, in Light light, in vec3 light_pos) {
  vec3 light_dist = light_pos - pos;
  vec3 light_dir = normalize(light_dist);
  float light_dist_norm2 = dot(light_dist, light_dist);

  float diff = max(dot(norm, light_dir), 0) * light.intensity;
  return light.color * diff / light_dist_norm2;
}

vec4 specular (in vec3 cam_pos, ) {

}

uniform Material material;
uniform Light ambient;
uniform PointLight point;

in vec3 world_pos;
in vec3 out_norm;
out vec4 final_color;

void main() {
  vec4 ambient_color = ambient.color * ambient.intensity;
  vec4 diffuse_color = diffuse(world_pos, out_norm, point.light, point.position);

  final_color = material.color * (ambient_color + diffuse_color);
}
