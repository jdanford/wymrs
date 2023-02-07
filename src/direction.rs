use crate::Point;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RelativeDirection {
    Forward = 0,
    Right = 1,
    Backward = 2,
    Left = 3,
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::Up),
            1 => Ok(Direction::Right),
            2 => Ok(Direction::Down),
            3 => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

impl From<RelativeDirection> for Direction {
    fn from(direction: RelativeDirection) -> Self {
        match direction {
            RelativeDirection::Forward => Direction::Up,
            RelativeDirection::Right => Direction::Right,
            RelativeDirection::Backward => Direction::Down,
            RelativeDirection::Left => Direction::Left,
        }
    }
}

impl From<Direction> for RelativeDirection {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => RelativeDirection::Forward,
            Direction::Right => RelativeDirection::Right,
            Direction::Down => RelativeDirection::Backward,
            Direction::Left => RelativeDirection::Left,
        }
    }
}

impl Direction {
    #[must_use]
    pub fn rotate(&self, offset: RelativeDirection) -> Direction {
        let di = *self as usize;
        let oi = offset as usize;
        let i = (di + oi + 4) % 4;
        i.try_into().expect("invalid direction")
    }
}

impl From<Direction> for Point<i8> {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
        }
    }
}
