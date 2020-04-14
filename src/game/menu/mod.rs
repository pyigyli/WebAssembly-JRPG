pub mod click_event;
pub mod container;
pub mod font;
pub mod item;
pub mod notification;
pub mod textbox;

use click_event::{match_click_event, OnClickEvent, ClickEventReturnType};
use container::MenuContainer;
use item::MenuItem;
use notification::Notification;
use crate::game::battle::character::Character;
use crate::game::battle::enemy::Enemy;
use crate::game::transition::Transition;
use crate::webgl::audio::Audio;
use crate::webgl::keyboard::is_pressed;
use crate::webgl::shader_program::ShaderProgram;

pub enum MenuMovement {
  Grid, ColumnOfRows, RowOfColumns
}

pub type MenuMutation = for<'a, 'b, 'c> fn(&'a mut Audio, &'b mut MenuScreen, &'c mut Vec<Character>);

pub struct MenuScreen {
  containers: Vec<MenuContainer>,
  selectables: Vec<Vec<MenuItem>>,
  unselectables: Vec<MenuItem>,
  movement: MenuMovement,
  cursor_x: usize,
  cursor_y: usize,
  return_action: OnClickEvent,
  mutation: Option<MenuMutation>
}

impl MenuScreen {
  pub fn new(
    containers: Vec<MenuContainer>,
    selectables: Vec<Vec<MenuItem>>,
    unselectables: Vec<MenuItem>,
    movement: MenuMovement,
    cursor_x: usize,
    cursor_y: usize,
    return_action: OnClickEvent
  ) -> Self {
    Self {
      containers,
      selectables,
      unselectables,
      movement,
      cursor_x,
      cursor_y,
      return_action,
      mutation: None
    }
  }

  pub fn update(&mut self, audio: &mut Audio, party: &mut Vec<Character>, enemies: &mut Vec<Vec<Enemy>>, transition: &mut Transition, notification: &mut Notification) {
    if is_pressed("a") {
      if self.return_action.is_some() {
        audio.play_sfx("menu_click");
      }
      let click_event_return_type = self.selectables[self.cursor_y][self.cursor_x].click_item(party, enemies, transition, notification);
      self.match_click_event_return_type(click_event_return_type);
    } else if is_pressed("s") {
      self.perform_return_action(audio, party, enemies, transition, notification);
    } else if is_pressed("up") {
      self.move_cursor_up();
    } else if is_pressed("down") {
      self.move_cursor_down();
    } else if is_pressed("left") {
      self.move_cursor_left();
    } else if is_pressed("right") {
      self.move_cursor_right();
    }
    if let Some(mutation_function) = &mut self.mutation {
      mutation_function(audio, self, party);
    }
  }

  pub fn set_menu(&mut self, new_menu: MenuScreen) {
    *self = new_menu;
  }

  pub fn perform_return_action(
    &mut self,
    audio: &mut Audio,
    party: &mut Vec<Character>,
    enemies: &mut Vec<Vec<Enemy>>,
    transition: &mut Transition,
    notification: &mut Notification
  ) {
    if self.return_action.is_some() {
      audio.play_sfx("menu_click"); // Replace with return sound when available
    }
    let click_event_return_type = match_click_event(&self.return_action, party, enemies, transition, notification);
    self.match_click_event_return_type(click_event_return_type);
  }

  pub fn start_mutation(&mut self, mutation_function: MenuMutation) {
    self.mutation = Some(mutation_function);
  }

  pub fn end_mutation(&mut self) {
    self.mutation = None;
  }

  pub fn is_open(&self) -> bool {
    self.containers.len() > 0 || self.selectables.len() > 0 || self.unselectables.len() > 0
  }

  pub fn get_selectable(&mut self, outer_index: usize, inner_index: usize) -> &mut MenuItem {
    &mut self.selectables[outer_index][inner_index]
  }

  pub fn get_unselectable(&mut self, index: usize) -> &mut MenuItem {
    &mut self.unselectables[index]
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

  fn match_click_event_return_type(&mut self, click_event_return_type: ClickEventReturnType) {
    match click_event_return_type {
      ClickEventReturnType::NewMenu(new_menu)                => self.set_menu(new_menu),
      ClickEventReturnType::StartMutation(mutation_function) => self.start_mutation(mutation_function),
      ClickEventReturnType::None => ()
    }
  }

  fn move_cursor_up(&mut self) {
    match self.movement {
      MenuMovement::Grid => {
        if self.cursor_y > 0 && self.cursor_x < self.selectables[self.cursor_y - 1].len() {
          self.cursor_y -= 1;
        }
      },
      MenuMovement::RowOfColumns => self.cursor_x = self.cursor_x.checked_sub(1).unwrap_or(0),
      MenuMovement::ColumnOfRows => {
        if self.cursor_y > 0 {
          self.cursor_x = 0;
          self.cursor_y -= 1;
        }
      }
    }
  }

  fn move_cursor_down(&mut self) {
    match self.movement {
      MenuMovement::Grid => {
        if self.cursor_y + 1 < self.selectables.len() && self.cursor_x < self.selectables[self.cursor_y + 1].len() {
          self.cursor_y += 1;
        }
      },
      MenuMovement::RowOfColumns => self.cursor_x = (self.cursor_x + 1).min(self.selectables[self.cursor_y].len() - 1),
      MenuMovement::ColumnOfRows => {
        if self.cursor_y + 1 < self.selectables.len() {
          self.cursor_x = 0;
          self.cursor_y += 1;
        }
      }
    }
  }

  fn move_cursor_left(&mut self) {
    match self.movement {
      MenuMovement::Grid => self.cursor_x = self.cursor_x.checked_sub(1).unwrap_or(0),
      MenuMovement::RowOfColumns => {
        if self.cursor_y > 0 {
          self.cursor_x = 0;
          self.cursor_y -= 1;
        }
      },
      MenuMovement::ColumnOfRows => {
        if self.cursor_x > 0 {
          self.cursor_x -= 1;
          self.cursor_y = 0;
        }
      }
    }
  }

  fn move_cursor_right(&mut self) {
    match self.movement {
      MenuMovement::Grid => {
        if self.cursor_x + 1 < self.selectables[self.cursor_y].len() {
          self.cursor_x += 1;
        }
      },
      MenuMovement::RowOfColumns => {
        if self.cursor_y + 1 < self.selectables.len() {
          self.cursor_x = 0;
          self.cursor_y += 1;
        }
      },
      MenuMovement::ColumnOfRows => {
        if self.cursor_x + 1 < self.selectables[self.cursor_y].len() {
          self.cursor_x += 1;
          self.cursor_y = 0;
        }
      }
    }
  }
}
