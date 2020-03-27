use std::sync::{Arc,Mutex};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::*;
use web_sys::*;

lazy_static! {
  static ref KEY_STATES: Mutex<Arc<Keyboard>> = Mutex::new(Arc::new(Keyboard::new()));
}

pub fn attach_keyboard_events() -> Result<(), JsValue> {
  let keydown_handler = move |event: KeyboardEvent| {
    let mut key_states = KEY_STATES.lock().unwrap();
    let mut new_states = Keyboard {..*key_states.clone()};
    match event.key().as_str() {
      "a"          => new_states.handle_keydown("a"),
      "s"          => new_states.handle_keydown("s"),
      "d"          => new_states.handle_keydown("d"),
      "f"          => new_states.handle_keydown("f"),
      "ArrowUp"    => new_states.handle_keydown("up"),
      "ArrowDown"  => new_states.handle_keydown("down"),
      "ArrowLeft"  => new_states.handle_keydown("left"),
      "ArrowRight" => new_states.handle_keydown("right"),
      _ => ()
    };
    *key_states = Arc::new(new_states);
  };
  
  let keydown_handler = Closure::wrap(Box::new(keydown_handler) as Box<dyn FnMut(_)>);
  window().unwrap().add_event_listener_with_callback("keydown", keydown_handler.as_ref().unchecked_ref())?;
  keydown_handler.forget();
  
  let keyup_handler = move |event: KeyboardEvent| {
    let mut key_states = KEY_STATES.lock().unwrap();
    let mut new_states = Keyboard {..*key_states.clone()};
    match event.key().as_str() {
      "a"          => new_states.handle_keyup("a"),
      "s"          => new_states.handle_keyup("s"),
      "d"          => new_states.handle_keyup("d"),
      "f"          => new_states.handle_keyup("f"),
      "ArrowUp"    => new_states.handle_keyup("up"),
      "ArrowDown"  => new_states.handle_keyup("down"),
      "ArrowLeft"  => new_states.handle_keyup("left"),
      "ArrowRight" => new_states.handle_keyup("right"),
      _ => ()
    };
    *key_states = Arc::new(new_states);
  };

  let keyup_handler = Closure::wrap(Box::new(keyup_handler) as Box<dyn FnMut(_)>);
  window().unwrap().add_event_listener_with_callback("keyup", keyup_handler.as_ref().unchecked_ref())?;
  keyup_handler.forget();

  Ok(())
}

pub fn is_pressed(key: &str) -> bool {
  let mut states = KEY_STATES.lock().unwrap();
  if states.is_pressed(key) && !states.is_held(key) {
    let mut new_states = Keyboard {..*states.clone()};
    new_states.handle_keyheld(key);
    *states = Arc::new(new_states);
    return true;
  }
  false
}

pub fn is_down(key: &str) -> bool {
  if KEY_STATES.lock().unwrap().is_pressed(key)  {
    return true;
  }
  false
}

pub struct Keyboard {
  a: bool,
  a_held: bool,
  s: bool,
  s_held: bool,
  d: bool,
  d_held: bool,
  f: bool,
  f_held: bool,
  up: bool,
  up_held: bool,
  down: bool,
  down_held: bool,
  left: bool,
  left_held: bool,
  right: bool,
  right_held: bool
}

impl Keyboard {
  pub fn new() -> Self {
    Self {
      a: false,
      a_held: false,
      s: false,
      s_held: false,
      d: false,
      d_held: false,
      f: false,
      f_held: false,
      up: false,
      up_held: false,
      down: false,
      down_held: false,
      left: false,
      left_held: false,
      right: false,
      right_held: false
    }
  }

  fn handle_keydown(&mut self, key: &str) {
    match key {
      "a"     => self.a     = true,
      "s"     => self.s     = true,
      "d"     => self.d     = true,
      "f"     => self.f     = true,
      "up"    => self.up    = true,
      "down"  => self.down  = true,
      "left"  => self.left  = true,
      "right" => self.right = true,
      _ => ()
    }
  }

  fn handle_keyheld(&mut self, key: &str) {
    match key {
      "a"     => self.a_held     = true,
      "s"     => self.s_held     = true,
      "d"     => self.d_held     = true,
      "f"     => self.f_held     = true,
      "up"    => self.up_held    = true,
      "down"  => self.down_held  = true,
      "left"  => self.left_held  = true,
      "right" => self.right_held = true,
      _ => ()
    }
  }

  fn handle_keyup(&mut self, key: &str) {
    match key {
      "a"     => {self.a     = false; self.a_held     = false;},
      "s"     => {self.s     = false; self.s_held     = false;},
      "d"     => {self.d     = false; self.d_held     = false;},
      "f"     => {self.f     = false; self.f_held     = false;},
      "up"    => {self.up    = false; self.up_held    = false;},
      "down"  => {self.down  = false; self.down_held  = false;},
      "left"  => {self.left  = false; self.left_held  = false;},
      "right" => {self.right = false; self.right_held = false;},
      _ => ()
    }
  }

  fn is_pressed(&self, key: &str) -> bool {
    match key {
      "a"     => self.a,
      "s"     => self.s,
      "d"     => self.d,
      "f"     => self.f,
      "up"    => self.up,
      "down"  => self.down,
      "left"  => self.left,
      "right" => self.right,
      _ => false
    }
  }

  fn is_held(&self, key: &str) -> bool {
    match key {
      "a"     => self.a_held,
      "s"     => self.s_held,
      "d"     => self.d_held,
      "f"     => self.f_held,
      "up"    => self.up_held,
      "down"  => self.down_held,
      "left"  => self.left_held,
      "right" => self.right_held,
      _ => false
    }
  }
}
