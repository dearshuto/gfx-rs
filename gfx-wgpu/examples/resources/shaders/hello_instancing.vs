#version 450

layout (location = 0) in vec2 i_Position;

layout (binding = 0, set = 0) uniform TransformData
{
    vec4 u_Positions[9];
};

void main()
{
    gl_Position = vec4(i_Position + u_Positions[gl_InstanceIndex].xy, 0.0, 1.0);
}
