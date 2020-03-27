use crate::webgl::shader_program::ShaderProgram;

pub struct Tile {
  sprite_key: String,
  x: usize,
  y: usize,
  blocking: bool,
  occupied: bool
}

impl Tile {
  pub fn new(sprite_key: String, x: usize, y: usize, blocking: bool) -> Self {
    Self {
      sprite_key: sprite_key.to_owned(),
      x,
      y,
      blocking,
      occupied: false
    }
  }

  pub fn set_occupied(&mut self) {
    self.occupied = true;
  }

  pub fn set_unoccupied(&mut self) {
    self.occupied = false;
  }

  pub fn is_walkable(&self) -> bool {
    if self.blocking || self.occupied {
      return false;
    }
    true
  }

  pub fn draw(&self, program: &mut ShaderProgram, player_coords: (f32, f32)) {
    program.draw(
      self.sprite_key.to_owned(),
      540. + self.x as f32 * 64. - player_coords.0,
      360. + self.y as f32 * 64. - player_coords.1,
      64.,
      64.,
      1.
    );
  }
}
