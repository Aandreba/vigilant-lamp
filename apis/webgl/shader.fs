struct Material {
  highp vec4 color;
  sampler2D texture;
};

uniform Material material;

void main() {
  gl_FragColor = material.color;
}
