use crate::webgl::shader_program::ShaderProgram;

pub struct MenuContainer {
  x1: f32,
  y1: f32,
  x2: f32,
  y2: f32
}

impl MenuContainer {
  pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
    Self {
      x1,
      y1,
      x2,
      y2
    }
  }

  pub fn draw(&self, program: &mut ShaderProgram) {
    program.draw(String::from("menu/up_left"),     self.x1,       self.y1,       16.,                     16.,                     1.);
    program.draw(String::from("menu/up"),          self.x1 + 16., self.y1,       self.x2 - self.x1 - 32., 16.,                     1.);
    program.draw(String::from("menu/up_right"),    self.x2 - 16., self.y1,       16.,                     16.,                     1.);
    program.draw(String::from("menu/left"),        self.x1,       self.y1 + 16., 16.,                     self.y2 - self.y1 - 32., 1.);
    program.draw(String::from("menu/middle"),      self.x1 + 16., self.y1 + 16., self.x2 - self.x1 - 32., self.y2 - self.y1 - 32., 1.);
    program.draw(String::from("menu/right"),       self.x2 - 16., self.y1 + 16., 16.,                     self.y2 - self.y1 - 32., 1.);
    program.draw(String::from("menu/down_left"),   self.x1,       self.y2 - 16., 16.,                     16.,                     1.);
    program.draw(String::from("menu/down"),        self.x1 + 16., self.y2 - 16., self.x2 - self.x1 - 32., 16.,                     1.);
    program.draw(String::from("menu/down_right"),  self.x2 - 16., self.y2 - 16., 16.,                     16.,                     1.);
  }
}
