use crate::game::animation::Direction;
use crate::game::battle::enemy::Enemy;
use crate::game::data::enemy_formations::*;
use crate::game::data::npcs;
use crate::game::map::Map;
use crate::game::map::player::Player;

pub fn none_map(player: &mut Player) -> Map {
  player.set(0, 0, Direction::Down);
  Map::new(
    String::new(),
    String::new(),
    Vec::new(),
    Vec::new(),
    |_rng_value: f64| -> Option<Vec<Vec<Enemy>>> {None}
  )
}

pub fn debug_room(player: &mut Player) -> Map {
  player.set(3, 3, Direction::Down);
  Map::new(
    String::from("test_map"),
    String::from("test_room_map"),
    vec![
      vec![("up_left_border",   true), ("up_border",      true ), ("up_border",   true ), ("up_border",      true ), ("up_border",   true ), ("up_border",      true ), ("up_right_border",   true)],
      vec![("left_border",      true), ("floor_decour_1", false), ("floor",       false), ("floor",          false), ("floor",       false), ("floor_decour_2", false), ("right_border",      true)],
      vec![("left_border",      true), ("floor",          false), ("floor",       false), ("floor",          false), ("floor",       false), ("floor",          false), ("right_border",      true)],
      vec![("left_border",      true), ("floor_decour_2", false), ("floor",       false), ("floor",          false), ("floor",       false), ("floor",          false), ("right_border",      true)],
      vec![("left_border",      true), ("floor",          false), ("floor",       false), ("floor_decour_1", false), ("floor",       false), ("floor",          false), ("right_border",      true)],
      vec![("left_border",      true), ("floor_decour_1", false), ("floor",       false), ("floor",          false), ("floor",       false), ("floor_decour_1", false), ("right_border",      true)],
      vec![("left_border",      true), ("floor_decour_2", false), ("floor",       false), ("floor",          false), ("floor",       false), ("floor_decour_1", false), ("right_border",      true)],
      vec![("down_left_border", true), ("down_border",    true ), ("down_border", true ), ("down_border",    true ), ("down_border", true ), ("down_border",    true ), ("down_right_border", true)],
    ],
    vec![npcs::nurse_seraphine()],
    |rng_value: f64| -> Option<Vec<Vec<Enemy>>> {
      if rng_value > 0.1 {
        return Some(test_room_formation_1());
      }
      None
    }
  )
}
