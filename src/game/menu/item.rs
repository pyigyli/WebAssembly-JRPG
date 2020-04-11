use crate::game::battle::ActionTuple;
use crate::game::battle::character::Character;
use crate::game::battle::enemy::Enemy;
use crate::game::data::battle_menus;
use crate::game::menu::font::print_text;
use crate::game::menu::{MenuScreen, MenuMutation};
use crate::game::menu::notification::Notification;
use crate::game::transition::Transition;
use crate::webgl::shader_program::ShaderProgram;

pub enum OnClickEvent {
  MenuTransition(for<'a> fn(&'a mut Transition)),
  MutateMenu(MenuMutation),
  SetBattleMenu(for<'a> fn(&'a Character) -> MenuScreen),
  ToTargetSelection(for<'r, 's> fn(&'r Vec<Character>, &'s mut Vec<Vec<Enemy>>, ActionTuple) -> MenuScreen, ActionTuple),
  BattleAction(for<'a, 'b, 'c> fn(&'a mut Vec<Character>, &'b mut Vec<Vec<Enemy>>, Vec<usize>, ActionTuple, &'c mut Notification), Vec<usize>, ActionTuple),
  ChangeScene(for<'a> fn(&'a mut Transition)),
  None
}

pub enum ClickEventReturnType {
  NewMenu(MenuScreen),
  StartMutation(MenuMutation),
  None
}

pub fn match_click_event(
  event: &OnClickEvent,
  party: &mut Vec<Character>,
  enemies: &mut Vec<Vec<Enemy>>,
  transition: &mut Transition,
  notification: &mut Notification
) -> ClickEventReturnType {
  match event {
    OnClickEvent::MenuTransition(to_new_menu) => to_new_menu(transition),
    OnClickEvent::MutateMenu(mutation_function) => return ClickEventReturnType::StartMutation(*mutation_function),
    OnClickEvent::SetBattleMenu(new_battle_menu) => {
      return ClickEventReturnType::NewMenu(new_battle_menu(party.iter().find(|character: &&Character| character.get_battle_state().is_turn_active()).unwrap()))
    },
    OnClickEvent::ToTargetSelection(to_target_selection, action_effects) => return ClickEventReturnType::NewMenu(to_target_selection(party, enemies, *action_effects)),
    OnClickEvent::BattleAction(action, target_ids, action_effects) => {
      action(party, enemies, target_ids.to_vec(), *action_effects, notification);
      return ClickEventReturnType::NewMenu(battle_menus::none_menu());
    },
    OnClickEvent::ChangeScene(to_new_map) => to_new_map(transition),
    OnClickEvent::None => ()
  };
  ClickEventReturnType::None
}

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
