use gtk::Align;
use gtk::ApplicationWindow;
use gtk::Button;
use gtk::prelude::*;
use gtk::Application;

const APP_ID : &str = "fpi.trab1";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application){
    let button = Button::builder()
    .label("Press me!")
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .halign(Align::Start)
    .build();

    button.connect_clicked(move |button| {
        button.set_label("Hello World!")
    });
    
    let window = ApplicationWindow::builder()
    .application(app)
    .title("FPI - Joao Cosme")
    .child(&button)
    .build();

    window.present();
}