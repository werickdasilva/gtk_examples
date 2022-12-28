use gtk::{prelude::*, traits::GtkWindowExt, Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("ex.create_window")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Create Window")
            .default_height(400)
            .default_width(600)
            .build();

        window.present();
    });

    app.run();
}
