use crate::Room;

#[derive(Clone, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
    Door,
    Empty,
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<Tile>>,
    pub rooms: Vec<Room>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let tiles = vec![vec![Tile::Empty; width as usize]; height as usize];
        Map {
            width,
            height,
            tiles,
            rooms: Vec::new(),
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(&self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.tiles[y as usize][x as usize] = tile;
        }
    }
}
