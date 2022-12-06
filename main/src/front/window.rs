use crate::SokobanError;

extern crate gio;
extern crate glib;
extern crate gtk;

use gio::{prelude::*};
use glib::{clone};
use gtk::{prelude::*, ApplicationWindow, Builder, Button, Entry, TextBuffer, ScrolledWindow, ListBox, Widget, Label};


pub fn get_button(builder: &Builder, button_name: &str) -> Button {
    let button: Button = builder
        .object(button_name)
        .expect("Couldn't get button");
    button
}

pub fn get_buttons(builder: Builder) -> (Button, Button, Button, Button) {
    let up = get_button(&builder, "up_button");
    let down = get_button(&builder, "down_button");
    let left = get_button(&builder, "left_button");
    let right = get_button(&builder, "right_button");
    (up, down, left, right)
}

fn show_ui() {
    gtk::init().expect("Couldn't open Window");
    let glade_src = include_str!("window.glade");
    let builder = Builder::from_string(glade_src);
    let window: ApplicationWindow = builder.object("Window").expect("Couldn't get Window");

    let scrolled: ScrolledWindow = builder
        .object("window_scrolled")
        .expect("Couldn't get scrolled window");

    let (up, down, left, right) = get_buttons(builder);

    window.show_all();
    gtk::main();
}

pub fn run_app() -> Result<(), SokobanError> {
    show_ui();
    Ok(())
}