pub mod character;
pub mod enemy;

use crate::game::battle::state::BattleState;

#[derive(Clone, Copy)]
pub enum Direction {
  Up, Down, Left, Right
}

#[derive(Clone, Copy)]
pub enum Animation {
  StartTurn,
  EndTurn,
  Attack,
  Hurt(for<'a> fn(&'a mut BattleState, f64), f64),
  HurtSelf(for<'a> fn(&'a mut BattleState, f64), f64),
  Dead,
  Flee,
  WalkTile(Direction),
  NpcWalkTile(Direction)
}
