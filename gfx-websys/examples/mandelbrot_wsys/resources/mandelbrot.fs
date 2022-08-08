#version 300 es

precision highp float;
layout(location = 0) out vec4 o_Color;
in vec4 v_UV;

void main()
{
  int LOOP_COUNT = 360;
  int count = 0;
  vec2 offset = vec2(-0.5, 0.0) + v_UV.xy;
  vec2 z = vec2(0.0);

  for (int i = 0; i < LOOP_COUNT; ++i) {
    ++count;
    if (length(z) > 2.0) {
      break;
    }

    z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + offset;
  }

  float h = log(float(count) / float(LOOP_COUNT));
  float s = 0.9;
  float v = 0.7;
  vec3 color =
    ((clamp(abs(fract(h + vec3(0.0, 2.0, 1.0) / 3.0) * 6.0 - 3.0) - 1.0, 0.0, 1.0) - 1.0)
     * s + 1.0)
    * v;
  o_Color = vec4(color, 1.0);
}
