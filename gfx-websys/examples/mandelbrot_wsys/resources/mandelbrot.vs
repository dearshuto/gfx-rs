#version 300 es

precision highp float;

layout(location = 0) in vec2 i_Position;
out vec4 v_UV;

void main()
{
  gl_Position = vec4(i_Position.x, i_Position.y, 0.0, 1.0);

  // [-1, 1] -> [0, 1]
  v_UV = vec4(0.5 * (1.0 + i_Position), 0.0, 0.0);
}
