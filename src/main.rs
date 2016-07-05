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
use app::ElevatorInterface;

fn main() {
    let fps = 60;
    let velocity = Floor::fraction(1, fps);
    let mut controller = ElevatorController::new(velocity);
    for _ in 0..4 {
        controller.add_floor();
    }
    controller.add_elevator();

    let interface = ElevatorInterface::new(controller);
    interface.start(fps);
}
