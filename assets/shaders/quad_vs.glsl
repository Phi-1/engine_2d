#version 330 core

layout(location = 0) in vec2 aPos;
layout(location = 1) in vec2 aTex;

out vec3 color;
out vec2 texCoords;

uniform mat4 model;
uniform mat4 projection;

void main()
{
    color = vec3(0.5, 0.2, 0.8);
    texCoords = aTex;
    gl_Position = projection * model * vec4(aPos, 0.0, 1.0);
}