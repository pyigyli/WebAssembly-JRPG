use crate::game::battle::Battle;
use crate::game::battle::character::Character;
use crate::game::data::menus;
use crate::game::map::Map;
use crate::game::map::player::Player;
use crate::game::menu::MenuScreen;
use crate::webgl::shader_program::ShaderProgram;

pub enum TransitionStyle {
  None,
  WhiteIn,
  BlackIn,
  BattleIn,
  BattleOut,
  BattleEndScreen(for<'a> fn(&'a mut Vec<Character>, u32) -> MenuScreen, u32),
  MenuIn(for<'a> fn(&'a mut Vec<Character>) -> MenuScreen),
  ChangeScene(for<'a> fn(&'a mut Player) -> Map),
  WhiteOut,
  BlackOut
}

pub struct Transition {
  style: TransitionStyle,
  opacity: f32
}

impl Transition {
  pub fn new() -> Self {
    Self {
      style: TransitionStyle::None,
      opacity: 0.
    }
  }

  pub fn set(&mut self, transition: TransitionStyle) {
    self.style = transition;
  }

  pub fn update(&mut self, map: &mut Map, player: &mut Player, party: &mut Vec<Character>, battle: &mut Battle, menu: &mut MenuScreen) {
    match self.style {
      TransitionStyle::None => (),
      TransitionStyle::WhiteIn | TransitionStyle::BlackIn => {
        self.opacity = ((self.opacity + 0.3) * 0.9).min(1.);
        if self.opacity == 1. {
          match self.style {
            TransitionStyle::WhiteIn => self.set(TransitionStyle::WhiteOut),
            TransitionStyle::BlackIn => self.set(TransitionStyle::BlackOut),
            _ => ()
          };
        }
      },
      TransitionStyle::BattleIn => {
        self.opacity = ((self.opacity + 0.3) * 0.9).min(1.);
        if self.opacity == 1. {
          battle.set_fighting_state(true);
          self.set(TransitionStyle::BlackOut);
        }
      },
      TransitionStyle::BattleOut => {
        self.opacity = ((self.opacity + 0.3) * 0.9).min(1.);
        if self.opacity == 1. {
          menu.set_menu(menus::none_menu(party));
          battle.set_fighting_state(false);
          self.set(TransitionStyle::BlackOut);
        }
      },
      TransitionStyle::BattleEndScreen(get_new_menu_function, experience) => {
        self.opacity = ((self.opacity + 0.3) * 0.9).min(1.);
        if self.opacity == 1. {
          menu.set_menu(get_new_menu_function(party, experience));
          self.set(TransitionStyle::BlackOut);
        }
      },
      TransitionStyle::MenuIn(get_new_menu_function) => {
        self.opacity = ((self.opacity + 0.3) * 0.9).min(1.);
        if self.opacity == 1. {
          menu.set_menu(get_new_menu_function(party));
          self.set(TransitionStyle::BlackOut);
        }
      },
      TransitionStyle::ChangeScene(get_new_map_function) => {
        self.opacity = ((self.opacity + 0.3) * 0.9).min(1.);
        if self.opacity == 1. {
          menu.set_menu(menus::none_menu(party));
          map.set_map(get_new_map_function(player));
          self.set(TransitionStyle::BlackOut);
        }
      },
      TransitionStyle::WhiteOut | TransitionStyle::BlackOut => {
        self.opacity = ((self.opacity - 0.25) * 1.1).max(0.);
        if self.opacity == 0. {
          self.set(TransitionStyle::None);
        }
      }
    }
  }

  pub fn is_transitioning(&self) -> bool {
    match self.style {
      TransitionStyle::None => false,
      _ => true
    }
  }

  pub fn draw(&self, program: &mut ShaderProgram) {
    let sprite_key = match self.style {
      TransitionStyle::WhiteIn | TransitionStyle::WhiteOut => String::from("white"),
      _ => String::from("black")
    };
    program.draw(sprite_key.to_owned(), 0., 0., 1080., 720., self.opacity);
  }
}
