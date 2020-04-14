macro_rules! iter_alive_members {
  ($party:expr) => {
    $party.iter_mut().filter(|character: &&mut Character| character.get_battle_state().get_hp() > 0)
  };
}

macro_rules! alive_members_count {
  ($party:expr) => {
    $party.iter().fold(0, |count: u32, character: &Character| {
      if character.get_battle_state().get_hp() > 0 {
        return count + 1;
      }
      count
    })
  };
}
