use crate::game::battle::enemy::Enemy;
use crate::game::data::enemies::*;

pub fn test_room_formation_1() -> Vec<Vec<Enemy>> {
  vec![
    vec![test_circle(5), test_circle(7)],
    vec![test_circle(6), test_circle(8)]
  ]
}
