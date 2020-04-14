use crate::game::battle::character::Character;
use crate::game::battle::enemy::Enemy;
use crate::game::menu::click_event::{OnClickEvent, ClickEventReturnType, match_click_event};
use crate::game::menu::font::print_text;
use crate::game::menu::notification::Notification;
use crate::game::transition::Transition;
use crate::webgl::shader_program::ShaderProgram;

pub struct MenuItem {
  text: String,
  x: f32,
  y: f32,
  on_click: OnClickEvent
}

impl MenuItem {
  pub fn new(text: String, x: f32, y: f32, on_click: OnClickEvent) -> Self {
    Self {
      text,
      x,
      y,
      on_click
    }
  }

  pub fn click_item(
    &self,
    party: &mut Vec<Character>,
    enemies: &mut Vec<Vec<Enemy>>,
    transition: &mut Transition,
    notification: &mut Notification
  ) -> ClickEventReturnType {
    match_click_event(&self.on_click, party, enemies, transition, notification)
  }

  pub fn set_text(&mut self, new_text: String) {
    self.text = new_text;
  }

  pub fn get_text(&self) -> String {
    self.text.to_owned()
  }

  pub fn get_coords(&self) -> (f32, f32) {
    (self.x, self.y)
  }

  pub fn set_click_event(&mut self, new_event: OnClickEvent) {
    self.on_click = new_event;
  }

  pub fn draw(&self, program: &mut ShaderProgram) {
    print_text(program, self.text.to_owned(), self.x, self.y);
  }
}
