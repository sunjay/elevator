use floor::Floor;
use elevator::Elevator;
use floor_controls::FloorControls;

pub struct ElevatorController {
    elevators: Vec<Elevator>,
    floors: Vec<FloorControls>,
    // floors/tick
    velocity: Floor,
}

impl ElevatorController {
    fn new(velocity: Floor) -> ElevatorController {
        ElevatorController {
            elevators: Vec::new(),
            floors: Vec::new(),
            velocity: velocity,
        }
    }

    /// Update should be called once per tick
    /// It is up to you to define what and how long a tick is
    fn update(&mut self) {
        // It is important that this function bare no dependence
        // on the number of elevators staying the same between calls
        // or on the number of floors remaining the same either
        // The calculations should work independently of any of those factors
        // and consider which elevators should be directed where "freshly"
        // each time
    }

    fn all(&self) -> &Vec<Elevator> {
        &self.elevators
    }

    fn all_floors(&self) -> &Vec<FloorControls> {
        &self.floors
    }

    fn vel(&self) -> Floor {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: Floor) {
        self.velocity = velocity;
    }
}

