use crate::game::animation::Animation;
use crate::game::animation::enemy::EnemyAnimation;
use crate::game::battle::character::Character;
use crate::game::battle::print_damage::PrintDamage;
use crate::game::battle::state::BattleState;
use crate::game::menu::notification::Notification;
use crate::webgl::audio::Audio;
use crate::webgl::shader_program::ShaderProgram;

pub type BattleScript = for<'a, 'b, 'c> fn(&'a mut Vec<Character>, &'b mut Vec<Vec<Enemy>>, &'c mut Notification);

pub struct Enemy {
  animation: EnemyAnimation,
  name: String,
  id: usize,
  state: BattleState,
  battle_script: BattleScript
}

impl Enemy {
  pub fn new(
    sprite_key: String,
    name: String,
    id: usize,
    level: u16,
    hp:  u16,
    mp:  u16,
    att: f64,
    def: f64,
    mag: f64,
    int: f64,
    res: f64,
    agi: f64,
    battle_script: BattleScript
  ) -> Self {
    Self {
      animation: EnemyAnimation::new(sprite_key),
      name,
      id,
      state: BattleState::new(level, hp, 1., mp, 1., att, 1., def, 1., mag, 1., int, 1., res, 1., agi, 1.),
      battle_script
    }
  }

  pub fn update(&mut self, audio: &mut Audio, x: f32, y: f32, print_damage: &mut PrintDamage) -> u8 {
    self.state.update();
    if self.animation.is_currently_animating() {
      let animation_done = self.animation.advance_animation();
      if animation_done {
        match self.animation.get_current_animation() {
          Animation::Attack | Animation::HurtSelf(_, _) => {
            self.get_battle_state_mut().end_turn();
            return 2;
          },
          _ => ()
        };
      } else {
        match self.animation.get_current_animation() {
          Animation::Attack => {
            if self.animation.get_frames_remaining() == 24 {
              return 1;
            }
          },
          Animation::Hurt(action, incoming_damage) | Animation::HurtSelf(action, incoming_damage) => {
            if self.animation.get_frames_remaining() == 20 {
              action(self.get_battle_state_mut(), incoming_damage);
              print_damage.set(incoming_damage, x - 16., y + 32., [1.; 3]);
              audio.play_sfx("physical_hit");
            }
          },
          _ => ()
        };
      }
    }
    0
  }

  pub fn start_turn(&mut self) {
    self.get_battle_state_mut().start_turn();
    self.animation.start_animation(Animation::Attack);
  }

  pub fn receive_battle_action(&mut self, action: for<'a> fn(&'a mut BattleState, f64), incoming_damage: f64) {
    if self.get_battle_state().is_turn_active() {
      self.animation.start_animation(Animation::HurtSelf(action, incoming_damage));
    } else {
      self.animation.start_animation(Animation::Hurt(action, incoming_damage));
    }
  }

  pub fn is_atb_full(&self) -> bool {
    self.state.is_atb_full()
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn get_battle_state(&self) -> &BattleState {
    &self.state
  }

  pub fn get_battle_state_mut(&mut self) -> &mut BattleState {
    &mut self.state
  }

  pub fn get_battle_script(&self) -> BattleScript {
    self.battle_script
  }
  
  pub fn draw(&self, program: &mut ShaderProgram, x: f32, y: f32) {
    self.animation.draw(program, x, y);
  }
}
