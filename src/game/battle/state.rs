use crate::game::menu::font::print_text;
use crate::webgl::shader_program::ShaderProgram;

pub struct BattleState {
  level: u32,
  experience: u32,
  hp: u16,
  hp_growth_rate: f32,
  mp: u16,
  mp_growth_rate: f32,
  att: f64,
  att_growth_rate: f32,
  def: f64,
  def_growth_rate: f32,
  mag: f64,
  mag_growth_rate: f32,
  int: f64,
  int_growth_rate: f32,
  res: f64,
  res_growth_rate: f32,
  agi: f64,
  agi_growth_rate: f32,
  atb: u8,
  atb_subtick: f64,
  is_turn_active: bool
}

impl BattleState {
  pub fn new(
    level: u32,
    hp:  u16, hp_growth_rate:  f32,
    mp:  u16, mp_growth_rate:  f32,
    att: f64, att_growth_rate: f32,
    def: f64, def_growth_rate: f32,
    mag: f64, mag_growth_rate: f32,
    int: f64, int_growth_rate: f32,
    res: f64, res_growth_rate: f32,
    agi: f64, agi_growth_rate: f32
  ) -> Self {
    Self {
      level,
      experience: 0,
      hp,  hp_growth_rate,
      mp,  mp_growth_rate,
      att, att_growth_rate,
      def, def_growth_rate,
      mag, mag_growth_rate,
      int, int_growth_rate,
      res, res_growth_rate,
      agi, agi_growth_rate,
      atb: 0,
      atb_subtick: 0.,
      is_turn_active: false
    }
  }

  pub fn update(&mut self) {
    if self.get_hp() > 0 {
      self.update_atb();
    }
  }

  pub fn update_atb(&mut self) {
    if self.is_atb_full() {
      self.atb = 0;
    }
    self.atb_subtick += self.agi;
    while self.atb_subtick > 4. {
      self.atb_subtick -= 5.;
      if let Some(atb) = self.atb.checked_add(1) {
        self.atb = atb;
      }
    }
  }

  pub fn start_turn(&mut self) {
    self.is_turn_active = true;
  }

  pub fn end_turn(&mut self) {
    self.is_turn_active = false;
    self.atb = 0;
    self.atb_subtick = 0.;
  }

  pub fn get_experience(&self) -> u32 {
    self.experience
  }

  pub fn add_experience(&mut self, experience: u32) {
    self.experience += experience;
    while self.experience >= self.level * self.level * 150 - 100 {
      self.experience -= self.level * self.level * 150 - 100;
      self.level += 1;
    }
  }

  pub fn is_turn_active(&self) -> bool {
    self.is_turn_active
  }

  pub fn is_atb_full(&self) -> bool {
    self.atb == std::u8::MAX
  }

  pub fn get_hp(&self) -> u16 {
    self.hp
  }

  pub fn reduce_hp(&mut self, value: u16) {
    if let Some(new_hp) = self.hp.checked_sub(value) {
      self.hp = new_hp;
    } else {
      self.hp = 0;
    }
  }

  pub fn get_attack_stat(&self) -> f64 {
    self.att
  }

  pub fn get_defence_stat(&self) -> f64 {
    self.def
  }

  pub fn draw(&self, program: &mut ShaderProgram, name: &String, id: usize) {
    print_text(program, name.to_owned(),            330.,                                                    384. + id as f32 * 62.);
    print_text(program, format!("{}/",    self.hp), 560. + (5 - format!("{}/", self.hp).len()) as f32 * 20., 384. + id as f32 * 62.);
    print_text(program, format!("{} HP" , self.hp), 660. + (4 - format!("{}" , self.hp).len()) as f32 * 20., 384. + id as f32 * 62.);
    print_text(program, format!("{}/",    self.mp), 834. + (4 - format!("{}/", self.mp).len()) as f32 * 20., 384. + id as f32 * 62.);
    print_text(program, format!("{} MP" , self.mp), 930. + (3 - format!("{}" , self.mp).len()) as f32 * 20., 384. + id as f32 * 62.);
  }
}
