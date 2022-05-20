use gio::Settings;
use glib::signal::Inhibit;
use gtk4::{gio, glib};
use gtk4::{subclass::prelude::*, ApplicationWindow};

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
    pub url: Url,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = ApplicationWindow;

    fn new() -> Self {
        Self { url: Url::new() }
    }
}
impl ObjectImpl for Window {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        // Load latest window state
        obj.load_window_size();
    }
}
impl WidgetImpl for Window {}
impl WindowImpl for Window {
    // Save window state right before the window will be closed
    fn close_request(&self, obj: &Self::Type) -> Inhibit {
        // Save window size
        obj.save_window_size().expect("Failed to save window state");

        // Don't inhibit the default handler
        Inhibit(false)
    }
}
impl ApplicationWindowImpl for Window {}
