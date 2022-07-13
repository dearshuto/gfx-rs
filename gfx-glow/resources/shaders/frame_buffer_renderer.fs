#version 410

layout(location = 0) out vec4 o_Color;
layout(location = 0) in vec2 v_UV;

uniform sampler2D u_FrameBuffer;

void main()
{
    o_Color = texture(u_FrameBuffer, v_UV);
    // o_Color = vec4(v_UV, 0.0, 1.0);
}
