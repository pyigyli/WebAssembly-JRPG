use crate::game::map::npc::Npc;

pub fn nurse_seraphine() -> Npc {
  Npc::new(String::from("Nurse_Seraphine"), String::from("Seraphine"), 1, 4, true, vec![String::from("Hello dude, what up?"), String::from("Cool bro!")])
}
