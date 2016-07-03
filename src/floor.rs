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

use std::ops::Add;

// The power of 10 to use for representing whole floors
const PRECISION: u32 = 2;

lazy_static! {
    static ref BASE_FLOOR: u32 = 10u32.pow(PRECISION);
}

#[derive(Eq, PartialEq)]
pub struct Floor(u32);

impl Floor {
    pub fn new(floor: u32) -> Floor {
        assert!(floor >= *BASE_FLOOR);
        Floor(floor)
    }

    pub fn from_floor_number(number: usize) -> Floor {
        Floor::new(number as u32 * *BASE_FLOOR)
    }

    pub fn last_floor(&self) -> Floor {
        Floor::new(self.0 / *BASE_FLOOR * *BASE_FLOOR)
    }
}

impl Add for Floor {
    type Output = Floor;

    fn add(self, rhs: Floor) -> Floor {
        unimplemented!();
    }
}

