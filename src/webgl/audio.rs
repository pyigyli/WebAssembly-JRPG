use web_sys::HtmlAudioElement;

pub struct Audio {
  soundtracks: Vec<HtmlAudioElement>,
  sfxs: Vec<HtmlAudioElement>,
  current_soundtrack: Option<usize>,
  current_sfx: Option<usize>
}

impl Audio {
  pub fn new() -> Self {
    Self {
      soundtracks: Vec::new(),
      sfxs: Vec::new(),
      current_soundtrack: None,
      current_sfx: None
    }
  }

  pub fn add_soundtrack(&mut self, soundtrack: HtmlAudioElement) {
    self.soundtracks.push(soundtrack);
  }

  pub fn add_sfx(&mut self, sfx: HtmlAudioElement) {
    self.sfxs.push(sfx);
  }

  pub fn set_soundtrack(&mut self, file_name: &str) {
    for (index, element) in self.soundtracks.iter().enumerate() {
      if element.src().contains(file_name) {
        self.current_soundtrack = Some(index);
        let _promise = self.soundtracks[self.current_soundtrack.unwrap()].play().unwrap();
      }
    }
  }
}
