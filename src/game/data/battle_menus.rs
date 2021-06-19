use crate::game::battle::ActionTuple;
use crate::game::battle::BattleActionTargetStart;
use crate::game::battle::character::Character;
use crate::game::battle::enemy::Enemy;
use crate::game::menu::click_event::OnClickEvent;
use crate::game::menu::container::MenuContainer;
use crate::game::menu::item::MenuItem;
use crate::game::menu::{MenuMovement, MenuScreen};
use crate::game::menu::notification::Notification;

pub fn none_menu() -> MenuScreen {
  MenuScreen::new(Vec::new(), Vec::new(), Vec::new(), MenuMovement::Grid, 0, 0, OnClickEvent::None)
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
  MenuScreen::new(vec![MenuContainer::new(16., 420., 250., 704.)], selectables, Vec::new(), MenuMovement::Grid, 0, 0, OnClickEvent::None)
}

pub fn single_target_targeting_everyone(
  party: &Vec<Character>,
  enemies: &mut Vec<Vec<Enemy>>,
  action_effects: ActionTuple,
  targeting_start_type: BattleActionTargetStart
) -> MenuScreen {
  let mut selectables = Vec::new();
  push_party_to_selectables(&mut selectables, party, action_effects);
  push_enemies_to_selectables(&mut selectables, enemies, action_effects);
  let (cursor_x_pos, cursor_y_pos) = match targeting_start_type {
    BattleActionTargetStart::Enemies => (0, 1),
    BattleActionTargetStart::Party => (0, 0),
    BattleActionTargetStart::Myself => (get_character_in_turn!(party).unwrap().get_id(), 0)
  };
  MenuScreen::new(
    vec![MenuContainer::new(16., 420., 250., 704.)],
    selectables, Vec::new(),
    MenuMovement::RowOfColumns,
    cursor_x_pos,
    cursor_y_pos,
    OnClickEvent::SetBattleMenu(main_battle_menu)
  )
}

fn push_party_to_selectables(selectables: &mut Vec<Vec<MenuItem>>, party: &Vec<Character>, action_effects: ActionTuple) {
  let perform_battle_action = |
    party: &mut Vec<Character>,
    _enemies: &mut Vec<Vec<Enemy>>,
    target_ids: Vec<usize>,
    action_effects: ActionTuple,
    notification: &mut Notification
  | {
    let acting_character = party.iter_mut().find(|character: &&mut Character| character.get_battle_state().is_turn_active()).unwrap();
    acting_character.perform_battle_action();

    let incoming_damage = action_effects.0(acting_character.get_battle_state_mut(), notification);
    for character in party.iter_mut() {
      if target_ids.contains(&character.get_id()) {
        character.receive_battle_action(action_effects.1, incoming_damage);
      }
    }
  };

  selectables.push(Vec::new());
  for character in party.iter() {
    let (x, y) = character.get_coords();
    selectables[0].push(MenuItem::new(String::new(), x, y, OnClickEvent::BattleAction(perform_battle_action, vec![character.get_id()], action_effects)));
  }
}

fn push_enemies_to_selectables(selectables: &mut Vec<Vec<MenuItem>>, enemies: &Vec<Vec<Enemy>>, action_effects: ActionTuple) {
  let perform_battle_action = |
    party: &mut Vec<Character>,
    enemies: &mut Vec<Vec<Enemy>>,
    target_ids: Vec<usize>,
    action_effects: ActionTuple,
    notification: &mut Notification
  | {
    let acting_character = party.iter_mut().find(|character: &&mut Character| character.get_battle_state().is_turn_active()).unwrap();
    acting_character.perform_battle_action();

    for enemy in enemies.iter_mut().flatten() {
      if target_ids.contains(&enemy.get_id()) {
        enemy.receive_battle_action(action_effects.1, action_effects.0(acting_character.get_battle_state_mut(), notification));
      }
    }
  };

  for (i, enemy_row) in enemies.iter().enumerate() {
    selectables.push(Vec::new());
    for (j, enemy) in enemy_row.iter().enumerate() {
      if enemy.get_battle_state().get_hp() > 0 {
        selectables.last_mut().unwrap().push(MenuItem::new(
          String::new(),
          700. + i as f32 * 100.,
          180. + j as f32 * 100.,
          OnClickEvent::BattleAction(perform_battle_action, vec![enemy.get_id()], action_effects)
        ));
      }
    }
    if selectables.last().unwrap().len() == 0 {
      selectables.remove(selectables.len() - 1);
    }
  }
}
