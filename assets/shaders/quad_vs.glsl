#version 330 core

layout(location = 0) in vec3 aPos;

out vec3 color;

void main()
{
    color = vec3(0.5, 0.2, 0.8);
    gl_Position = vec4(aPos, 1.0);
}