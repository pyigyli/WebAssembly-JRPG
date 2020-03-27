use crate::game::battle::character::Character;
use crate::game::battle::enemy::Enemy;
use crate::game::battle::state::BattleState;
use crate::game::data::battle_menus;
use crate::game::menu::font::print_text;
use crate::game::menu::MenuScreen;
use crate::game::transition::Transition;
use crate::webgl::shader_program::ShaderProgram;

type ActionTuple = (for<'a> fn(&'a mut BattleState) -> f64, for<'a> fn(&'a mut BattleState, f64));

#[derive(Clone)]
pub enum OnClickEvent {
  MenuTransition(for<'a> fn(&'a mut Transition)),
  SetBattleMenu(for<'a> fn(&'a Character) -> MenuScreen),
  ToTargetSelection(for<'a, 'b> fn(&'a Vec<Character>, &'b mut Vec<Vec<Enemy>>, ActionTuple) -> MenuScreen, ActionTuple),
  BattleAction(for<'a, 'b> fn(&'a mut Vec<Character>, &'b mut Vec<Vec<Enemy>>, Vec<usize>, ActionTuple), Vec<usize>, ActionTuple),
  None
}

pub fn match_click_event(event: &OnClickEvent, party: &mut Vec<Character>, enemies: &mut Vec<Vec<Enemy>>, transition: &mut Transition) -> Option<MenuScreen> {
  match event {
    OnClickEvent::MenuTransition(to_new_menu) => to_new_menu(transition),
    OnClickEvent::SetBattleMenu(new_battle_menu) => {
      return Some(new_battle_menu(party.iter().find(|character: &&Character| character.get_battle_state().is_turn_active()).unwrap()))
    },
    OnClickEvent::ToTargetSelection(to_target_selection, action_effects) => return Some(to_target_selection(party, enemies, *action_effects)),
    OnClickEvent::BattleAction(action, target_ids, action_effects) => {
      action(party, enemies, target_ids.to_vec(), *action_effects);
      return Some(battle_menus::none_menu());
    },
    OnClickEvent::None => ()
  };
  None
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

  pub fn click_item(&self, party: &mut Vec<Character>, enemies: &mut Vec<Vec<Enemy>>, transition: &mut Transition) -> Option<MenuScreen> {
    match_click_event(&self.on_click, party, enemies, transition)
  }

  pub fn get_coords(&self) -> (f32, f32) {
    (self.x, self.y)
  }

  pub fn draw(&self, program: &mut ShaderProgram) {
    print_text(program, self.text.to_owned(), self.x, self.y);
  }
}
