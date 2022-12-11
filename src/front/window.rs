use crate::SokobanError;

extern crate gio;
extern crate glib;
extern crate gtk;

use gio::{prelude::*};
use glib::{clone};
use gtk::{prelude::*, ApplicationWindow, Builder, Button, Entry, TextBuffer, ScrolledWindow, ListBox, Widget, Label, gdk};


fn show_ui() {
    gtk::init().expect("Couldn't open Window");
    let glade_src = include_str!("window.glade");
    let builder = Builder::from_string(glade_src);

    let scrolled: ScrolledWindow = builder.object("window_scrolled").expect("Couldn't get scrolled window");
    let window: ApplicationWindow = builder.object("Window").expect("Couldn't get Window");
    let command_entry: Entry = builder.object("command_entry").expect("Couldn't get command entry");
    let textbuffer: TextBuffer = builder.object("textbuffer_message").expect("Couldn't get text buffer");

    let left_button: Button = builder.object("left_button").expect("Couldn't get button");
    let right_button: Button = builder.object("right_button").expect("Couldn't get button");
    let up_button: Button = builder.object("up_button").expect("Couldn't get button");
    let down_button: Button = builder.object("down_button").expect("Couldn't get button");

    let mut left_event: gdk::Event = gdk::Event::new(gdk::EventType::KeyPress);

    // on click
    left_button.connect_clicked(|_| {
        println!("clicked");
    });

    /*right_button.connect_clicked(|_| {
    });

    up_button.connect_clicked(|_| {
    });

    down_button.connect_clicked(|_| {
    });*/

    // on enter
    command_entry.connect_activate(clone!(@weak command_entry => move |_|
        let command = command_entry.text(); // la validacion deberia tomarse desde el back
        println!("command: {}", command); // solo para probar que toma el comando, borrar
        command_entry.set_text("");
    ));

    window.show_all();
    gtk::main();
}

pub fn run_app() -> Result<(), SokobanError> {
    show_ui();
    Ok(())
}
