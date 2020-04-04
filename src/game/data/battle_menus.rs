use crate::game::battle::ActionTuple;
use crate::game::battle::character::Character;
use crate::game::battle::enemy::Enemy;
use crate::game::menu::container::MenuContainer;
use crate::game::menu::item::{MenuItem, OnClickEvent};
use crate::game::menu::MenuScreen;

pub fn none_menu() -> MenuScreen {
  MenuScreen::new(Vec::new(), Vec::new(), Vec::new(), 0, 0, OnClickEvent::None)
}

pub fn main_battle_menu(character_in_turn: &Character) -> MenuScreen {
  let selectables = vec![
    vec![character_in_turn.get_attack_ability_as_menuitem()],
    vec![character_in_turn.get_primary_ability_as_menuitem()],
    vec![character_in_turn.get_secondary_ability_as_menuitem()],
    vec![MenuItem::new(String::from("Item"),   70., 564., OnClickEvent::None)],
    vec![MenuItem::new(String::from("Defend"), 70., 596., OnClickEvent::None)],
    vec![MenuItem::new(String::from("Row"),    70., 628., OnClickEvent::None)]
  ];
  MenuScreen::new(vec![MenuContainer::new(16., 420., 250., 704.)], selectables, Vec::new(), 0, 0, OnClickEvent::None)
}

pub fn single_target_targeting_everyone(party: &Vec<Character>, enemies: &mut Vec<Vec<Enemy>>, action_effects: ActionTuple) -> MenuScreen {
  let mut selectables = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
  push_party_to_selectables(&mut selectables, party, action_effects);
  push_enemies_to_selectables(&mut selectables, enemies, action_effects);
  MenuScreen::new(vec![MenuContainer::new(16., 420., 250., 704.)], selectables, Vec::new(), 0, 0, OnClickEvent::SetBattleMenu(main_battle_menu))
}

fn push_party_to_selectables(selectables: &mut Vec<Vec<MenuItem>>, party: &Vec<Character>, action_effects: ActionTuple) {
  let perform_battle_action = |party: &mut Vec<Character>, _enemies: &mut Vec<Vec<Enemy>>, target_ids: Vec<usize>, action_effects: ActionTuple| {
    let acting_character = party.iter_mut().find(|character: &&mut Character| character.get_battle_state().is_turn_active()).unwrap();
    acting_character.perform_battle_action();

    let incoming_damage = action_effects.0(acting_character.get_battle_state_mut());
    for character in party.iter_mut() {
      if target_ids.contains(&character.get_id()) {
        character.receive_battle_action(action_effects.1, incoming_damage);
      }
    }
  };
  for (index, character) in party.iter().enumerate() {
    let (x, y) = character.get_coords();
    selectables[index].push(MenuItem::new(String::new(), x, y, OnClickEvent::BattleAction(perform_battle_action, vec![character.get_id()], action_effects)));
  }
}

fn push_enemies_to_selectables(selectables: &mut Vec<Vec<MenuItem>>, enemies: &Vec<Vec<Enemy>>, action_effects: ActionTuple) {
  let perform_battle_action = |party: &mut Vec<Character>, enemies: &mut Vec<Vec<Enemy>>, target_ids: Vec<usize>, action_effects: ActionTuple| {
    let acting_character = party.iter_mut().find(|character: &&mut Character| character.get_battle_state().is_turn_active()).unwrap();
    acting_character.perform_battle_action();

    for enemy in enemies.iter_mut().flatten() {
      if target_ids.contains(&enemy.get_id()) {
        enemy.receive_battle_action(action_effects.1, action_effects.0(acting_character.get_battle_state_mut()));
      }
    }
  };
  for (j, enemy_row) in enemies.iter().enumerate() {
    for (i, enemy) in enemy_row.iter().enumerate() {
      selectables[j].push(MenuItem::new(
        String::new(),
        700. + i as f32 * 100.,
        180. + j as f32 * 100.,
        OnClickEvent::BattleAction(perform_battle_action, vec![enemy.get_id()], action_effects)
      ));
    }
  }
}
