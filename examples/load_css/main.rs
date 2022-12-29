use gtk::{gdk::Display, prelude::*, Application, ApplicationWindow, CssProvider, StyleContext};

fn main() {
    let app = Application::new(Some("br.load_css"), Default::default());

    app.connect_activate(|app| {
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("style.css"));

        StyleContext::add_provider_for_display(&Display::default().expect("Erro ao conectar ao display"), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        build_ui(app);
    });

    app.run();
}

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder().application(app).title("Carregando Css").build();

    let button = gtk::Button::with_label("Style css");
    button.add_css_class("button");

    window.set_child(Some(&button));

    window.present()
}
