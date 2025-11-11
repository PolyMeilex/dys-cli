use gtk::prelude::*;

pub fn build(application: &gtk::Application) -> gtk::ApplicationWindow {
    #[cfg(feature = "dock")]
    let type_hint = gdk::WindowTypeHint::Dock;
    #[cfg(not(feature = "dock"))]
    let type_hint = gdk::WindowTypeHint::Dialog;

    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("DYS")
        .default_height(200)
        .default_width(200)
        .type_hint(type_hint)
        .window_position(gtk::WindowPosition::Center)
        .build();

    let targets = vec![
        gtk::TargetEntry::new("text/uri-list", gtk::TargetFlags::OTHER_APP, 0),
        gtk::TargetEntry::new("text/plain", gtk::TargetFlags::OTHER_APP, 0),
        gtk::TargetEntry::new("STRING", gtk::TargetFlags::OTHER_APP, 0),
    ];

    window.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);

    window.connect_drag_data_received(move |win, _, _, _, s, _, _| {
        match s.data_type().name().as_str() {
            "text/uri-list" => {
                let uris: Vec<String> = s.uris().into_iter().map(|uri| uri.to_string()).collect();

                let out = uris.join("\n");
                print!("{}", out);

                win.close();
            }
            _ => {
                let text = s.text().map(|gs| gs.to_string()).unwrap_or_default();
                print!("{}", text);

                win.close();
            }
        }
    });

    let root = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .expand(true)
        .spacing(6)
        .parent(&window)
        .build();

    let _image = gtk::Image::builder()
        .icon_name("insert-image")
        .pixel_size(48)
        .parent(&root)
        .build();

    let _label = gtk::Label::builder()
        .label("Drop here")
        .parent(&root)
        .build();

    window.show_all();

    window
}
