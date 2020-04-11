use crate::game::battle::enemy::Enemy;
use crate::game::data::enemies::*;

// all enemy formations are row of columns

pub fn test_room_formation_1() -> Vec<Vec<Enemy>> {
  vec![
    vec![test_circle(5)],
  ]
}
