use crate::game::map::tile::Tile;
use crate::webgl::shader_program::ShaderProgram;

pub struct TileGrid {
  tiles: Vec<Vec<Tile>>
}

impl TileGrid {
  pub fn new(tiles: Vec<Vec<Tile>>) -> Self {
    Self {
      tiles
    }
  }

  pub fn set_occupied(&mut self, x: usize, y: usize) {
    self.tiles[y][x].set_occupied();
  }

  pub fn set_unoccupied(&mut self, x: usize, y: usize) {
    self.tiles[y][x].set_unoccupied();
  }

  pub fn is_tile_empty(&self, x: usize, y: usize) -> bool {
    self.tiles[y][x].is_walkable()
  }

  pub fn draw(&self, program: &mut ShaderProgram, player_coords: (f32, f32)) {
    for row in self.tiles.iter() {
      for tile in row.iter() {
        tile.draw(program, player_coords);
      }
    }
  }
}
