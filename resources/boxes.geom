#version 450 core

layout(points, invocations = 2) in;
layout(line_strip, max_vertices = 16) out;

layout(location = 0) uniform mat4 obj_to_clp;

in vec4 ge_rgba[];
in vec3 ge_p1[];

out vec4 fs_rgba;

#define EMIT_CORNER(px, py, pz) \
  gl_Position = obj_to_clp * vec4(px.x, py.y, pz.z, 1.0); \
  EmitVertex();

void main() {
  fs_rgba = ge_rgba[0];
  vec3 p0 = (gl_InvocationID == 0) ? gl_in[0].gl_Position.xyz : ge_p1[0];
  vec3 p1 = (gl_InvocationID == 0) ? ge_p1[0] : gl_in[0].gl_Position.xyz;
  EMIT_CORNER(p0, p0, p0);
  EMIT_CORNER(p0, p0, p1);
  EMIT_CORNER(p0, p1, p1);
  EMIT_CORNER(p0, p1, p0);
  EMIT_CORNER(p0, p0, p0);
  EMIT_CORNER(p1, p0, p0);
  EMIT_CORNER(p1, p1, p0);
  EMIT_CORNER(p0, p1, p0);
  EndPrimitive();
}
