use std::ops::AddAssign;
use crate::vector_2d::Vector2D;

#[derive(Default,Debug,Clone,PartialEq,PartialOrd)]
pub struct Point<T>
{
    pub x: T,
    pub y: T
}

impl Point<T>
{

}

impl AddAssign for Point<T>
{
    fn add_assign(&mut self, values: &Vector2D<T>) {
        self.x += values.x;
        self.y += values.y;
    }
}

