use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug,Clone,Default,PartialOrd)]
pub struct StorableResources
{
    pub resources: [i32;4],
}

impl StorableResources {
    pub fn cap(&mut self, other: &StorableResources) -> StorableResources {
        let mut out = Self::default();
        for i in 0..self.resources.len() {
            out.resources[i] = i32::clamp(self.resources[i], 0, other.resources[i]);
        }
        out
    }

    pub fn cap_int(&mut self, max: i32) -> StorableResources {
        let mut out = Self::default();
        for i in 0 .. self.resources.len() {
            out.resources[i] = i32::clamp(self.resources[i], 0, max);
        }
        out
    }

    pub fn is_empty(&mut self) -> bool
    {
        for i in 0 .. self.resources.len() {
            if self.resources[i] > 0 {
                false
            }
        }
        true
    }

    pub fn total(&mut self) -> i32 {
        let mut y = 0i32;
        for x in self.resources.iter() {
            y += x
        }
        y
    }

    pub fn get_indices_with_stock(&mut self) -> Vec<usize>
    {
        let mut indices_with_stock: Vec<usize> = vec![];
        for i in 0 .. self.resources.len() {
            if self.resources[i] > 0 {
                indices_with_stock.push(i)
            }
        }
        indices_with_stock
    }
}

impl Mul for StorableResources {
    type Output = Self;

    fn mul(&mut self, rhs: i32) -> Self::Output {
        for i in 0 .. self.resources.len() {
            self.resources[i] *= rhs;
        }
        self
    }
}

impl Div for StorableResources {
    type Output = Self;

    fn div(&mut self, rhs: i32) -> Self::Output {
        for i in 0 .. self.resources.len() {
            self.resources[i] /= rhs;
        }
        self
    }
}

impl AddAssign for StorableResources {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0 .. self.resources.len() {
            self.resources[i] += rhs.resources[i];
        }
    }
}

impl SubAssign for StorableResources {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0 .. self.resources.len() {
            self.resources[i] -= rhs.resources[i];
        }
    }
}

impl Add for StorableResources {
    type Output = Self;

    fn add(&mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub for StorableResources {
    type Output = ();

    fn sub(&mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
