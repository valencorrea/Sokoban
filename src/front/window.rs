use crate::SokobanError;

extern crate gio;
extern crate glib;
extern crate gtk;

use gio::{prelude::*};
use glib::{clone};
use gtk::{prelude::*, ApplicationWindow, Builder, Button, Entry, TextBuffer, ScrolledWindow, ListBox, Widget, Label};


fn show_ui() {
    gtk::init().expect("Couldn't open Window");
    let glade_src = include_str!("window.glade");
    let builder = Builder::from_string(glade_src);

    let scrolled: ScrolledWindow = builder.object("window_scrolled").expect("Couldn't get scrolled window");
    let window: ApplicationWindow = builder.object("Window").expect("Couldn't get Window");

    let left_button: Button = builder.object("left_button").expect("Couldn't get button");
    let right_button: Button = builder.object("right_button").expect("Couldn't get button");
    let up_button: Button = builder.object("up_button").expect("Couldn't get button");
    let down_button: Button = builder.object("down_button").expect("Couldn't get button");

    left_button.connect_clicked(|_| {
        println!("clicked");
    });

    /*right_button.connect_clicked(|_| {
    });

    up_button.connect_clicked(|_| {
    });

    down_button.connect_clicked(|_| {
    });*/

    window.show_all();
    gtk::main();
}

pub fn run_app() -> Result<(), SokobanError> {
    show_ui();
    Ok(())
}
