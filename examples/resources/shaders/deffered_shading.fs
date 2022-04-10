#version 450

layout(location = 0) out vec4 o_Color;
layout(location = 0) in vec2 v_Uv;

layout(binding = 0, set = 0) uniform texture2D u_AlbedoTexture;
layout(binding = 1, set = 0) uniform texture2D u_NormalTexture;
layout(binding = 2, set = 0) uniform texture2D u_DepthTexture;
layout(binding = 3, set = 0) uniform sampler u_Sampler;

void main() {
    vec3 albedo = texture(sampler2D(u_AlbedoTexture, u_Sampler), v_Uv).xyz;
    vec3 normal = 2.0 * texture(sampler2D(u_NormalTexture, u_Sampler), v_Uv).xyz - 1.0;
    float depth = texture(sampler2D(u_DepthTexture, u_Sampler), v_Uv).x;

    float diffuseValue = max(0.0, dot(normal, vec3(0.0, 0.0, -1.0)));
    o_Color = vec4(albedo * diffuseValue, 1.0);
}
