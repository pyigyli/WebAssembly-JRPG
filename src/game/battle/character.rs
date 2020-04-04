use crate::game::battle::enemy::Enemy;
use crate::game::animation::{Animation, Direction};
use crate::game::animation::character::CharacterAnimation;
use crate::game::battle::ActionTuple;
use crate::game::battle::print_damage::PrintDamage;
use crate::game::battle::state::BattleState;
use crate::game::data::battle_menus;
use crate::game::menu::item::{MenuItem, OnClickEvent};
use crate::game::menu::MenuScreen;
use crate::webgl::shader_program::ShaderProgram;

pub struct Character {
  animation: CharacterAnimation,
  name: String,
  id: usize,
  x: f32,
  y: f32,
  state: BattleState,
  attack_ability: (String, for<'r, 's> fn(&'r Vec<Character>, &'s mut Vec<Vec<Enemy>>, ActionTuple) -> MenuScreen, ActionTuple),
  primary_ability: (String, for<'r, 's> fn(&'r Vec<Character>, &'s mut Vec<Vec<Enemy>>, ActionTuple) -> MenuScreen, ActionTuple),
  secondary_ability: (String, for<'r, 's> fn(&'r Vec<Character>, &'s mut Vec<Vec<Enemy>>, ActionTuple) -> MenuScreen, ActionTuple)
}

impl Character {
  pub fn new(
    name: String,
    sprite_folder: String,
    id: usize, 
    level: u16,
    hp:  u16, hp_growth_rate:  f32,
    mp:  u16, mp_growth_rate:  f32,
    att: f64, att_growth_rate: f32,
    def: f64, def_growth_rate: f32,
    mag: f64, mag_growth_rate: f32,
    int: f64, int_growth_rate: f32,
    res: f64, res_growth_rate: f32,
    agi: f64, agi_growth_rate: f32,
    attack_ability: (String, for<'r, 's> fn(&'r Vec<Character>, &'s mut Vec<Vec<Enemy>>, ActionTuple) -> MenuScreen, ActionTuple),
    primary_ability: (String, for<'r, 's> fn(&'r Vec<Character>, &'s mut Vec<Vec<Enemy>>, ActionTuple) -> MenuScreen, ActionTuple),
    secondary_ability: (String, for<'r, 's> fn(&'r Vec<Character>, &'s mut Vec<Vec<Enemy>>, ActionTuple) -> MenuScreen, ActionTuple)
  ) -> Self {
    Self {
      animation: CharacterAnimation::new(sprite_folder),
      name,
      id,
      x: 0.,
      y: 0.,
      state: BattleState::new(
        level,
        hp,  hp_growth_rate,
        mp,  mp_growth_rate,
        att, att_growth_rate,
        def, def_growth_rate,
        mag, mag_growth_rate,
        int, int_growth_rate,
        res, res_growth_rate,
        agi, agi_growth_rate
      ),
      attack_ability,
      primary_ability,
      secondary_ability
    }
  }

  pub fn update(&mut self, battle_menu: &mut MenuScreen, print_damage: &mut PrintDamage) -> bool {
    self.state.update();
    if self.animation.is_currently_animating() {
      let animation_done = self.animation.advance_animation();
      if animation_done {
        match self.animation.get_current_animation() {
          Animation::StartTurn => battle_menu.set_menu(battle_menus::main_battle_menu(&self)),
          Animation::EndTurn => {
            self.get_battle_state_mut().end_turn();
            return true;
          },
          Animation::Attack | Animation::HurtSelf(_, _) => self.animation.start_animation(Animation::EndTurn),
          _ => ()
        }
      } else {
        match self.animation.get_current_animation() {
          Animation::StartTurn => self.x += 5.,
          Animation::EndTurn   => self.x -= 14.,
          Animation::Attack => {
            if self.animation.get_frames_remaining() > 24 {
              self.x += 5.;
            }
          },
          Animation::Hurt(action, incoming_damage) | Animation::HurtSelf(action, incoming_damage) => {
            if self.animation.get_frames_remaining() == 20 {
              action(self.get_battle_state_mut(), incoming_damage);
              print_damage.set(incoming_damage, self.x + 32., self.y + 32., [1.; 3]);
            }
          },
          _ => ()
        };
      }
    }
    false
  }

  pub fn start_battle(&mut self, surprise: bool) {
    self.x = 100.;
    self.y = self.id as f32 * 80.;
    if surprise {
      self.animation.turn_character(Direction::Left);
    } else {
      self.animation.turn_character(Direction::Right);
    }
  }

  pub fn start_turn(&mut self) {
    self.get_battle_state_mut().start_turn();
    self.animation.start_animation(Animation::StartTurn);
  }

  pub fn perform_battle_action(&mut self) {
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

  pub fn get_coords(&self) -> (f32, f32) {
    (self.x, self.y)
  }

  pub fn get_battle_state(&self) -> &BattleState {
    &self.state
  }

  pub fn get_battle_state_mut(&mut self) -> &mut BattleState {
    &mut self.state
  }

  pub fn get_attack_ability_as_menuitem(&self) -> MenuItem {
    let on_click = OnClickEvent::ToTargetSelection(self.attack_ability.1, self.attack_ability.2);
    MenuItem::new(self.attack_ability.0.to_owned(), 70., 468., on_click)
  }

  pub fn get_primary_ability_as_menuitem(&self) -> MenuItem {
    MenuItem::new(self.primary_ability.0.to_owned(), 70., 500., OnClickEvent::None)
  }

  pub fn get_secondary_ability_as_menuitem(&self) -> MenuItem {
    MenuItem::new(self.secondary_ability.0.to_owned(), 70., 532., OnClickEvent::None)
  }

  pub fn draw(&self, program: &mut ShaderProgram) {
    self.animation.draw(program, self.x, self.y);
  }

  pub fn draw_battle_info(&self, program: &mut ShaderProgram) {
    self.state.draw(program, &self.name, self.id);
  }
}
