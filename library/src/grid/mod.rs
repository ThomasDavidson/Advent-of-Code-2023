#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    None,
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn get_translation(self) -> (i16, i16) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::None => (0, 0),
        }
    }
    pub fn inverse(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::None => Direction::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridState {
    pub direction: Direction,
    pub x: usize,
    pub y: usize,
}
impl GridState {
    pub fn check_bounds(&self, width: usize, height: usize) -> bool {
        match self.direction {
            Direction::South => {
                if self.y + 1 == height {
                    false
                } else {
                    true
                }
            }
            Direction::East => {
                if self.x + 1 == width {
                    false
                } else {
                    true
                }
            }
            Direction::North => {
                if self.y == 0 {
                    false
                } else {
                    true
                }
            }
            Direction::West => {
                if self.x == 0 {
                    false
                } else {
                    true
                }
            }
            Direction::None => true,
        }
    }
}
