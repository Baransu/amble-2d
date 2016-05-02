#version 330 core

layout (location = 0) in vec2 position;
layout (location = 1) in vec2 texcoord;

uniform mat4 projection;
uniform mat4 model;

out vec2 TexCoord;

void main() {
    gl_Position = projection * model * vec4(position.x * 10.0, position.y * 10.0, 0.0, 1.0);
    TexCoord = texcoord;
}
