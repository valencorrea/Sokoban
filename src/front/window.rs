/*use crate::SokobanError;

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

    let window: ApplicationWindow = builder.object("Window").expect("Couldn't get Window");
    window.show_all();
    gtk::main();
}

pub fn run_app() -> Result<(), SokobanError> {
    show_ui();
    Ok(())
}*/
