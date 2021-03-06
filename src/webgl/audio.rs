use web_sys::{window, HtmlAudioElement};

pub struct Audio {
  soundtracks: Vec<HtmlAudioElement>,
  sfxs: Vec<HtmlAudioElement>,
  current_soundtrack: Option<usize>,
  soundtrack_volume: f64,
  sfx_volume: f64
}

impl Audio {
  pub fn new() -> Self {
    Self {
      soundtracks: Vec::new(),
      sfxs: Vec::new(),
      current_soundtrack: None,
      soundtrack_volume: 0.5,
      sfx_volume: 0.5
    }
  }

  pub fn update(&mut self, file_name: &String) {
    for (index, element) in self.soundtracks.iter().enumerate() {
      if element.src().contains(file_name) {
        self.soundtracks[index].set_volume(self.soundtrack_volume);

        if let Some(current) = self.current_soundtrack {
          if current != index {
            self.stop_soundtrack();
            self.play_soundtrack(index);
          } else if self.soundtracks[current].ended() {
            self.soundtracks[current].set_current_time(5.14287);
            let _promise = self.soundtracks[index].play().unwrap();
          }
        } else {
          self.play_soundtrack(index);
        }
        return;
      }
    }
    self.stop_soundtrack();
  }

  fn play_soundtrack(&mut self, index: usize) {
    self.current_soundtrack = Some(index);
    let _promise = self.soundtracks[index].play().unwrap();
  }

  fn stop_soundtrack(&mut self) {
    if let Some(current) = self.current_soundtrack {
      self.soundtracks[current].pause().unwrap();
      self.soundtracks[current].set_current_time(0.);
      self.current_soundtrack = None;
    }
  }

  pub fn play_sfx(&mut self, file_name: &str) {
    let sfx = self.sfxs.iter().find(|element: &&HtmlAudioElement| element.src().contains(file_name)).unwrap();
    sfx.set_current_time(0.);
    sfx.set_volume(self.sfx_volume);
    let _promise = sfx.play().unwrap();
  }

  pub fn add_soundtrack(&mut self, soundtrack: HtmlAudioElement) {
    self.soundtracks.push(soundtrack);
  }

  pub fn add_sfx(&mut self, sfx: HtmlAudioElement) {
    self.sfxs.push(sfx);
  }

  pub fn set_soundtrack_volume(&mut self, value: f64) {
    self.soundtrack_volume = value;
  }

  pub fn set_sfx_volume(&mut self, value: f64) {
    self.sfx_volume = value;
  }
}
