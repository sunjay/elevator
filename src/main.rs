#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate conrod;

extern crate find_folder;
extern crate piston_window;

use conrod::{Theme, Widget};
use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};

/// Conrod is backend agnostic. Here, we define the `piston_window` backend to use for our `Ui`.
type Backend = (piston_window::G2dTexture<'static>, piston_window::Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

mod floor;
mod direction;
mod elevator;
mod floor_controls;
mod elevator_controller;

use floor::Floor;
use elevator_controller::ElevatorController;

fn main() {
    let fps = 60;
    let velocity = Floor::fraction(1, fps);
    let mut controller = ElevatorController::new(velocity);
    for _ in 0..4 {
        controller.add_floor();
    }
    controller.add_elevator();

    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new("Elevator", [640, 480])
            .opengl(OpenGL::V3_2).exit_on_esc(true).samples(4).vsync(true).build().unwrap();

    // Construct our `Ui`.
    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = piston_window::Glyphs::new(&font_path, window.factory.clone()).unwrap();
        Ui::new(glyph_cache, theme)
    };

    window.set_ups(fps as u64);

    // Poll events from the window.
    while let Some(event) = window.next() {
        ui.handle_event(event.clone());
        event.update(|_| ui.set_widgets(|ui_cell| set_ui(ui_cell, &mut controller)));
        window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
    }
}

// Declare the `WidgetId`s and instantiate the widgets.
fn set_ui(ref mut ui: UiCell, controller: &mut ElevatorController) {
}

