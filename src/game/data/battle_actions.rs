use crate::game::battle::state::BattleState;
use crate::game::menu::notification::Notification;
use js_sys::Math::{floor, random};

fn random_in_range(start: f64, end: f64) -> f64 {
  floor(random() * (end - start) + start)
}

pub fn physical_attack() -> (for<'a, 'b> fn(&'a mut BattleState, &'b mut Notification) -> f64, for<'a> fn(&'a mut BattleState, f64)) {
  let action_for_actor = |actor_state: &mut BattleState, notification: &mut Notification| -> f64 {
    notification.set_notification(String::from("Attack"));
    actor_state.get_attack_stat() * random_in_range(9., 11.)
  };
  let action_for_target = |target_state: &mut BattleState, incoming_damage: f64| {
    target_state.reduce_hp(((incoming_damage * 5. / target_state.get_defence_stat()) as u16).min(9999))
  };
  (action_for_actor, action_for_target)
}
