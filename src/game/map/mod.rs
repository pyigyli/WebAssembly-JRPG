pub mod npc;
pub mod player;
mod tile;
mod tilegrid;

use npc::Npc;
use tile::Tile;
use tilegrid::TileGrid;
use crate::game::battle::enemy::Enemy;
use crate::webgl::audio::Audio;
use crate::webgl::shader_program::ShaderProgram;
use js_sys::Math::random;

pub struct Map {
  soundtrack_file: String,
  tiles:  TileGrid,
  npcs: Vec<Npc>,
  encounter_function: fn(f64) -> Option<Vec<Vec<Enemy>>>
}

impl Map {
  pub fn new(
    tileset_folder: String,
    soundtrack_file: String,
    tile_keys_and_blocks: Vec<Vec<(&str, bool)>>,
    npcs: Vec<Npc>,
    encounter_function: fn(f64) -> Option<Vec<Vec<Enemy>>>
  ) -> Self {
    let mut tiles = Vec::new();
    for (y, row) in tile_keys_and_blocks.iter().enumerate() {
      let mut tile_row = Vec::new();
      for (x, (key, blocking)) in row.iter().enumerate() {
        tile_row.push(Tile::new(format!("tilesets/{}/{}", tileset_folder, key), x, y, *blocking));
      }
      tiles.push(tile_row);
    }
    for npc in npcs.iter() {
      tiles[npc.get_y()][npc.get_x()].set_occupied();
    }
    Self {
      soundtrack_file,
      tiles: TileGrid::new(tiles),
      npcs,
      encounter_function
    }
  }

  pub fn update(&mut self, audio: &mut Audio) {
    audio.update(&self.soundtrack_file);
    for npc in self.npcs.iter_mut() {
      npc.update(&mut self.tiles);
    }
  }

  pub fn set_map(&mut self, new_map: Map) {
    *self = new_map;
  }
  
  pub fn get_encounter(&self) -> Option<Vec<Vec<Enemy>>> {
    (self.encounter_function)(random())
  }

  pub fn set_occupied(&mut self, x: usize, y: usize) {
    self.tiles.set_occupied(x, y);
  }

  pub fn set_unoccupied(&mut self, x: usize, y: usize) {
    self.tiles.set_unoccupied(x, y);
  }

  pub fn is_tile_empty(&self, x: usize, y: usize) -> bool {
    self.tiles.is_tile_empty(x, y)
  }

  pub fn draw(&self, program: &mut ShaderProgram, player_coords: (f32, f32)) {
    self.tiles.draw(program, player_coords);
    for npc in self.npcs.iter() {
      npc.draw(program, player_coords)
    }
  }
}
