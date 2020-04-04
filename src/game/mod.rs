mod animation;
mod battle;
mod data;
mod map;
mod menu;
mod transition;

use battle::Battle;
use battle::character::Character;
use map::Map;
use map::player::Player;
use menu::MenuScreen;
use menu::notification::Notification;
use menu::textbox::Textbox;
use transition::{Transition, TransitionStyle};
use crate::webgl::audio::Audio;
use crate::webgl::keyboard::is_down;
use crate::webgl::shader_program::ShaderProgram;

pub struct GameState {
  menu: MenuScreen,
  map: Map,
  player: Player,
  party: Vec<Character>,
  reserves: Vec<Character>,
  battle: Battle,
  notification: Notification,
  textbox: Textbox,
  transition: Transition
}

impl GameState {
  pub fn new() -> Self {
    let mut player = Player::new();
    player.set_character_sprites(String::from("Darrel_Deen"));
    Self {
      menu: data::menus::title_menu(),
      map: data::maps::none_map(&mut player),
      player,
      party: vec![data::characters::darrel_deen(1), data::characters::nurse_seraphine(2), data::characters::darrel_deen(3), data::characters::nurse_seraphine(4)],
      reserves: Vec::new(),
      battle: Battle::new(),
      notification: Notification::new(),
      textbox: Textbox::new(),
      transition: Transition::new()
    }
  }

  pub fn update(&mut self, audio: &mut Audio) {
    if self.transition.is_transitioning() {
      self.transition.update(&mut self.map, &mut self.player, &mut self.battle, &mut self.menu);

    } else if self.menu.is_open() {
      self.menu.update(&mut self.party, self.battle.get_enemies(), &mut self.transition);

    } else if self.battle.in_battle() {
      self.battle.update(&mut self.party, &mut self.transition);

    } else if is_down("f") {
      self.transition.set(TransitionStyle::MenuIn(data::menus::main_menu));

    } else if self.textbox.is_open() {
      self.textbox.update();

    } else {
      self.player.update(&mut self.map, &mut self.party, &mut self.battle, &mut self.transition, &mut self.textbox);
      self.map.update();
    }
    self.notification.update();
  }

  pub fn draw(&mut self, program: &mut ShaderProgram) {
    if self.menu.is_open() {
      self.menu.draw(program);

    } else if self.battle.in_battle() {
      self.battle.draw(program, &self.party);
      
    } else {
      self.map.draw(program, self.player.get_coords());
      self.player.draw(program);
    }
    if self.textbox.is_open() {
      self.textbox.draw(program);
    }
    self.notification.draw(program);

    if self.transition.is_transitioning() {
      self.transition.draw(program);
    }
  }
}
