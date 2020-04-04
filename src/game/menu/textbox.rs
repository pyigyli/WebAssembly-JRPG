use crate::game::menu::container::MenuContainer;
use crate::game::menu::font::print_text;
use crate::webgl::keyboard::is_pressed;
use crate::webgl::shader_program::ShaderProgram;

pub struct Textbox {
  open: bool,
  name: String,
  texts: Vec<String>,
  textbox_index: usize
}

impl Textbox {
  pub fn new() -> Self {
    Self {
      open: false,
      name: String::new(),
      texts: Vec::new(),
      textbox_index: 0
    }
  }

  pub fn update(&mut self) {
    if is_pressed("a") {
      if self.textbox_index + 1 == self.texts.len() {
        self.open = false;
        self.textbox_index = 0;
      } else {
        self.textbox_index += 1;
      }
    }
  }

  pub fn set_textboxes(&mut self, name: String, texts: &Vec<String>) {
    self.open = true;
    self.name = name;
    self.texts = texts.to_owned();
  }

  pub fn is_open(&self) -> bool {
    self.open
  }

  pub fn draw(&self, program: &mut ShaderProgram) {
    MenuContainer::new(16., 500., 1064., 704.).draw(program);
    print_text(program, self.name.to_owned(), 64., 524.);
    print_text(program, self.texts[self.textbox_index].to_owned(), 40., 556.);
  }
}
