#version 330

// Built-in raylib locations
uniform mat4 matModel;      // model (world) matrix
uniform mat4 matView;       // view matrix
uniform mat4 matProjection; // projection matrix

in vec3 vertexPosition;
in vec3 vertexNormal;

out vec3 fragPos;
out vec3 fragNormal;

void main() {
    // Position in world space
    fragPos    = vec3(matModel * vec4(vertexPosition, 1.0));
    // Transform normal by model matrix (ignore translation)
    fragNormal = mat3(transpose(inverse(matModel))) * vertexNormal;
    // Final clip-space position
    gl_Position = matProjection * matView * vec4(fragPos, 1.0);
}
