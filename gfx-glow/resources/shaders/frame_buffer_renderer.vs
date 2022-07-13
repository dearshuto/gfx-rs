#version 410

layout(location = 0) out vec2 v_UV;

layout(location = 0) in vec2 i_Position;
layout(location = 1) in vec2 i_UV;

void main()
{
  gl_Position = vec4(i_Position, 0.0, 1.0);
  v_UV = i_UV;
}
