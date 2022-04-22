#version 450 core
#extension GL_ARB_bindless_texture:require

in vec2 f_tex_coords;
in flat uint f_tex_index;

uniform Textures{
   sampler2D textures[<%texture_count%>];
};
 
out vec4 color;

void main(void){
   color=texture(textures[f_tex_index],f_tex_coords);
};