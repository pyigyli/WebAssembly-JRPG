use crate::game::battle::character::Character;
use crate::game::battle::enemy::Enemy;
use crate::game::data::battle_actions;
use crate::game::menu::notification::Notification;
use js_sys::Math::{floor, random};

fn random_index(length: usize) -> usize {
  floor(random() * length as f64) as usize
}

pub fn test_circle(id: usize) -> Enemy {
  Enemy::new(
    String::from("test-circle"),
    String::from("Circle"),
    id,
    1,   // Lvl
    10,  // Hp
    32,  // Mp
    12., // Attack
    7.,  // Defence
    5.,  // Magic
    3.,  // Intelligence
    7.,  // Resistance
    17.,  // Agility
    |party: &mut Vec<Character>, enemies: &mut Vec<Vec<Enemy>>, notification: &mut Notification| {
      let action_effects = battle_actions::physical_attack();
      let acting_enemy = enemies.iter_mut().flatten().find(|enemy: &&mut Enemy| enemy.get_battle_state().is_turn_active()).unwrap();
      let party_length = party.len();
      party[random_index(party_length)].receive_battle_action(action_effects.1, action_effects.0(acting_enemy.get_battle_state_mut(), notification));
    }
  )
}
