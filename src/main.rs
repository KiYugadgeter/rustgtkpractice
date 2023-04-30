use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button, Label, Orientation};
use std::cell::Cell;
use std::rc::Rc;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let number = Rc::new(Cell::new(0));
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(20)
        .margin_bottom(10)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(20)
        .margin_bottom(10)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number_copy = number.clone();
    let number_label = Label::builder()
        .label(number_copy.get().to_string())
        .build();
    let nl: &'static Label = &number_label;
    button_increase.connect_clicked(move |_| {
        number_copy.set(number_copy.get() + 1);
        let s: &str = &((number_copy.get() + 1).to_string());
        nl.set_text(s);
    });
    button_decrease.connect_clicked(move |_| {
        number.set(number.get() - 1);
        let s: &str = &((number.get() - 1).to_string());
        nl.set_text(s);
    });

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&number_label);
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My Gtk App")
        .child(&gtk_box)
        .build();
    window.present()
}
