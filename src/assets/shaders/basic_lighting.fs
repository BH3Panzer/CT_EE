#version 330

uniform vec3 viewPos;
uniform vec4 ambient;

struct Light {
    vec3 position;
    vec3 color;
};

uniform Light light0;
uniform sampler2D texture0;

in vec3 fragPos;
in vec3 fragNormal;
in vec2 fragTexCoord;

out vec4 outColor;

void main() {
    vec3 N = normalize(fragNormal);
    vec3 L = normalize(light0.position - fragPos);
    vec3 V = normalize(viewPos - fragPos);
    vec3 H = normalize(L + V);

    vec3 ambientTerm = ambient.rgb * ambient.a;
    float diff = max(dot(N, L), 0.0);
    vec3 diffuseTerm = diff * light0.color;

    float spec = pow(max(dot(N, H), 0.0), 16.0);
    vec3 specularTerm = spec * light0.color;

    vec3 lightResult = ambientTerm + diffuseTerm + specularTerm;

    vec4 texColor = texture(texture0, fragTexCoord);

    outColor = vec4(lightResult, 1.0) * texColor;
}
