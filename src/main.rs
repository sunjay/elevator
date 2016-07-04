#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate kiss_ui;

mod floor;
mod direction;
mod elevator;
mod floor_controls;
mod elevator_controller;
mod app;

use floor::Floor;
use elevator_controller::ElevatorController;

fn main() {
    let fps = 60;
    let velocity = Floor::fraction(1, fps);
    let controller = ElevatorController::new(velocity);

    let interface = app::ElevatorInterface::new(controller);
    interface.start(fps);
}
