#version 330 core
in vec2 TexCoords;
out vec4 color;

uniform sampler2D punk_texture;
uniform vec3 punk_texture_color;

void main()
{    
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(punk_texture, TexCoords).r);
    color = vec4(punk_texture_color, 1.0) * sampled;
}  
