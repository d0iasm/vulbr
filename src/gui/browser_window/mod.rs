mod imp;

use glib::{clone, Object};
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gio, glib, Application};

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

    fn setup_callbacks(&self) {
        self.imp()
            .entry
            .connect_activate(clone!(@weak self as window => move |entry| {
                println!("entry connect_activate {:?}", entry);
                window.emit_by_name::<()>("start-handle-input", &[&entry.text().to_string()]);
                window.imp().entry.set_placeholder_text(Some(""));
            }));
    }

    pub fn get_content_area(&self) -> gtk4::Box {
        self.imp().content_area.get()
    }
}
