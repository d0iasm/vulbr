use glib::subclass::InitializingObject;
use glib::subclass::Signal;
use glib::{ParamFlags, ParamSpec, ParamSpecString, Value};
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gio, glib, ApplicationWindow, CompositeTemplate, Entry, ListView};
use once_cell::sync::Lazy;
use std::cell::RefCell;

#[derive(CompositeTemplate, Default)]
#[template(file = "window.ui")]
pub struct BrowserWindow {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    url: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for BrowserWindow {
    const NAME: &'static str = "BrowserWindow";
    type Type = super::BrowserWindow;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BrowserWindow {
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        self.parent_constructed(obj);

        obj.setup_callbacks();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder(
                "signal-test",
                &[String::static_type().into()],
                <()>::static_type().into(),
            )
            .build()]
        });
        SIGNALS.as_ref()
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecString::new(
                // Name
                "url",
                // Nickname
                "url",
                // Short description
                "url that a user types. This is updated when an enter key is pressed.",
                // Default value
                None,
                // The property can be read and written to
                ParamFlags::READWRITE,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        println!("set_property");
        match pspec.name() {
            "url" => {
                let input = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.url.replace(input);
            }
            _ => unimplemented!(),
        }
        println!("set_property {}", self.url.borrow());
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "url" => self.url.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}
impl WidgetImpl for BrowserWindow {}
impl WindowImpl for BrowserWindow {}
impl ApplicationWindowImpl for BrowserWindow {}
