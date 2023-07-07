use std::cmp::Ordering;
use std::ops::{AddAssign, Div, Mul, SubAssign};

#[derive(Debug,Clone,Default,PartialOrd)]
pub struct StorableResources
{
    pub resources: [i32;4],
}

impl StorableResources {
    pub fn cap(self, other: &StorableResources) {
        
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

