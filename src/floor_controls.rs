use direction::Direction;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FloorControls {
    up: bool,
    down: bool,
}

impl FloorControls {
    pub fn new() -> FloorControls {
        FloorControls {
            up: false,
            down: false,
        }
    }

    /// Returns if the up button is pressed
    pub fn is_up_pressed(&self) -> bool {
        self.up
    }

    /// Returns if the down button is pressed
    pub fn is_down_pressed(&self) -> bool {
        self.down
    }

    /// Activates the given direction button
    /// No change if that button is already pressed
    pub fn press(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.up = true,
            Direction::Down => self.down = true,
        }
    }
}

