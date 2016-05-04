#version 330 core

layout (location = 0) in vec2 position;
layout (location = 1) in vec2 texcoord;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 view;

out vec2 TexCoords;

void main() {
    gl_Position = projection * view * model * vec4(position.x, position.y, 0.0, 1.0);
    TexCoords = texcoord;
}
