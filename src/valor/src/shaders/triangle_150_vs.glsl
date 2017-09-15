#version 150 core

in vec4 a_Pos;
in vec3 a_Color;
out vec4 v_Color;

uniform b_Locals {
    mat4 u_World;
    // vec4 u_Color;
    // vec4 u_MatParams;
    // vec4 u_UvRange;
};

uniform b_Globals {
    mat4 u_ViewProj;
};

void main() {
  v_Color = vec4(a_Color, 1.0);
  gl_Position = u_ViewProj * u_World * a_Pos;
}
