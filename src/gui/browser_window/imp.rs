use glib::{ParamFlags, ParamSpec, ParamSpecString, Value};
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::ApplicationWindow;
use once_cell::sync::Lazy;
use std::cell::Cell;

#[derive(Default)]
pub struct BrowserWindow {
    url: Cell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for BrowserWindow {
    const NAME: &'static str = "MyGtkAppBrowserWindow";
    type Type = super::BrowserWindow;
    type ParentType = ApplicationWindow;

    fn new() -> Self {
        Self { url: Cell::new(0) }
    }
}

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for BrowserWindow {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecString::new(
                // Name
                "url",
                // Nickname
                "url",
                // Short description
                "url",
                // Default value
                None,
                // The property can be read and written to
                ParamFlags::READWRITE,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "url" => {
                let input = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.url.replace(input);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "url" => self.url.get().to_value(),
            _ => unimplemented!(),
        }
    }
}
impl WidgetImpl for BrowserWindow {}
impl WindowImpl for BrowserWindow {}
impl ApplicationWindowImpl for BrowserWindow {}
