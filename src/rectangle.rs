use sdl2::rect::Rect;
use crate::point::Point;
use crate::vector_2d::Vector2D;

#[derive(Default,Debug,Clone,PartialEq,PartialOrd)]
pub struct Rectangle<T> {
    pub position: Point<T>,
    pub size: Vector2D<T>
}

impl Rectangle<T>
{
    pub fn create(start_point: Point<T>, end_point: Point<T>) -> Self
    {
        Self {
            position: start_point,
            size: Vector2D::from_point((&end_point - &start_point))
        }
    }

    pub fn start_point(&self) -> Point<T>
    {
        self.position.clone()
    }

    pub fn end_point(&mut self) -> Point<T>
    {
        self.position.add_vector(&self.size)
    }

    pub fn cross_x_point(&self) -> Point<T>
    {
        Point {
            x: self.position.x + self.size.x,
            y: self.position.y
        }
    }

    pub fn cross_y_point(&self) -> Point<T>
    {
        Point {
            x: self.position.x,
            y: self.position.y + self.size.y
        }
    }

    pub fn null(&mut self) -> bool {
        self.size.x == 0 || self.size.y == 0
    }

    pub fn empty(&mut self) -> bool {
        self.size.x <= 0 || self.size.y <= 0
    }

    pub fn set_start_point(&mut self, new_start_point: Point<T>) {
        self.position = new_start_point
    }

    pub fn translate(&mut self, offset: &Vector2D<T>) -> Rectangle<T>
    {
        Rectangle {
            position: self.position.add_vector(offset),
            size: self.size.clone()
        }
    }

    pub fn inset(&mut self, amount: T) -> Rectangle<T>
    {
        Rectangle {
            position: Point {
                x: self.position.x + amount,
                y: self.position.y + amount
            },
            size: Vector2D {
                x: self.size.x - amount * 2,
                y: self.size.y - amount * 2
            }
        }
    }

    pub fn inset2(&mut self, amount: Vector2D<T>) -> Rectangle<T>
    {
        Rectangle {
            position: Point {
                x: self.position.x + amount.x,
                y: self.position.y + amount.y
            },
            size: Vector2D {
                x: self.size.x - amount.x * 2,
                y: self.size.y - amount.y * 2
            }
        }
    }

    pub fn inset3(&mut self, amount_start: Vector2D<T>, amount_end: Vector2D<T>) -> Rectangle<T>
    {
        Rectangle {
            position: Point {
                x: self.position.x + amount_start.x,
                y: self.position.y + amount_start.y
            },
            size: Vector2D {
                x: self.size.x - amount_start.x - amount_end.x,
                y: self.size.y - amount_start.y - amount_end.y
            }
        }
    }

    pub fn skew_by(&mut self, scale_factor: &Vector2D<T>) -> Rectangle<T>
    {
        Rectangle {
            position: self.position.skew_by(scale_factor),
            size: self.size.skew_by(scale_factor)
        }
    }

    pub fn skew_inverse_by(&mut self, scale_factor: &Vector2D<T>) -> Rectangle<T>
    {
        Rectangle {
            position: self.position.skew_inverse_by(scale_factor),
            size: self.size.skew_inverse_by(scale_factor)
        }
    }

    pub fn to<U>(&mut self) -> Rectangle<U>
    {
        Rectangle {
            position: self.position.to(),
            size: self.size.to()
        }
    }

    pub fn contains_point(&self, point: &Point<T>) -> bool
    {
        point.x >= self.position.x && point.x <= self.position.x + self.size.x &&
        point.y >= self.position.y && point.y <= self.position.y + self.size.y
    }

    pub fn contains_rectangle(&self, rectangle: &mut Rectangle<T>) -> bool
    {
        self.contains_point(&rectangle.position) && self.contains_point(&rectangle.end_point())
    }

    pub fn overlaps(&mut self, rect: &Rectangle<T>) -> bool {
        self.position.x < rect.position.x + rect.size.x &&
        self.position.x + self.size.x > rect.position.x &&
        self.position.y < rect.position.y + rect.size.y &&
        self.position.y + self.size.y > rect.position.y
    }

    pub fn center(&mut self) -> Point<T>
    {
        Point {
            x: self.position.x + self.size.x / 2,
            y: self.position.y + self.size.y / 2
        }
    }
}