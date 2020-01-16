#version 450 core

in vec4 fs_pos_in_clp;

layout(location = 0) out vec4 frag_color;
layout(location = 2) uniform vec4 rgba;

void main() {
  frag_color = rgba;
}
