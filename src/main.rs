#[macro_use]
extern crate kiss_ui;

use kiss_ui::prelude::*;

use kiss_ui::container::Horizontal;
use kiss_ui::text::Label;

fn main() {
    kiss_ui::show_gui(|| {
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
        .set_title("Elevator")
        .set_size_pixels(1024, 640)
    });
}