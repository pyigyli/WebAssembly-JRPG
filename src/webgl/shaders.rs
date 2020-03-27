pub const VERTEX_SHADER: &str = r#"
  attribute vec2 a_position;
  attribute vec2 a_texcoord;
  attribute float a_opacity;

  varying vec2 v_texcoord;
  varying float v_opacity;

  void main() {
    vec4 screenTransform = vec4(2.0 / 1080.0, -2.0 / 720.0, -1.0, 1.0);
    gl_Position = vec4(a_position * screenTransform.xy + screenTransform.zw, 0.0, 1.0);
    v_texcoord = a_texcoord;
    v_opacity = a_opacity;
  }
"#;

pub const FRAGMENT_SHADER: &str = r#"
  precision mediump float;

  uniform sampler2D spriteTexture;

  varying vec2 v_texcoord;
  varying float v_opacity;

  void main() {
    vec4 texture = texture2D(spriteTexture, v_texcoord);
    gl_FragColor = vec4(texture.r, texture.g, texture.b, texture.a * v_opacity);
  }
"#;
