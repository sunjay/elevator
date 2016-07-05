use floor::Floor;
use elevator::Elevator;
use floor_controls::FloorControls;

#[derive(Debug, Clone)]
pub struct ElevatorController {
    elevators: Vec<Elevator>,
    floors: Vec<FloorControls>,
    // floors/tick
    velocity: Floor,
}

impl ElevatorController {
    pub fn new(velocity: Floor) -> ElevatorController {
        let mut ctrl = ElevatorController {
            elevators: Vec::new(),
            floors: Vec::new(),
            velocity: velocity,
        };
        // need at least one floor and one elevator
        ctrl.add_floor();
        ctrl.add_elevator();

        ctrl
    }

    /// Update should be called once per tick
    /// It is up to you to define what and how long a tick is
    pub fn update(&mut self) {
        // It is important that this function bare no dependence
        // on the number of elevators staying the same between calls
        // or on the number of floors remaining the same either
        // The calculations should work independently of any of those factors
        // and consider which elevators should be directed where "freshly"
        // each time
    }

    pub fn all(&self) -> &Vec<Elevator> {
        &self.elevators
    }

    pub fn all_floors(&self) -> &Vec<FloorControls> {
        &self.floors
    }

    pub fn vel(&self) -> Floor {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Floor) {
        self.velocity = velocity;
    }

    pub fn add_elevator(&mut self) {
        self.elevators.push(Elevator::new());
    }

    pub fn add_floor(&mut self) {
        self.floors.push(FloorControls::new());
    }
}

