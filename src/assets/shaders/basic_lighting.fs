#version 330

// Standard raylib parameters
uniform vec3 viewPos;     // camera/world position

// Custom ambient light
uniform vec4 ambient;     // e.g. vec4(0.2,0.2,0.2,1.0)

// Light structure (we'll bind only one)
struct Light {
    vec3 position;
    vec3 color;
};

uniform Light light0;

in vec3 fragPos;
in vec3 fragNormal;

out vec4 outColor;

void main() {
    // Normalize inputs
    vec3 N = normalize(fragNormal);
    vec3 L = normalize(light0.position - fragPos);
    vec3 V = normalize(viewPos - fragPos);
    vec3 H = normalize(L + V);

    // Ambient term
    vec3 ambientTerm = ambient.rgb * ambient.a;

    // Diffuse (Lambert)
    float diff = max(dot(N, L), 0.0);
    vec3 diffuseTerm = diff * light0.color;

    // Specular (Blinn-Phong)
    float spec = pow(max(dot(N, H), 0.0), 16.0);
    vec3 specularTerm = spec * light0.color;

    vec3 result = ambientTerm + diffuseTerm + specularTerm;
    outColor = vec4(result, 1.0);
}
