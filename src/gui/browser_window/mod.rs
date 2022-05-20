mod imp;

use glib::Object;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::Application;
use gtk4::{gio, glib};

// ANCHOR: mod
glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Accessible, gtk4::Buildable,
                    gtk4::ConstraintTarget, gtk4::Native, gtk4::Root, gtk4::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create `Window`.")
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        //let url = &mut self.imp().url;

        //url.set_url("example.com".to_string());
        Ok(())
    }

    fn load_window_size(&self) {
        //let settings = &self.imp().settings;

        self.set_default_size(800i32, 1024i32);
    }
}
// ANCHOR_END: mod
