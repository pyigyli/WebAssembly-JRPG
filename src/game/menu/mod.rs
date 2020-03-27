pub mod container;
pub mod font;
pub mod item;
pub mod notification;
pub mod textbox;

use container::MenuContainer;
use item::{match_click_event, MenuItem, OnClickEvent};
use crate::game::battle::character::Character;
use crate::game::battle::enemy::Enemy;
use crate::game::transition::Transition;
use crate::webgl::keyboard::is_pressed;
use crate::webgl::shader_program::ShaderProgram;

pub struct MenuScreen {
  containers: Vec<MenuContainer>,
  selectables: Vec<Vec<MenuItem>>,
  unselectables: Vec<MenuItem>,
  cursor_x: usize,
  cursor_y: usize,
  return_action: OnClickEvent
}

impl MenuScreen {
  pub fn new(
    containers: Vec<MenuContainer>,
    selectables: Vec<Vec<MenuItem>>,
    unselectables: Vec<MenuItem>,
    cursor_x: usize,
    cursor_y: usize,
    return_action: OnClickEvent
  ) -> Self {
    Self {
      containers,
      selectables,
      unselectables,
      cursor_x,
      cursor_y,
      return_action
    }
  }

  pub fn update(&mut self, party: &mut Vec<Character>, enemies: &mut Vec<Vec<Enemy>>, transition: &mut Transition) {
    if is_pressed("a") {
      let possible_new_menu = self.selectables[self.cursor_y][self.cursor_x].click_item(party, enemies, transition);
      if let Some(new_menu) = possible_new_menu {
        self.set_menu(new_menu);
      }
    } else if is_pressed("s") {
      self.perform_return_action(party, enemies, transition);

    } else if is_pressed("up") {
      self.cursor_y = self.cursor_y.checked_sub(1).unwrap_or(0);

    } else if is_pressed("down") && self.cursor_y + 1 < self.selectables.len() && self.cursor_x < self.selectables[self.cursor_y + 1].len() {
      self.cursor_y = (self.cursor_y + 1).min(self.selectables.len() - 1);

    } else if is_pressed("left") {
      self.cursor_x = self.cursor_x.checked_sub(1).unwrap_or(0);

    } else if is_pressed("right") {
      self.cursor_x = (self.cursor_x + 1).min(self.selectables[self.cursor_y].len() - 1);
    }
  }

  pub fn set_menu(&mut self, new_menu: MenuScreen) {
    *self = new_menu;
  }

  pub fn perform_return_action(&mut self, party: &mut Vec<Character>, enemies: &mut Vec<Vec<Enemy>>, transition: &mut Transition) {
    let possible_new_menu = match_click_event(&self.return_action, party, enemies, transition);
    if let Some(new_menu) = possible_new_menu {
      self.set_menu(new_menu);
    }
  }

  pub fn is_open(&self) -> bool {
    self.containers.len() > 0
  }

  pub fn draw(&self, program: &mut ShaderProgram) {
    for container in self.containers.iter() {
      container.draw(program);
    }
    for item in self.selectables.iter().flatten() {
      item.draw(program);
    }
    for item in self.unselectables.iter() {
      item.draw(program);
    }
    if self.selectables.len() > 0 {
      let (x, y) = self.selectables.get(self.cursor_y).unwrap().get(self.cursor_x).unwrap().get_coords();
      program.draw(String::from("cursor"), x - 40., y, 48., 48., 1.);
    }
  }
}
