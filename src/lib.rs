mod webgl;
mod game;

use webgl::audio::Audio;
use webgl::{gl_setup, keyboard};
use webgl::shader_program::ShaderProgram;
use crate::game::GameState;

#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

// use wasm_bindgen::prelude::*;
// #[wasm_bindgen]
// extern "C" {
//   #[wasm_bindgen(js_namespace = console)]
//   fn log(s: &str);
// }

#[wasm_bindgen]
pub struct GameClient {
  game_state: GameState,
  audio: Audio,
  shader_program: ShaderProgram,
  gl: WebGlRenderingContext
}

#[wasm_bindgen]
impl GameClient {

  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    console_error_panic_hook::set_once();
    let gl = gl_setup::initialize_webgl_context().unwrap();
    keyboard::attach_keyboard_events().unwrap();
    Self {
      game_state: GameState::new(),
      audio: Audio::new(),
      shader_program: ShaderProgram::new(&gl),
      gl
    }
  }

  pub fn update(&mut self) {
    self.game_state.update(&mut self.audio);
  }

  pub fn add_sprite(&mut self, key: String, sprite: HtmlImageElement) {
    self.shader_program.add_to_sprite_data(key, sprite);
  }

  pub fn add_soundtrack(&mut self, soundtrack: HtmlAudioElement) {
    self.audio.add_soundtrack(soundtrack);
  }

  pub fn add_sfx(&mut self, sfx: HtmlAudioElement) {
    self.audio.add_sfx(sfx);
  }

  pub fn render(&mut self) {
    self.gl.clear(GL::COLOR_BUFFER_BIT);
    self.game_state.draw(&mut self.shader_program);
    self.shader_program.render(&self.gl);
  }
}
