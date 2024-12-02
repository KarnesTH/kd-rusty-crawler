pub struct Room {
    pub width: i32,
    pub height: i32,
}

impl Room {
    pub fn new(width: i32, height: i32) -> Self {
        Room { width, height }
    }

    pub fn center(&self) -> (i32, i32) {
        (self.width / 2, self.height / 2)
    }
}
