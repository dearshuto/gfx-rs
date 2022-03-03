#version 450

layout(location = 0) in vec3 i_Position;

layout(location = 0) out vec3 v_Color;

void main()
{
    gl_Position = vec4(i_Position, 1.0);

    v_Color = vec3(gl_VertexIndex / 3);
}
