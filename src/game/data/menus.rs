use crate::game::battle::character::Character;
use crate::game::data::maps;
use crate::game::menu::container::MenuContainer;
use crate::game::menu::click_event::OnClickEvent;
use crate::game::menu::item::MenuItem;
use crate::game::menu::{MenuMovement, MenuScreen};
use crate::game::transition::{Transition, TransitionStyle};
use crate::webgl::audio::Audio;

pub fn none_menu(_party: &mut Vec<Character>) -> MenuScreen {
  MenuScreen::new(Vec::new(), Vec::new(), Vec::new(), MenuMovement::Grid, 0, 0, OnClickEvent::MenuTransition(|_transition: &mut Transition| ()))
}

pub fn title_menu(_party: &mut Vec<Character>) -> MenuScreen {
  let to_debug_room = |transition: &mut Transition| transition.set(TransitionStyle::ChangeScene(maps::debug_room));
  let selectables = vec![
    vec![MenuItem::new(String::from("New Game"),   476., 400., OnClickEvent::None)],
    vec![MenuItem::new(String::from("Continue"),   476., 432., OnClickEvent::None)],
    vec![MenuItem::new(String::from("Debug room"), 456., 464., OnClickEvent::ChangeScene(to_debug_room))]
  ];
  MenuScreen::new(Vec::new(), selectables, Vec::new(), MenuMovement::Grid, 0, 2, OnClickEvent::None)
}

pub fn main_menu(_party: &mut Vec<Character>) -> MenuScreen {
  let to_item_menu = |transition: &mut Transition| transition.set(TransitionStyle::MenuIn(item_menu));
  let exit_menu    = |transition: &mut Transition| transition.set(TransitionStyle::MenuIn(none_menu));
  let containers = vec![
    MenuContainer::new(16.,  16., 232.,  256.),
    MenuContainer::new(256., 16., 1064., 704.)
  ];
  let selectables = vec![
    vec![MenuItem::new(String::from("Item"),    70., 48.,  OnClickEvent::MenuTransition(to_item_menu))],
    vec![MenuItem::new(String::from("Skill"),   70., 80.,  OnClickEvent::None)],
    vec![MenuItem::new(String::from("Equip"),   70., 112., OnClickEvent::None)],
    vec![MenuItem::new(String::from("Change"),  70., 144., OnClickEvent::None)],
    vec![MenuItem::new(String::from("Config"),  70., 176., OnClickEvent::None)]
  ];
  let unselectables = Vec::new();
  MenuScreen::new(containers, selectables, unselectables, MenuMovement::Grid, 0, 0, OnClickEvent::MenuTransition(exit_menu))
}

pub fn item_menu(_party: &mut Vec<Character>) -> MenuScreen {
  let back_to_main_menu = |transition: &mut Transition| transition.set(TransitionStyle::MenuIn(main_menu));
  MenuScreen::new(vec![MenuContainer::new(16., 16., 1064., 704.)], Vec::new(), Vec::new(), MenuMovement::Grid, 0, 0, OnClickEvent::MenuTransition(back_to_main_menu))
}

pub fn battle_won(party: &mut Vec<Character>, mut experience: u32) -> MenuScreen {
  let start_exp_count = |audio: &mut Audio, menu: &mut MenuScreen, party: &mut Vec<Character>| {
    
    let finish_exp_count = |audio: &mut Audio, menu: &mut MenuScreen, party: &mut Vec<Character>| {
      let exp_left = menu.get_unselectable(1).get_text().parse::<u32>().unwrap();
      let alive_count = alive_members_count!(party);
      if exp_left > 0 {
        audio.play_sfx("counter_tick");
        menu.get_unselectable(1).set_text(0.to_string());
        for character in iter_alive_members!(party) {
          character.get_battle_state_mut().add_experience(exp_left / alive_count);
          menu.get_unselectable(character.get_id() * 2 + 2).set_text(character.get_battle_state().get_experience().to_string());
        }
      } else {
        menu.end_mutation();
      }
      let exit_menu = |transition: &mut Transition| transition.set(TransitionStyle::BattleOut);
      menu.get_selectable(0, 0).set_click_event(OnClickEvent::MenuTransition(exit_menu));
    };

    let exp_left = menu.get_unselectable(1).get_text().parse::<u32>().unwrap();
    let alive_count = alive_members_count!(party);
    if exp_left > 0 {
      audio.play_sfx("counter_tick");
      menu.get_unselectable(1).set_text(format!("{}", exp_left - alive_count));
      for character in iter_alive_members!(party) {
        character.get_battle_state_mut().add_experience(1);
        menu.get_unselectable(character.get_id() * 2 + 2).set_text(character.get_battle_state().get_experience().to_string());
      }
      menu.get_selectable(0, 0).set_click_event(OnClickEvent::MutateMenu(finish_exp_count));
    } else {
      finish_exp_count(audio, menu, party);
    }
  };
  let containers = vec![
    MenuContainer::new(50.,  10.,  550.,  110.),
    MenuContainer::new(10.,  120., 1070., 360.),
    MenuContainer::new(50.,  370., 250.,  470.),
    MenuContainer::new(10.,  480., 1070., 600.),
    MenuContainer::new(760., 610., 1040., 710.)
  ];
  let alive_count = alive_members_count!(party);
  if experience % alive_count != 0 {
    experience += alive_count - experience % alive_count;
  }
  let selectables = vec![vec![MenuItem::new(String::from("Continue"), 825., 650., OnClickEvent::MutateMenu(start_exp_count))]];

  let mut unselectables = Vec::new();
  unselectables.push(MenuItem::new(String::from("Experience"), 90.,  50.,  OnClickEvent::None));
  unselectables.push(MenuItem::new(experience.to_string(),     350., 50.,  OnClickEvent::None));
  unselectables.push(MenuItem::new(String::from("Items"),      90.,  410., OnClickEvent::None));

  for (character, (x, y)) in party.iter().zip([(50., 160.), (590., 160.), (50., 260.), (590., 260.)].iter()) {
    unselectables.push(MenuItem::new(character.get_name(), *x, *y, OnClickEvent::None));
    unselectables.push(MenuItem::new(character.get_battle_state().get_experience().to_string(), x + 250., *y, OnClickEvent::None));
  }
  MenuScreen::new(containers, selectables, unselectables, MenuMovement::Grid, 0, 0, OnClickEvent::None)
}
