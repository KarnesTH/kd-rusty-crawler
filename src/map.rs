//! Map system module for managing game world and level generation.

use crate::Room;

/// Represents different types of tiles in the game map.
#[derive(Clone, PartialEq, Debug)]
pub enum Tile {
    /// Walkable floor tile
    Floor,
    /// Solid wall tile that blocks movement
    Wall,
    /// Door tile that can be opened/closed
    Door,
    /// Empty space (void/unused space)
    Empty,
}

/// Represents the game world map structure.
#[derive(Debug)]
pub struct Map {
    /// Width of the map in tiles
    pub width: i32,
    /// Height of the map in tiles
    pub height: i32,
    /// 2D vector of tiles representing the map layout
    pub tiles: Vec<Vec<Tile>>,
    /// Collection of rooms in the map
    pub rooms: Vec<Room>,
}

impl Map {
    /// Creates a new empty map with specified dimensions.
    ///
    /// # Arguments
    /// * `width` - The width of the map in tiles
    /// * `height` - The height of the map in tiles
    ///
    /// # Returns
    /// A new Map instance filled with Empty tiles
    pub fn new(width: i32, height: i32) -> Self {
        let tiles = vec![vec![Tile::Empty; width as usize]; height as usize];
        Map {
            width,
            height,
            tiles,
            rooms: Vec::new(),
        }
    }

    /// Gets the tile at the specified coordinates.
    ///
    /// # Arguments
    /// * `x` - The x-coordinate
    /// * `y` - The y-coordinate
    ///
    /// # Returns
    /// * `Some(&Tile)` if coordinates are valid
    /// * `None` if coordinates are out of bounds
    pub fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(&self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Sets a tile at the specified coordinates.
    ///
    /// # Arguments
    /// * `x` - The x-coordinate
    /// * `y` - The y-coordinate
    /// * `tile` - The tile to set
    ///
    /// # Note
    /// Silently fails if coordinates are out of bounds
    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.tiles[y as usize][x as usize] = tile;
        }
    }

    /// Creates a room centered in the map.
    ///
    /// # Arguments
    /// * `room` - The room to create
    ///
    /// # Note
    /// Creates walls around the perimeter and floor tiles inside
    pub fn create_room(&mut self, room: Room) {
        let x = (self.width - room.width) / 2;
        let y = (self.height - room.height) / 2;

        for i in y..y + room.height {
            for j in x..x + room.width {
                if i == y || i == y + room.height - 1 || j == x || j == x + room.width - 1 {
                    self.set_tile(j, i, Tile::Wall);
                } else {
                    self.set_tile(j, i, Tile::Floor);
                }
            }
        }
    }
}
