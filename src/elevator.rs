use floor::Floor;
use direction::Direction;

pub struct Elevator {
    position: Floor,
    direction: Option<Direction>,
}

impl Elevator {
    pub fn is_at_floor_number(&self, floor_number: usize) -> bool {
        self.is_at_floor(Floor::from_floor_number(floor_number))
    }

    pub fn is_at_floor(&self, floor: Floor) -> bool {
        self.position == floor
    }

    /// Returns the last floor this elevator was at
    pub fn last_floor(&self) -> Floor {
        self.direction.as_ref().map(|d| match *d {
            Direction::Up => self.position.round_down(),
            Direction::Down => self.position.round_up(),
        }).unwrap_or_else(|| {
            debug_assert!(self.position.round_down() == self.position,
                "Elevator was not stopped at a floor");
            self.position.clone()
        })
    }

    pub fn stop(&mut self) {
        self.direction = None;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = Some(direction);
    }

    /// Moves in the current direction towards the given target
    /// Since it is possible that the given velocity will cause the
    /// elevator to move past the target, the elevator is able to
    /// move itself directly on to the target if it close enough
    pub fn move_towards_target(&mut self, target: Floor, velocity: Floor) {
        assert!(!self.direction.is_none(), "Cannot move while stopped");

        if self.position.distance_to(&target) < velocity {
            self.position = target;
        }
        else {
            self.position = self.direction.as_ref().map(|d| match *d {
                Direction::Up => self.position + velocity,
                Direction::Down => self.position - velocity,
            }).unwrap()
        }
    }
}

