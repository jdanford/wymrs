use std::collections::VecDeque;

use sdl2::rect::Point;

use crate::{Color, Direction};

pub struct Wyrm {
    pub id: u16,
    pub color: Color,
    pub direction: Direction,
    pub segments: VecDeque<Point>,
}

pub struct NewWyrmParams {
    pub id: u16,
    pub color: Color,
    pub direction: Direction,
    pub position: Point,
}

impl Wyrm {
    #[must_use]
    pub fn new(params: &NewWyrmParams) -> Self {
        let mut segments = VecDeque::new();
        segments.push_front(params.position);

        Wyrm {
            id: params.id,
            color: params.color,
            direction: params.direction,
            segments,
        }
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.segments.len()
    }

    #[must_use]
    pub fn head(&self) -> Point {
        *self.segments.get(0).expect("wyrm is empty")
    }
}
