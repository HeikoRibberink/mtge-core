#version 450 core

in vec3 vert_pos;
in vec2 tex_coords;
in uint tex_index;

out vec2 f_tex_coords;
out flat uint f_tex_index;

uniform mat4 camera;

void main(void) {
   f_tex_coords = tex_coords;
   f_tex_index = tex_index;
   gl_Position = camera * vec4(vert_pos.xyz, 1.0);
};