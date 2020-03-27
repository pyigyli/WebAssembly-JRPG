use crate::game::menu::container::MenuContainer;
use crate::game::menu::item::{MenuItem, OnClickEvent};
use crate::game::menu::MenuScreen;
use crate::game::transition::{Transition, TransitionStyle};

pub fn none_menu() -> MenuScreen {
  MenuScreen::new(Vec::new(), vec![Vec::new()], Vec::new(), 0, 0, OnClickEvent::MenuTransition(|_transition: &mut Transition| ()))
}

pub fn main_menu() -> MenuScreen {
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
  MenuScreen::new(containers, selectables, unselectables, 0, 0, OnClickEvent::MenuTransition(exit_menu))
}

pub fn item_menu() -> MenuScreen {
  let back_to_main_menu = |transition: &mut Transition| transition.set(TransitionStyle::MenuIn(main_menu));
  MenuScreen::new(vec![MenuContainer::new(16., 16., 1064., 704.)], Vec::new(), Vec::new(), 0, 0, OnClickEvent::MenuTransition(back_to_main_menu))
}
