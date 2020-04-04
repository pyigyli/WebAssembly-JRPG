pub mod character;
pub mod enemy;
pub mod state;
pub mod print_damage;

use character::Character;
use enemy::{BattleScript, Enemy};
use print_damage::PrintDamage;
use state::BattleState;

use crate::game::data::battle_menus;
use crate::game::menu::container::MenuContainer;
use crate::game::menu::MenuScreen;
use crate::game::menu::notification::Notification;
use crate::game::transition::{Transition, TransitionStyle};
use crate::webgl::audio::Audio;
use crate::webgl::shader_program::ShaderProgram;

pub type ActionTuple = (for<'a, 'b> fn(&'a mut BattleState, &'b mut Notification) -> f64, for<'a> fn(&'a mut BattleState, f64));

pub struct Battle {
  soundtrack_file: String,
  battle_menu: MenuScreen,
  enemies: Vec<Vec<Enemy>>,
  active_turns: Vec<usize>,
  current_turn: usize, // 0 = Noone's turn, 1-4 party member's turn, 5 >= enemy's turn
  experience_gained: u32,
  fighting: bool,
  print_damage: PrintDamage
}

impl Battle {
  pub fn new() -> Self {
    Self {
      soundtrack_file: String::from("battle_test_theme"),
      battle_menu: battle_menus::none_menu(),
      enemies: vec![Vec::new()],
      active_turns: Vec::new(),
      current_turn: 0, // 0 = Noone's turn, 1-4 party member's turn, 5 >= enemy's turn
      experience_gained: 0,
      fighting: false,
      print_damage: PrintDamage::new()
    }
  }

  pub fn update(&mut self, audio: &mut Audio, party: &mut Vec<Character>, transition: &mut Transition, notification: &mut Notification) {
    audio.update(&self.soundtrack_file);
    self.start_turn(party);
    if self.battle_menu.is_open() {
      self.battle_menu.update(party, &mut self.enemies, transition, notification);
    } else {
      for character in party.iter_mut() {
        if character.is_atb_full() {
          self.active_turns.push(character.get_id());
        }
      }
      for enemy in self.enemies.iter_mut().flatten() {
        if enemy.is_atb_full() {
          self.active_turns.push(enemy.get_id());
        }
      }
    }
    for character in party.iter_mut() {
      let turn_done = character.update(audio, &mut self.battle_menu, &mut self.print_damage);
      if turn_done {
        self.current_turn = 0;
      }
    }
    let possible_battle_script = self.handle_enemy_updates(audio);
    if let Some(battle_script) = possible_battle_script {
      battle_script(party, &mut self.enemies, notification);
    }
    self.print_damage.update();
  }

  pub fn handle_enemy_updates(&mut self, audio: &mut Audio) -> Option<BattleScript> {
    for (j, enemy_row) in self.enemies.iter_mut().enumerate() {
      for (i, enemy) in enemy_row.iter_mut().enumerate() {
        let turn_progression_value = enemy.update(audio, 700. + i as f32 * 100., 180. + j as f32 * 100., &mut self.print_damage);
        match turn_progression_value {
          1 => return Some(enemy.get_battle_script()),
          2 => self.current_turn = 0,
          _ => ()
        }
      }
    }
    None
  }

  pub fn start_battle(&mut self, party: &mut Vec<Character>, enemies: Vec<Vec<Enemy>>, transition: &mut Transition) {
    for character in party.iter_mut() {
      character.start_battle(false);
    }
    self.enemies = enemies;
    transition.set(TransitionStyle::BattleIn);
  }

  pub fn start_turn(&mut self, party: &mut Vec<Character>) {
    if self.active_turns.len() > 0 && self.current_turn == 0 {
      self.current_turn = self.active_turns.remove(0);

      if self.current_turn > 0 && self.current_turn < 5 {
        party.iter_mut().find(|character: &&mut Character| self.current_turn == character.get_id()).unwrap().start_turn();

      } else if self.current_turn > 4 {
        for enemy in self.enemies.iter_mut().flatten() {
          if enemy.get_id() == self.current_turn {
            enemy.start_turn();
          }
        }
      }
    }
  }

  pub fn set_in_battle(&mut self) {
    self.fighting = true;
  }

  pub fn in_battle(&self) -> bool {
    self.fighting
  }

  pub fn get_enemies(&mut self) -> &mut Vec<Vec<Enemy>> {
    &mut self.enemies
  }

  pub fn draw(&mut self, program: &mut ShaderProgram, party: &Vec<Character>) {
    MenuContainer::new(300., 420., 1064., 704.).draw(program);
    if self.battle_menu.is_open() {
      self.battle_menu.draw(program);
    }
    for character in party.iter() {
      character.draw(program);
      character.draw_battle_info(program);
    }
    for (j, enemy_row) in self.enemies.iter().enumerate() {
      for (i, enemy) in enemy_row.iter().enumerate() {
        enemy.draw(program, 700. + i as f32 * 100., 180. + j as f32 * 100.);
      }
    }
    self.print_damage.draw(program);
  }
}
