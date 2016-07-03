//! Models a value representing a floor to a certain fixed accuracy
//! Enforces invariants like floors being >= 100 and not being able
//! to accidentally skip floors
//! Not being able to skip floors means that care must be taken to
//! never add to a floor a value that would result in that floor not
//! landing on the next "whole" floor
//! A "whole" floor is a value of the given precision. So if the precision
//! is 2, the whole floors are 100, 200, 300, 400
//! You can't do 100 + 101 because that would result in floor 200 being
//! skipped.
//! This is supported with the `unchecked_add()` method

use std::cmp::{max, min};
use std::ops::{Add, Sub};

// The power of 10 to use for representing whole floors
const PRECISION: u32 = 2;

lazy_static! {
    static ref BASE_FLOOR: u32 = 10u32.pow(PRECISION);
}

#[derive(Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub struct Floor(u32);

impl Floor {
    pub fn new(floor: u32) -> Floor {
        Floor(floor)
    }

    pub fn from_floor_number(number: usize) -> Floor {
        let floor = number as u32 * *BASE_FLOOR;
        assert!(floor >= *BASE_FLOOR);
        Floor::new(floor)
    }

    pub fn to_floor_number(&self) -> usize {
        (self.0 / *BASE_FLOOR) as usize
    }

    /// Round up to the nearest floor number
    pub fn round_up(&self) -> Floor {
        Floor::new((self.0 + *BASE_FLOOR) / *BASE_FLOOR * *BASE_FLOOR)
    }

    /// Round down to the nearest floor number
    pub fn round_down(&self) -> Floor {
        Floor::new(self.0 / *BASE_FLOOR * *BASE_FLOOR)
    }

    /// Gets the distance from one floor to another
    pub fn distance_to(&self, other: &Floor) -> Floor {
        Floor::new(max(self.0, other.0) - min(self.0, other.0))
    }
}

impl Add for Floor {
    type Output = Floor;

    fn add(self, rhs: Floor) -> Floor {
        unimplemented!();
    }
}

impl Sub for Floor {
    type Output = Floor;

    fn sub(self, rhs: Floor) -> Floor {
        unimplemented!();
    }
}

