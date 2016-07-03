use floor::Floor;
use direction::Direction;

pub struct Elevator {
    current_floor: Floor,
    target_floor: Option<Floor>,
    direction: Option<Direction>,
    // floors/tick
    velocity: Floor,
}

impl Elevator {
    pub fn is_at_floor_number(&self, floor_number: usize) -> bool {
        self.is_at_floor(Floor::from_floor_number(floor_number))
    }

    pub fn is_at_floor(&self, floor: Floor) -> bool {
        self.current_floor == floor
    }

    pub fn is_idle(&self) -> bool {
        self.target_floor.is_none() && self.direction.is_none()
    }
}

