//! Smart Connect for Linux: GTK4/libadwaita front-end for `smartconnectd`.

mod apps;
mod daemon;
mod crosscontrol;
mod display;
mod files;
mod icons;
mod lastdevice;
mod lifecycle;
mod mirror;
mod palette;
mod prefs;
mod setup;
mod share;
mod theme;
mod tray;
mod window;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;
use gtk::glib;

const APP_ID: &str = "dev.smartconnect.Linux";

fn main() -> glib::ExitCode {
    // `--background` starts the app hidden (tray only), for the autostart entry. GApplication never
    // sees the flag: we strip it and forward the rest of argv.
    let mut forwarded: Vec<String> = Vec::new();
    let mut background = false;
    for arg in std::env::args() {
        if arg == "--background" {
            background = true;
        } else {
            forwarded.push(arg);
        }
    }

    let app = adw::Application::builder().application_id(APP_ID).build();
    let ui: Rc<RefCell<Option<Rc<window::Ui>>>> = Rc::default();
    // Only the first activation honours `--background`; launching the app again always shows it.
    let first_activation = Rc::new(Cell::new(true));

    app.connect_startup(|_| {
        theme::apply_theme();
    });

    let slot = ui.clone();
    app.connect_activate(move |app| {
        let is_first = first_activation.replace(false);
        // Started from the AppImage file: set it up first (or open the installed copy).
        if is_first && slot.borrow().is_none() {
            if setup::installed_same_build() {
                setup::open_installed_and_quit(app);
                return;
            }
            if let Some((appimage, update)) = setup::pending_install() {
                let (slot, app_c) = (slot.clone(), app.clone());
                setup::welcome(app, appimage, update, move || {
                    let ui = window::Ui::new(&app_c);
                    ui.start();
                    ui.present();
                    *slot.borrow_mut() = Some(ui);
                });
                return;
            }
        }
        let mut slot = slot.borrow_mut();
        let ui = slot.get_or_insert_with(|| {
            let ui = window::Ui::new(app);
            ui.start();
            ui
        });
        // Stay hidden to the tray only when launched with `--background` on the very first
        // activation; any later launch (or a normal launch) presents the window.
        if !(is_first && background) {
            ui.present();
        }
    });

    let slot = ui.clone();
    app.connect_shutdown(move |_| {
        if let Some(ui) = slot.borrow().as_ref() {
            ui.shutdown();
        }
    });

    app.run_with_args(&forwarded)
}
