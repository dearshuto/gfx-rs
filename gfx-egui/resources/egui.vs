#version 450

layout(location = 0) out vec2 v_TexCoord;
layout(location = 1) out vec4 v_Color;

layout(location = 0) in vec2 i_Position;
layout(location = 1) in vec2 i_TexCoord;
layout(location = 2) in uint i_Color;

layout(binding = 2) uniform Locals {
  vec2 u_ScreenSize;
  vec2 _p;
};

void main()
{
  // カラー
  float red = float(i_Color & 0x00FF);
  float green = float((i_Color >> 8) & 0x00FF);
  float blue = float((i_Color >> 16) & 0x00FF);
  float alpha = float((i_Color >> 24) & 0x00FF);
  v_Color = vec4(red, green, blue, alpha) / 255.0;

  // UV
  v_TexCoord = i_TexCoord;

  // 位置
  float x = 2.0 * i_Position.x / u_ScreenSize.x - 1.0;
  float y = 1.0 - 2.0 * i_Position.y / u_ScreenSize.y;
  gl_Position = vec4(x, y, 0.0, 1.0);
}
