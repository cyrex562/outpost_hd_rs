use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Default,Debug,Clone,PartialOrd, PartialEq)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T
}

impl Vector2D<T>
{
    pub fn skew_by(&mut self, other: &Vector2D<T>) -> Vector2D<T>
    {
        Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }

    pub fn skew_inverse_by(&mut self, other: &Vector2D<T>) -> Vector2D<T>
    {
        if other.x == 0 || other.y == 0 {
            panic!("cannot skew inverse with a vector containing a 0")
        }
        Self {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }

    pub fn length_squared(&mut self) -> T {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn dot_product(&mut self, other: &Vector2D<T>) -> T {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn reflect_x(&mut self) -> Vector2D<T>
    {
        Self {
            x: self.x * -1,
            y: self.y
        }
    }

    pub fn reflect_y(&mut self) -> Vector2D<T>
    {
        Self {
            x: self.x,
            y: self.y * -1,
        }
    }

    pub fn to<U>(&mut self) ->Self<U>
    {
        Self {
            x: self.x as U,
            y: self.y as U
        }
    }
}

impl Into<U> for Vector2D<T>
{
    fn into(self) -> Vector2D<U> {
        Self {
            x: self.x as U,
            y: self.y as U
        }
    }
}

impl AddAssign for Vector2D<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }


}

impl SubAssign for Vector2D<T>
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Add for Vector2D<T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Vector2D<T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl MulAssign for Vector2D<T>
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign for Vector2D<T>
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Mul for Vector2D<T>
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Div for Vector2D<T>
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

