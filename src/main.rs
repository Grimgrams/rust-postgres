mod db_functions;
mod account_functions;

use crate::db_functions::check::check_user_details;
use crate::db_functions::create_user::create_user;
use crate::db_functions::init_db::init_db;
use crate::account_functions::update_user::{update_email, update_password, update_username};
use crate::account_functions::login::login;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ButtonBox};

fn main() {
    let app = Application::builder()
        .application_id("com.GrimGrams.rust_postgres")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        let login_button = Button::with_label("Log In");
        login_button.connect_clicked(|_| {

        });


        win.show_all();
    });

    app.run();
}