use crate::game::menu::container::MenuContainer;
use crate::game::menu::font::print_text;
use crate::webgl::shader_program::ShaderProgram;

pub struct Notification {
  text: String,
  show_time_left: u8
}

impl Notification {
  pub fn new() -> Self {
    Self {
      text: String::new(),
      show_time_left: 0
    }
  }

  pub fn update(&mut self) {
    if self.show_time_left > 0 {
      self.show_time_left -= 1;
    }
  }

  pub fn set_notification(&mut self, text: String) {
    self.text = text;
    self.show_time_left = 60;
  }

  pub fn draw(&self, program: &mut ShaderProgram) {
    if self.show_time_left > 0 {
      MenuContainer::new(16., 16., 1064., 80.).draw(program);
      print_text(program, self.text.to_owned(), 540. - self.text.len() as f32 * 8., 36.);
    }
  }
}