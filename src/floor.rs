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
// Note: Modifying this value has serious effects on how raw u32 values
// are interpreted in the rest of the code. Do so with caution.
const PRECISION: u32 = 2;

lazy_static! {
    static ref BASE_FLOOR: u32 = 10u32.pow(PRECISION);
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub struct Floor(u32);

impl Floor {
    pub fn new(floor: u32) -> Floor {
        Floor(floor)
    }

    /// Returns the ground floor (bottom floor)
    pub fn ground() -> Floor {
        Floor::new(*BASE_FLOOR)
    }

    /// Returns a fraction of a floor
    /// A more dependable way to get a fraction of a floor, independent
    /// of the implementation and of what PRECISION is defined to be
    pub fn fraction(numerator: u32, denominator: u32) -> Floor {
        Floor::new(numerator * *BASE_FLOOR / denominator)
    }

    pub fn from_floor_number(number: usize) -> Floor {
        let floor = number as u32 * *BASE_FLOOR;
        assert!(floor >= *BASE_FLOOR,
            "Cannot create a floor under the bottom floor");
        Floor::new(floor)
    }

    /// Returns the floor number this floor represent
    /// Should only be called on a whole (rounded) floor
    /// Round the floor based on the direction of the elevator
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
        let Floor(rhs_value) = rhs;
        let Floor(value) = self;

        Floor::new(value + rhs_value)
    }
}

impl Sub for Floor {
    type Output = Floor;

    fn sub(self, rhs: Floor) -> Floor {
        // since floors cannot be negative, it is an error if this
        // subtraction would cause wrap around
        // note that we can't just test the difference because
        // these are unsigned numbers
        if rhs > self {
            panic!("Cannot have a Floor less than zero");
        }
        else {
            let Floor(rhs_value) = rhs;
            let Floor(value) = self;

            Floor::new(value - rhs_value)
        }
    }
}

