#version 330 core
const float SPEC_STRENGTH = 0.5;

struct Material {
  vec4 color;
  float shininess;
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

uniform vec3 cam_pos;
uniform Material material;
uniform Light ambient;
uniform PointLight point;

vec4 calculate (in vec3 pos, in vec3 norm, in vec3 cam_pos, in Light light, in vec3 light_pos) {
  // DIFFUSE
  vec3 light_dist = light_pos - pos;
  vec3 light_dir = normalize(light_dist);
  float light_dist_norm2 = dot(light_dist, light_dist);
  float diff = max(dot(norm, light_dir), 0);

  // SPECULAR
  vec3 view_dir = normalize(cam_pos - pos);
  vec3 reflect_dir = reflect(-light_dir, norm);
  float spec = clamp(SPEC_STRENGTH * pow(max(dot(view_dir, reflect_dir), 0), material.shininess), 0, 1);

  return light.color * (diff + spec) * light.intensity / light_dist_norm2;
}

in vec3 world_pos;
in vec3 out_norm;
out vec4 final_color;

void main() {
  vec4 ambient_color = ambient.color * ambient.intensity;
  vec4 point_color = calculate(world_pos, out_norm, cam_pos, point.light, point.position);

  final_color = material.color * (ambient_color + point_color);
}
