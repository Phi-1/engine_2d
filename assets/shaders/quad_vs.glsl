#version 330 core

layout(location = 0) in vec2 aPos;

out vec3 color;
out vec2 texCoords;

void main()
{
    color = vec3(0.5, 0.2, 0.8);
    texCoords = vec2(clamp(aPos.x, 0.0, 1.0), clamp(aPos.y, 0.0, 1.0));
    gl_Position = vec4(aPos, 0.0, 1.0);
}