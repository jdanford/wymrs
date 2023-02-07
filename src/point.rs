use std::ops::{Add, Div, Mul, Sub};

use num::Num;

use crate::{Direction, Result};

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Point<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Point<T> {
    #[must_use]
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    #[must_use]
    pub fn map<U: Num, F: Fn(T) -> U>(self, f: F) -> Point<U> {
        let x = f(self.x);
        let y = f(self.y);
        Point { x, y }
    }

    #[must_use]
    pub fn map2<U: Num, F: Fn(T, T) -> U>(self, rhs: Self, f: F) -> Point<U> {
        let x = f(self.x, rhs.x);
        let y = f(self.y, rhs.y);
        Point { x, y }
    }

    #[must_use]
    pub fn wrap(self, width: T, height: T) -> Self {
        let x = self.x % width;
        let y = self.y % height;
        Point { x, y }
    }
}

impl<T: Num + Into<i32>> Point<T>
where
    T: TryFrom<i32>,
    i32: From<T>,
{
    #[allow(clippy::cast_lossless)]
    pub fn move_in_direction(self, direction: Direction) -> Result<Self> {
        let point = self.map(i32::from);
        let offset = Point::from(direction).map(|n| n as i32);
        let ix = point.x + offset.x;
        let iy = point.y + offset.y;
        let x = T::try_from(ix).map_err(|_| format!("can't cast {ix}"))?;
        let y = T::try_from(iy).map_err(|_| format!("can't cast {iy}"))?;
        Ok(Point { x, y })
    }
}

impl<T: Num> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        self.map2(rhs, Add::add)
    }
}

impl<T: Num> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.map2(rhs, Sub::sub)
    }
}

impl<T: Num> Mul for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.map2(rhs, Mul::mul)
    }
}

impl<T: Num> Div for Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: Self) -> Self::Output {
        self.map2(rhs, Div::div)
    }
}
