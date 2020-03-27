use crate::webgl::shader_program::ShaderProgram;

pub struct PrintDamage {
  value: f64,
  x: f32, 
  y: f32,
  color: [f32; 3],
  opacity: f32,
  show_time_left: f32
}

impl PrintDamage {
  pub fn new() -> Self {
    Self {
      value: 0.,
      x: 0., 
      y: 0.,
      color: [0.; 3],
      opacity: 1.,
      show_time_left: 0.
    }
  }

  pub fn set(&mut self, value: f64, x: f32, y: f32, color: [f32; 3]) {
    self.value = value;
    self.x = x;
    self.y = y;
    self.color = color;
    self.opacity = 1.;
    self.show_time_left = 60.;
  }

  pub fn update(&mut self) {
    if self.show_time_left > 0. {
      self.show_time_left -= 1.;
      self.y -= 0.0005 * self.show_time_left * self.show_time_left;
      if self.show_time_left < 30. {
        self.opacity = (self.opacity - 0.01) * self.opacity * self.opacity;
      }
    }
  }

  pub fn draw(&self, program: &mut ShaderProgram) {
    if self.show_time_left > 0. {
      for (index, character) in format!("{}", self.value).chars().enumerate() {
        let sprite_key = match character {
          '0' => "0",
          '1' => "1",
          '2' => "2",
          '3' => "3",
          '4' => "4",
          '5' => "5",
          '6' => "6",
          '7' => "7",
          '8' => "8",
          '9' => "9",
          _ => ""
        };
        program.draw(format!("fonts/numbers/{}", sprite_key), self.x + index as f32 * 20., self.y, 16., 24., self.opacity);
      }
    }
  }
}
