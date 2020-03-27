use crate::game::animation::{Animation, Direction};
use crate::game::animation::character::CharacterAnimation;
use crate::game::map::player::Player;
use crate::game::map::tilegrid::TileGrid;
use crate::game::menu::textbox::Textbox;
use crate::webgl::shader_program::ShaderProgram;
use js_sys::Math::random;

pub struct Npc {
  animation: CharacterAnimation,
  name: String,
  x: usize,
  y: usize,
  x_sub: f32,
  y_sub: f32,
  direction: Direction,
  stationary: bool,
  can_walk: bool,
  textboxes: Vec<String>
}

impl Npc {
  pub fn new(sprite_folder: String, name: String, x: usize, y: usize, stationary: bool, textboxes: Vec<String>) -> Self {
    Self {
      animation: CharacterAnimation::new(sprite_folder),
      name,
      x,
      y,
      x_sub: 0.,
      y_sub: 0.,
      direction: Direction::Down,
      stationary,
      can_walk: true,
      textboxes
    }
  }

  pub fn update(&mut self, tiles: &mut TileGrid) {
    if !self.stationary {
      if self.can_walk {
        let random_value = random();
        if random_value < 0.01 {
          self.direction = Direction::Up;
          self.move_tile(tiles);
        } else if random_value < 0.02 {
          self.direction = Direction::Down;
          self.move_tile(tiles);
        } else if random_value < 0.03 {
          self.direction = Direction::Left;
          self.move_tile(tiles);
        } else if random_value < 0.4 {
          self.direction = Direction::Right;
          self.move_tile(tiles);
        }
      } else {
        self.finish_walking(tiles);
      }
    }
  }

  pub fn move_tile(&mut self, tiles: &mut TileGrid) {
    let (x, y) = match self.direction {
      Direction::Up    => (self.x, self.y - 1),
      Direction::Down  => (self.x, self.y + 1),
      Direction::Left  => (self.x - 1, self.y),
      Direction::Right => (self.x + 1, self.y)
    };
    if tiles.is_tile_empty(x, y) {
      self.animation.start_animation(Animation::NpcWalkTile(self.direction));
      self.can_walk = false;
      tiles.set_occupied(x, y);
    } else {
      self.animation.turn_character(self.direction);
    }
  }

  fn finish_walking(&mut self, tiles: &mut TileGrid) {
    let animation_finished = self.animation.advance_animation();
    match self.direction {
      Direction::Up    => self.y_sub -= 2.,
      Direction::Down  => self.y_sub += 2.,
      Direction::Left  => self.x_sub -= 2.,
      Direction::Right => self.x_sub += 2.
    };
    if animation_finished {
      self.can_walk = true;
      self.x_sub = 0.;
      self.y_sub = 0.;
      tiles.set_unoccupied(self.x, self.y);
      match self.direction {
        Direction::Up    => self.y -= 1,
        Direction::Down  => self.y += 1,
        Direction::Left  => self.x -= 1,
        Direction::Right => self.x += 1
      };
    }
  }

  pub fn set_direction(&mut self, direction: Direction) {
    self.direction = direction;
  }

  pub fn start_interaction(&mut self, player: &Player, textbox: &mut Textbox) {
    self.set_direction(match player.get_direction() {
      Direction::Up    => Direction::Down,
      Direction::Down  => Direction::Up,
      Direction::Left  => Direction::Right,
      Direction::Right => Direction::Left
    });
    textbox.set_textboxes(self.name.to_owned(), &self.textboxes);
  }

  pub fn get_x(&self) -> usize {
    self.x
  }

  pub fn get_y(&self) -> usize {
    self.y
  }

  pub fn draw(&self, program: &mut ShaderProgram, player_coords: (f32, f32)) {
    self.animation.draw(
      program,
      540. + self.x as f32 * 64. - player_coords.0 + self.x_sub,
      360. + self.y as f32 * 64. - player_coords.1 + self.y_sub
    );
  }
}
