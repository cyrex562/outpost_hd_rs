use std::ops::{Add, AddAssign, Sub, SubAssign};
use crate::vector_2d::Vector2D;

#[derive(Default,Debug,Clone,PartialEq,PartialOrd)]
pub struct Point<T>
{
    pub x: T,
    pub y: T
}

impl Point<T>
{
    pub fn skew_by(&mut self, other: &Vector2D<T>) -> Point<T>
    {
        Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }

    pub fn skew_inverse_by(&mut self, other: &Vector2D<T>) -> Point<T>
    {
        if other.x == 0 || other.y == 0 {
            panic!("cannot skew inverse with Vector2D containing a zero")
        }

        Self {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }

    pub fn to<U>(&mut self) -> Point<U>{
        Self {
            x: self.x as U,
            y: self.x as U
        }
    }

    pub fn add_vector(&mut self, vector: &Vector2D<T>) -> Point<T>
    {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y
        }
    }
}

impl Into<U> for Point<T>
{
    fn into(self) -> Point<U> {
        Self {
            x: self.x as U,
            y: self.y as U
        }
    }
}

impl AddAssign for Point<T>
{
    fn add_assign(&mut self, values: &Vector2D<T>) {
        self.x += values.x;
        self.y += values.y;
    }
}

impl SubAssign for Point<T>
{
    fn sub_assign(&mut self, values: &Vector2D<T>) {
        self.x -= values.x;
        self.y -= values.y;
    }
}

impl Add for Point<T>
{
    type Output = Self;

    fn add(self, rhs: Vector2D<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Point<T>
{
    type Output = Self;

    fn sub(self, rhs: Vector2D<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

