use gtk4::{gio, glib};
use gtk4::{subclass::prelude::*, ApplicationWindow};
use std::cell::Cell;

pub struct Url {
    url: Option<String>,
}

impl Url {
    fn new() -> Self {
        Self { url: None }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }

    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }
}

pub struct Window {
    //pub url: mut Url,
    //pub url: Option<String>,
    pub url: Cell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkAppIntegerObject";
    type Type = super::Window;
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecInt::new(
                // Name
                "number",
                // Nickname
                "number",
                // Short description
                "number",
                // Minimum value
                i32::MIN,
                // Maximum value
                i32::MAX,
                // Default value
                0,
                // The property can be read and written to
                ParamFlags::READWRITE,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "number" => {
                let input_number = value.get().expect("The value needs to be of type `i32`.");
                self.number.replace(input_number);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "number" => self.number.get().to_value(),
            _ => unimplemented!(),
        }
    }
}
