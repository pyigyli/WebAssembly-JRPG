use js_sys::WebAssembly;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use crate::webgl::*;

const SPRITE_MAX_SIZE: usize = 1000;

pub struct ShaderProgram {
  program: WebGlProgram,
  sprite_data: HashMap<String, HtmlImageElement>,
  sprite_keys: Vec<String>,
  vertex_data: [f32; SPRITE_MAX_SIZE * 12],
  texture_data: [f32; SPRITE_MAX_SIZE * 12],
  opacity_data: [f32; SPRITE_MAX_SIZE * 6],
  data_size: usize
}

impl ShaderProgram {
  pub fn new(gl: &WebGlRenderingContext) -> Self {
    let program = link_to_program(&gl, shaders::VERTEX_SHADER, shaders::FRAGMENT_SHADER).unwrap();
    Self {
      program,
      sprite_data: HashMap::new(),
      sprite_keys: Vec::new(),
      vertex_data: [0.; SPRITE_MAX_SIZE * 12],
      texture_data: [0.; SPRITE_MAX_SIZE * 12],
      opacity_data: [0.; SPRITE_MAX_SIZE * 6],
      data_size: 0
    }
  }

  pub fn add_to_sprite_data(&mut self, key: String, sprite: HtmlImageElement) {
    self.sprite_data.insert(key, sprite);
  }

  pub fn get_sprite(&self, key: &String) -> &HtmlImageElement {
    self.sprite_data.get(key).unwrap()
  }

  pub fn draw(&mut self, key: String, x: f32, y: f32, width: f32, height: f32, opacity: f32) {
    self.sprite_keys.push(key);
    self.vertex_data[self.data_size * 12]      = x;
    self.vertex_data[self.data_size * 12 + 1]  = y;
    self.vertex_data[self.data_size * 12 + 2]  = x;
    self.vertex_data[self.data_size * 12 + 3]  = y + height;
    self.vertex_data[self.data_size * 12 + 4]  = x + width;
    self.vertex_data[self.data_size * 12 + 5]  = y;
    self.vertex_data[self.data_size * 12 + 6]  = x;
    self.vertex_data[self.data_size * 12 + 7]  = y + height;
    self.vertex_data[self.data_size * 12 + 8]  = x + width;
    self.vertex_data[self.data_size * 12 + 9]  = y + height;
    self.vertex_data[self.data_size * 12 + 10] = x + width;
    self.vertex_data[self.data_size * 12 + 11] = y;
    self.texture_data[self.data_size * 12]      = 0.;
    self.texture_data[self.data_size * 12 + 1]  = 0.;
    self.texture_data[self.data_size * 12 + 2]  = 0.;
    self.texture_data[self.data_size * 12 + 3]  = 1.;
    self.texture_data[self.data_size * 12 + 4]  = 1.;
    self.texture_data[self.data_size * 12 + 5]  = 0.;
    self.texture_data[self.data_size * 12 + 6]  = 0.;
    self.texture_data[self.data_size * 12 + 7]  = 1.;
    self.texture_data[self.data_size * 12 + 8]  = 1.;
    self.texture_data[self.data_size * 12 + 9]  = 1.;
    self.texture_data[self.data_size * 12 + 10] = 1.;
    self.texture_data[self.data_size * 12 + 11] = 0.;
    for i in 0..6 {
      self.opacity_data[self.data_size * 6 + i] = opacity;
    }
    self.data_size += 1;
  }

  pub fn render(&mut self, gl: &WebGlRenderingContext) {
    gl.use_program(Some(&self.program));
    let memory_buffer = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();

    let vertices_location = self.vertex_data.as_ptr() as u32 / 4;
    let vertices_array = js_sys::Float32Array::new(&memory_buffer).subarray(vertices_location, vertices_location + self.vertex_data.len() as u32);
    let vertices_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertices_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::STATIC_DRAW);

    let pos_location = gl.get_attrib_location(&self.program, "a_position") as u32;
    gl.enable_vertex_attrib_array(pos_location);
    gl.vertex_attrib_pointer_with_i32(pos_location, 2, GL::FLOAT, false, 0, 0);

    let texcoord_location = self.texture_data.as_ptr() as u32 / 4;
    let texcoord_array = js_sys::Float32Array::new(&memory_buffer).subarray(texcoord_location, texcoord_location + self.texture_data.len() as u32);
    let texcoord_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&texcoord_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &texcoord_array, GL::STATIC_DRAW);

    let texcoord_location = gl.get_attrib_location(&self.program, "a_texcoord") as u32;
    gl.enable_vertex_attrib_array(texcoord_location);
    gl.vertex_attrib_pointer_with_i32(texcoord_location, 2, GL::FLOAT, true, 0, 0);

    let opacity_location = self.opacity_data.as_ptr() as u32 / 4;
    let opacity_array = js_sys::Float32Array::new(&memory_buffer).subarray(opacity_location, opacity_location + self.opacity_data.len() as u32);
    let opacity_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&opacity_buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &opacity_array, GL::STATIC_DRAW);

    let opacity_location = gl.get_attrib_location(&self.program, "a_opacity") as u32;
    gl.enable_vertex_attrib_array(opacity_location);
    gl.vertex_attrib_pointer_with_i32(opacity_location, 1, GL::FLOAT, false, 0, 0);

    for (index, key) in self.sprite_keys.iter().enumerate() {
      let sprite = self.get_sprite(key);
      let gl_texture = gl.create_texture().unwrap();
      gl.active_texture(GL::TEXTURE0);
      gl.bind_texture(GL::TEXTURE_2D, Some(&gl_texture));

      gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
      gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
      gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
      gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
      gl.tex_image_2d_with_u32_and_u32_and_image(GL::TEXTURE_2D, 0, GL::RGBA as i32, GL::RGBA, GL::UNSIGNED_BYTE, sprite).unwrap();

      gl.draw_arrays(GL::TRIANGLES, index as i32 * 6, 6);
    }

    self.sprite_keys.clear();
    self.data_size = 0;
  }
}
