use crate::game::animation::{Animation, Direction};
use crate::game::animation::character::CharacterAnimation;
use crate::game::battle::Battle;
use crate::game::battle::character::Character;
use crate::game::map::Map;
use crate::game::map::npc::Npc;
use crate::game::menu::textbox::Textbox;
use crate::game::transition::Transition;
use crate::webgl::keyboard::{is_pressed, is_down};
use crate::webgl::shader_program::ShaderProgram;

pub struct Player {
  animation: CharacterAnimation,
  x: usize,
  y: usize,
  x_sub: f32,
  y_sub: f32,
  direction: Direction,
  can_walk: bool
}

impl Player {
  pub fn new() -> Self {
    Self {
      animation: CharacterAnimation::new(String::from("Darrel_Deen")),
      x: 0,
      y: 0,
      x_sub: 0.,
      y_sub: 0.,
      direction: Direction::Down,
      can_walk: true
    }
  }

  pub fn update(&mut self, map: &mut Map, party: &mut Vec<Character>, battle: &mut Battle, transition: &mut Transition, textbox: &mut Textbox) {
    if self.can_walk {
      if is_pressed("a") {
        let (x, y) = self.get_position_in_front();
        self.attempt_interaction(map, textbox, x, y);

      } else if is_down("up") {
        self.direction = Direction::Up;
        self.move_tile(map);

      } else if is_down("down") {
        self.direction = Direction::Down;
        self.move_tile(map);

      } else if is_down("left") {
        self.direction = Direction::Left;
        self.move_tile(map);

      } else if is_down("right") {
        self.direction = Direction::Right;
        self.move_tile(map);
      }
    } else {
      self.finish_walking(map, party, battle, transition);
    }
  }

  pub fn move_tile(&mut self, map: &mut Map) {
    let (x, y) = self.get_position_in_front();
    if map.is_tile_empty(x, y) {
      self.animation.start_animation(Animation::WalkTile(self.direction));
      self.can_walk = false;
      map.set_occupied(x, y);
    } else {
      self.animation.turn_character(self.direction);
    }
  }

  fn finish_walking(&mut self, map: &mut Map, party: &mut Vec<Character>, battle: &mut Battle, transition: &mut Transition) {
    let animation_finished = self.animation.advance_animation();
    match self.direction {
      Direction::Up    => self.y_sub -= 8.,
      Direction::Down  => self.y_sub += 8.,
      Direction::Left  => self.x_sub -= 8.,
      Direction::Right => self.x_sub += 8.
    };
    if animation_finished {
      self.can_walk = true;
      self.x_sub = 0.;
      self.y_sub = 0.;
      map.set_unoccupied(self.x, self.y);
      match self.direction {
        Direction::Up    => self.y -= 1,
        Direction::Down  => self.y += 1,
        Direction::Left  => self.x -= 1,
        Direction::Right => self.x += 1
      };
      if let Some(enemies) = map.get_encounter() {
        battle.start_battle(party, enemies, transition);
      }
    }
  }

  pub fn attempt_interaction(&self, map: &mut Map, textbox: &mut Textbox, x: usize, y: usize) {
    if let Some(npc) = map.npcs.iter_mut().find(|npc: &&mut Npc| npc.get_x() == x && npc.get_y() == y) {
      npc.start_interaction(&self, textbox);
    }
  }

  pub fn set(&mut self, x: usize, y: usize, direction: Direction) {
    self.x = x;
    self.y = y;
    self.direction = direction;
  }

  pub fn set_character_sprites(&mut self, sprite_folder: String) {
    self.animation = CharacterAnimation::new(sprite_folder);
  }

  pub fn get_coords(&self) -> (f32, f32) {
    (self.x as f32 * 64. + self.x_sub, self.y as f32 * 64. + self.y_sub)
  }

  pub fn get_position_in_front(&self) -> (usize, usize) {
    match self.direction {
      Direction::Up    => (self.x, self.y - 1),
      Direction::Down  => (self.x, self.y + 1),
      Direction::Left  => (self.x - 1, self.y),
      Direction::Right => (self.x + 1, self.y)
    }
  }

  pub fn get_direction(&self) -> Direction {
    self.direction
  }

  pub fn draw(&self, program: &mut ShaderProgram) {
    self.animation.draw(program, 540., 360.);
  }
}
