#version 300 es

uniform Constants
{
  mat4 transform;
};

in mediump vec2 pos;
in mediump vec2 uv;
in lowp vec4 col;

out mediump vec2 f_uv;
out lowp vec4 f_color;

// Built-in:
// vec4 gl_Position

void main() {
  f_uv = uv;
  f_color = col;
  gl_Position = transform * vec4(pos.xy, 0, 1);
}
