#version 450 core

layout(location = 0) in vec3 vs_p0;
layout(location = 1) in vec3 vs_p1;
layout(location = 2) in vec4 vs_rgba;

out vec3 ge_p1;
out vec4 ge_rgba;

void main() {
  vec4 pos_in_obj = vec4(vs_p0, 1.0);
  gl_Position = pos_in_obj;
  ge_p1 = vs_p1;
  ge_rgba = vs_rgba;
}
