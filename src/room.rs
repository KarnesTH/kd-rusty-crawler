//! Room module for dungeon room generation and management.

/// Represents a room in the dungeon.
#[derive(Debug)]
pub struct Room {
    /// Width of the room in tiles
    pub width: i32,
    /// Height of the room in tiles
    pub height: i32,
}

impl Room {
    /// Creates a new room with specified dimensions.
    ///
    /// # Arguments
    /// * `width` - The width of the room in tiles
    /// * `height` - The height of the room in tiles
    ///
    /// # Returns
    /// A new Room instance with the specified dimensions
    pub fn new(width: i32, height: i32) -> Self {
        Room { width, height }
    }

    /// Calculates the center coordinates of the room.
    ///
    /// # Returns
    /// A tuple (x, y) representing the center coordinates
    pub fn center(&self) -> (i32, i32) {
        (self.width / 2, self.height / 2)
    }
}
