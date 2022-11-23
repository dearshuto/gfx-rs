#version 450

layout(location = 0) out vec3 v_Normal;

layout(location = 0) in vec3 i_Position;
layout(location = 1) in vec3 i_Normal;

layout(binding = 0) uniform ConstantBuffer
{
    vec4 u_ProjectionViewMatrix[4];
};

void main()
{
    gl_Position = vec4(
        dot(u_ProjectionViewMatrix[0], vec4(i_Position, 1.0)),
        dot(u_ProjectionViewMatrix[1], vec4(i_Position, 1.0)),
        dot(u_ProjectionViewMatrix[2], vec4(i_Position, 1.0)),
        dot(u_ProjectionViewMatrix[3], vec4(i_Position, 1.0))
    );
    v_Normal = i_Normal;
}
