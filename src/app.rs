use std::sync::{Arc, RwLock};

use kiss_ui;
use kiss_ui::prelude::*;

use kiss_ui::container::Horizontal;
use kiss_ui::text::Label;
use kiss_ui::timer::Timer;

use elevator_controller::ElevatorController;

pub struct ElevatorInterface {
    controller: ElevatorController,
}

impl ElevatorInterface {
    pub fn new(controller: ElevatorController) -> ElevatorInterface {
        ElevatorInterface {
            controller: controller,
        }
    }

    /// Consumes self and starts the application
    pub fn start(mut self, fps: u32) {

        kiss_ui::show_gui(move || {
            let mut shared = Arc::new(RwLock::new(self));
            shared.write().unwrap().create_ui()
                .set_title("Elevator")
                .set_size_pixels(1024, 640)
                .set_on_show(move |_| {
                    let copy = shared.clone();
                    Timer::new()
                        .set_interval(1000 / fps)
                        .set_on_interval(move |_| {
                            copy.write().unwrap().update();
                        })
                        .start();
                })
        });
    }

    pub fn create_ui(&mut self) -> Dialog {
        Dialog::new(
            Horizontal::new(
                children![
                    Label::new("Hello, world!"),
                    Label::new("Hello, world!"),
                    Label::new("Hello, world!"),
                    Label::new("Hello, world!"),
                ]
            )
        )
    }

    pub fn update(&mut self) {
        self.controller.update();
        //TODO: update interface components
    }
}


