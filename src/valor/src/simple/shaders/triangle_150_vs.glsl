#version 150 core

// Attributes
in vec4 position;
in vec3 color;
out vec4 v_Color;

// Uniforms
uniform mat4 u_World;
// vec4 u_Color;
// vec4 u_MatParams;
// vec4 u_UvRange;

uniform mat4 u_ViewProj;

void main() {
  v_Color = vec4(color, 1.0);
  gl_Position = u_ViewProj * u_World * position;
}
