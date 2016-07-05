use std::sync::{Arc, RwLock};

use kiss_ui;
use kiss_ui::prelude::*;

use kiss_ui::container::{Vertical, Horizontal, HAlign, VAlign};
use kiss_ui::text::Label;
use kiss_ui::button::Button;
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
    pub fn start(self, fps: u32) {
        // Need to use reference counting here because these
        // closures may not all last the same amount of time
        let shared = Arc::new(RwLock::new(self));

        kiss_ui::show_gui(|| {
            let copy = shared.clone();
            shared.write().unwrap().create_ui()
                .set_title("Elevator")
                .set_size_pixels(1024, 640)
                .set_on_show(move |_| {
                    let copy2 = copy.clone();
                    Timer::new()
                        .set_interval(1000 / fps)
                        .set_on_interval(move |_| {
                            copy2.write().unwrap().update();
                        }).start();
                })
        });
    }

    pub fn update(&mut self) {
        self.controller.update();
        //TODO: update interface components
    }

    pub fn create_ui(&mut self) -> Dialog {
        Dialog::new(
            Horizontal::new(
                children![
                    self.render_elevators(),
                    self.floor_buttons(),
                ]
            ).set_elem_spacing_pixels(10).set_valign(VAlign::Center)
        )
    }

    pub fn render_elevators(&mut self) -> BaseWidget {
        Horizontal::new(
            self.controller.all().iter().map(|e| {
                Vertical::new(
                    self.controller.all_floors().iter().enumerate().map(|(i, _)| {
                        Horizontal::new(children![
                            Label::new(i.to_string()),
                            Button::new().set_label(i.to_string()),
                        ]).set_valign(VAlign::Center).set_elem_spacing_pixels(10).to_base()
                    }).collect::<Vec<_>>()
                ).set_halign(HAlign::Center).set_elem_spacing_pixels(10).to_base()
            }).collect::<Vec<_>>()
        ).set_elem_spacing_pixels(10).to_base()
    }

    pub fn floor_buttons(&mut self) -> BaseWidget {
        Button::new().to_base()
    }
}


