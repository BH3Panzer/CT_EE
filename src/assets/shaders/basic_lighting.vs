#version 330

uniform mat4 matModel;
uniform mat4 matView;
uniform mat4 matProjection;

in vec3 vertexPosition;
in vec3 vertexNormal;
in vec2 vertexTexCoord;

out vec3 fragPos;
out vec3 fragNormal;
out vec2 fragTexCoord;

void main() {
    fragPos = vec3(matModel * vec4(vertexPosition, 1.0));
    fragNormal = mat3(transpose(inverse(matModel))) * vertexNormal;
    fragTexCoord = vertexTexCoord;

    gl_Position = matProjection * matView * vec4(fragPos, 1.0);
}
