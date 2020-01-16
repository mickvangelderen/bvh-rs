#version 450 core

layout(location = 0) in vec3 vs_pos_in_obj;

layout(location = 0) uniform mat4 obj_to_clp;

out vec4 fs_pos_in_clp;

void main() {
  vec4 pos_in_obj = vec4(vs_pos_in_obj, 1.0);
  gl_Position = obj_to_clp * pos_in_obj;
  fs_pos_in_clp = obj_to_clp * pos_in_obj;
}
