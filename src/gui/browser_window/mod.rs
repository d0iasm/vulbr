mod imp;

use glib::Object;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::Application;
use gtk4::{gio, glib};

glib::wrapper! {
    pub struct BrowserWindow(ObjectSubclass<imp::BrowserWindow>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Accessible, gtk4::Buildable,
                    gtk4::ConstraintTarget, gtk4::Native, gtk4::Root, gtk4::ShortcutManager;
}

impl BrowserWindow {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create `BrowserWindow`.")
    }
}
