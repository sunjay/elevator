use elevator::Elevator;
use floor_controls::FloorControls;

pub struct ElevatorController {
    elevators: Vec<Elevator>,
    floors: Vec<FloorControls>,
}

