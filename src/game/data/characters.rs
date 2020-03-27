use crate::game::battle::character::Character;
use crate::game::data::{battle_actions, battle_menus};
use crate::game::menu::item::OnClickEvent;

pub fn darrel_deen(id: usize) -> Character {
  Character::new(
    String::from("Darrel"),
    String::from("Darrel_Deen"),
    id,
    1,         // Lvl
    9999, 1.12, // Hp
    999, 0.95, // Mp
    12., 1.2,  // Attack
    7.,  1.,   // Defence
    5.,  0.8,  // Magic
    3.,  0.75, // Intelligence
    7.,  1.,   // Resistance
    10., 1.1,  // Agility
    (String::from("Attack"), OnClickEvent::ToTargetSelection(battle_menus::single_target_targeting_everyone, battle_actions::physical_attack())),
    (String::from("Steal"),  OnClickEvent::None),
    (String::from("Flee"),   OnClickEvent::None)
  )
}

pub fn nurse_seraphine(id: usize) -> Character {
  Character::new(
    String::from("Seraphine"),
    String::from("Nurse_Seraphine"),
    id,
    2,         // Lvl
    40, 0.8,   // Hp
    40, 1.2,   // Mp
    9.,  0.95, // Attack
    5.,  0.7,  // Defence
    7.,  1.2,  // Magic
    7.,  1.5,  // Intelligence
    8.,  1.05, // Resistance
    8.,  0.9,  // Agility
    (String::from("Attack"),   OnClickEvent::ToTargetSelection(battle_menus::single_target_targeting_everyone, battle_actions::physical_attack())),
    (String::from("Medicine"), OnClickEvent::None),
    (String::from("Seduce"),   OnClickEvent::None)
  )
}
