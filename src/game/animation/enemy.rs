use crate::game::animation::Animation;
use crate::webgl::shader_program::ShaderProgram;

pub struct EnemyAnimation {
  sprite_key: String,
  animation: Animation,
  remaining_frames: usize,
  x_offset: f32,
  opacity: f32
}

impl EnemyAnimation {
  pub fn new(sprite_key: String) -> Self {
    Self {
      sprite_key: format!("enemies/{}", sprite_key),
      animation: Animation::Attack,
      remaining_frames: 0,
      x_offset: 0.,
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
      _ => ()
    };
  }

  pub fn advance_animation(&mut self) -> bool {
    self.remaining_frames -= 1;
    match self.animation {
      Animation::StartTurn => return self.remaining_frames == 0,
      Animation::EndTurn => return self.remaining_frames == 0,
      Animation::Attack => {
        if self.remaining_frames > 36 {
          self.x_offset -= 2.;
        } else if self.remaining_frames > 28 {
          self.x_offset += 2.;
        } else if self.remaining_frames > 20 {
          self.x_offset -= 2.;
        } else if self.remaining_frames > 16 {
          self.x_offset += 2.;
        }
        return self.remaining_frames == 0;
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
        self.opacity -= 0.05;
        return self.remaining_frames == 0;
      },
      Animation::Flee => return self.remaining_frames == 0,
      _ => true
    }
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

  pub fn draw(&self, program: &mut ShaderProgram, x: f32, y: f32) {
    program.draw(self.sprite_key.to_owned(), x + self.x_offset, y, 64., 64., self.opacity);
  }
}
