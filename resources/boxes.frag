#version 450 core

in vec4 fs_rgba;

layout(location = 0) out vec4 frag_color;

void main() {
  frag_color = fs_rgba;
}
