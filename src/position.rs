/// A struct representing a position in the grid
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    /// X coordinate (column)
    pub x: usize,
    /// Y coordinate (row)
    pub y: usize,
}

impl Position {
    /// Creates a new position with the given coordinates
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}
