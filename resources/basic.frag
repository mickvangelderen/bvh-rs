#version 450 core

in vec4 fs_pos_in_clp;

layout(location = 0) out vec4 frag_color;
layout(location = 1) uniform mat4 clp_to_cam;

vec3 from_homogeneous(vec4 p) {
  return p.xyz/p.w;
}

void main() {
  float z0 = -200.0f;
  float z1 = -0.5f;

  vec3 pos_in_cam = from_homogeneous(clp_to_cam * fs_pos_in_clp);

  frag_color = vec4(vec3((pos_in_cam.z - z0)/(z1 - z0)), 1.0);
}
