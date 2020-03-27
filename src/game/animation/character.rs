use crate::game::animation::{Animation, Direction};
use crate::webgl::shader_program::ShaderProgram;

enum Sprite {
  StandLeft,
  WalkLeft1,
  WalkLeft2,
  StandRight,
  WalkRight1,
  WalkRight2,
  StandUp,
  WalkUp1,
  WalkUp2,
  StandDown,
  WalkDown1,
  WalkDown2,
  Dead,
  Attack,
  Victory
}

pub struct CharacterAnimation {
  sprite_folder: String,
  animation: Animation,
  remaining_frames: usize,
  sprite: Sprite,
  opacity: f32
}

impl CharacterAnimation {
  pub fn new(sprite_folder: String) -> Self {
    Self {
      sprite_folder: format!("characters/{}", sprite_folder),
      animation: Animation::Attack,
      remaining_frames: 0,
      sprite: Sprite::StandDown,
      opacity: 1.
    }
  }

  pub fn start_animation(&mut self, animation: Animation) {
    self.animation = animation;
    match self.animation {
      Animation::StartTurn      => self.remaining_frames = 12,
      Animation::EndTurn        => self.remaining_frames = 10,
      Animation::Attack         => self.remaining_frames = 40,
      Animation::Hurt(_, _)     => self.remaining_frames = 40,
      Animation::HurtSelf(_, _) => self.remaining_frames = 24,
      Animation::Dead           => self.remaining_frames = 20,
      Animation::Flee           => self.remaining_frames = 80,
      Animation::WalkTile(_)    => self.remaining_frames = 8,
      Animation::NpcWalkTile(_) => self.remaining_frames = 32
    };
  }

  pub fn advance_animation(&mut self) -> bool {
    self.remaining_frames -= 1;
    match self.animation {
      Animation::StartTurn => {
        self.advance_walk_animation(Direction::Right);
        return self.remaining_frames == 0;
      },
      Animation::EndTurn => {
        self.advance_walk_animation(Direction::Left);
        if self.remaining_frames == 0 {
          self.sprite = Sprite::StandRight;
          return true;
        }
        return false;
      },
      Animation::Attack => {
        if self.remaining_frames > 24 {
          self.advance_walk_animation(Direction::Right);
        } else if self.remaining_frames == 24 {
          self.sprite = Sprite::Attack;
        } else if self.remaining_frames == 0 {
          self.sprite = Sprite::StandRight;
          return true;
        }
        return false;
      },
      Animation::Hurt(_, _) | Animation::HurtSelf(_, _) => {
        if self.remaining_frames <= 24 {
          if self.remaining_frames % 8 == 0 {
            self.opacity = 1.;
          } else if self.remaining_frames % 4 == 0 {
            self.opacity = 0.;
          }
        }
        return self.remaining_frames == 0;
      },
      Animation::Dead => {
        self.sprite = Sprite::Dead;
        return self.remaining_frames == 0;
      },
      Animation::Flee => {
        self.advance_walk_animation(Direction::Left);
        return self.remaining_frames == 0;
      },
      Animation::WalkTile(direction) | Animation::NpcWalkTile(direction) => {
        match direction {
          Direction::Up    => self.advance_walk_animation(Direction::Up),
          Direction::Down  => self.advance_walk_animation(Direction::Down),
          Direction::Left  => self.advance_walk_animation(Direction::Left),
          Direction::Right => self.advance_walk_animation(Direction::Right)
        }
        return self.remaining_frames == 0;
      }
    }
  }

  pub fn turn_character(&mut self, direction: Direction) {
    match direction {
      Direction::Up    => self.sprite = Sprite::StandUp,
      Direction::Down  => self.sprite = Sprite::StandDown,
      Direction::Left  => self.sprite = Sprite::StandLeft,
      Direction::Right => self.sprite = Sprite::StandRight
    };
  }

  pub fn is_currently_animating(&self) -> bool {
    self.remaining_frames > 0
  }

  pub fn get_current_animation(&self) -> Animation {
    self.animation
  }

  pub fn get_frames_remaining(&self) -> usize {
    self.remaining_frames
  }

  fn advance_walk_animation(&mut self, direction: Direction) {
    let (stand, walk_1, walk_2) = match direction {
      Direction::Up    => (Sprite::StandUp,    Sprite::WalkUp1,    Sprite::WalkUp2),
      Direction::Down  => (Sprite::StandDown,  Sprite::WalkDown1,  Sprite::WalkDown2),
      Direction::Left  => (Sprite::StandLeft,  Sprite::WalkLeft1,  Sprite::WalkLeft2),
      Direction::Right => (Sprite::StandRight, Sprite::WalkRight1, Sprite::WalkRight2)
    };
    if self.remaining_frames % 8 == 0 {
      self.sprite = stand;
    } else if self.remaining_frames % 6 == 0 {
      self.sprite = walk_1;
    } else if self.remaining_frames % 4 == 0 {
      self.sprite = stand;
    } else if self.remaining_frames % 2 == 0 {
      self.sprite = walk_2;
    }
  }

  fn get_sprite_key(&self) -> String {
    match self.sprite {
      Sprite::StandLeft  => format!("{}/standing_left",      self.sprite_folder),
      Sprite::WalkLeft1  => format!("{}/walk_frame_1_left",  self.sprite_folder),
      Sprite::WalkLeft2  => format!("{}/walk_frame_2_left",  self.sprite_folder),
      Sprite::StandRight => format!("{}/standing_right",     self.sprite_folder),
      Sprite::WalkRight1 => format!("{}/walk_frame_1_right", self.sprite_folder),
      Sprite::WalkRight2 => format!("{}/walk_frame_2_right", self.sprite_folder),
      Sprite::StandUp    => format!("{}/standing_up",        self.sprite_folder),
      Sprite::WalkUp1    => format!("{}/walk_frame_1_up",    self.sprite_folder),
      Sprite::WalkUp2    => format!("{}/walk_frame_2_up",    self.sprite_folder),
      Sprite::StandDown  => format!("{}/standing_down",      self.sprite_folder),
      Sprite::WalkDown1  => format!("{}/walk_frame_1_down",  self.sprite_folder),
      Sprite::WalkDown2  => format!("{}/walk_frame_2_down",  self.sprite_folder),
      Sprite::Dead       => format!("{}/dead",               self.sprite_folder),
      Sprite::Attack     => format!("{}/attack",             self.sprite_folder),
      Sprite::Victory    => format!("{}/victory",            self.sprite_folder)
    }
  }

  pub fn draw(&self, program: &mut ShaderProgram, x: f32, y: f32) {
    program.draw(self.get_sprite_key(), x, y, 64., 64., self.opacity);
  }
}
