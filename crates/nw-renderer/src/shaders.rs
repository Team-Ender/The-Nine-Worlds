pub const VERTEX_SHADER: &str = r#"
#version 450

layout(location = 0) out vec3 vertex_color;

vec2 positions[3] = vec2[](
    vec2( 0.0, -0.7),
    vec2( 0.7,  0.7),
    vec2(-0.7,  0.7)
);

vec3 colors[3] = vec3[](
    vec3(1.0, 0.0, 0.0),
    vec3(0.0, 1.0, 0.0),
    vec3(0.0, 0.0, 1.0)
);

void main() {
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    vertex_color = colors[gl_VertexIndex];
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 vertex_color;
layout(location = 0) out vec4 out_color;

void main() {
    out_color = vec4(vertex_color, 1.0);
}
"#;