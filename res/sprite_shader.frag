#version 330 core

in vec2 TexCoords;

uniform sampler2D texture0;

out vec4 color;

void main() {
    color = texture(texture0, TexCoords);
}
