mod browser_window;

use browser_window::BrowserWindow;
use gio::SimpleAction;
use glib::clone;
use glib::closure_local;
use glib::subclass::Signal;
use gtk4::glib::value::ToValue;
use gtk4::prelude::*;
use gtk4::{gio, glib};
use gtk4::{
    Align, Application, Box, HeaderBar, Label, Orientation, SearchBar, SearchEntry, ToggleButton,
};

pub fn start_browser_window() -> Application {
    let app = Application::builder().application_id("vulbr").build();

    app.connect_activate(build_ui);

    /*
    app.connect_closure(
        "clicked",
        false,
        closure_local!(move |_button: i32| { println!("Clicked!") }),
    );
    */
    /*
    let action_close = SimpleAction::new("close", None);
    action_close.connect_activate(clone!(@weak window => move |_, _| {
    window.close();
        }));
    window.add_action(&action_close);
    */

    app.run();

    app
}

fn build_ui(app: &Application) {
    let window = BrowserWindow::new(app);
    window.set_default_size(1280, 800);
    window.set_title(Some("vulbr"));

    let header_bar = HeaderBar::new();
    window.set_titlebar(Some(&header_bar));

    let search_button = ToggleButton::new();
    search_button.set_icon_name("system-search-symbolic");
    search_button.set_active(true);
    header_bar.pack_end(&search_button);

    let container = Box::new(Orientation::Vertical, 6);
    window.set_child(Some(&container));

    let search_bar = SearchBar::builder()
        .valign(Align::Start)
        .key_capture_widget(&window)
        .build();

    container.append(&search_bar);

    search_button
        .bind_property("active", &search_bar, "search-mode-enabled")
        .flags(glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
        .build();

    let entry = SearchEntry::new();
    entry.set_hexpand(true);
    search_bar.set_child(Some(&entry));

    let label = Label::builder()
        .label("Type to start search")
        .vexpand(true)
        .halign(Align::Center)
        .valign(Align::Center)
        .css_classes(vec!["large-title".to_string()])
        .build();

    container.append(&label);

    entry.connect_activate(clone!(@weak label, @weak window => move |entry| {
        println!("connect_activate");
        window.set_property("url", entry.text());
        let n = 42.to_value();
        window.emit_by_name::<()>("signal-test", &[&n]);
    }));

    entry.connect_search_changed(clone!(@weak label => move |entry| {
        println!("entry.connect_search_changed {:?}", entry.text());
        if entry.text() != "" {
            label.set_text(&entry.text());
        } else {
            label.set_text("Type to start search");
        }
    }));

    window.connect_closure(
        "signal-test",
        false,
        closure_local!(move |_w: BrowserWindow, number: i32| {
            println!("get a signal !!!!!!!!!!! {}", number);
        }),
    );

    window.show();
}
