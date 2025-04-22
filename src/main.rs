mod gui;
mod ffmpeg;

use gtk::prelude::*;
use gtk::Application;

fn main() {
    let app = Application::builder()
        .application_id("com.example.mediaconverter")
        .build();

    app.connect_activate(gui::build_ui);
    app.run();
}
