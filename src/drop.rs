use gtk::prelude::*;

pub fn build(application: &gtk::Application) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindowBuilder::new()
        .application(application)
        .title("Dady")
        .default_height(200)
        .default_width(200)
        .type_hint(gdk::WindowTypeHint::Dialog)
        .build();

    let targets = vec![
        gtk::TargetEntry::new("text/uri-list", gtk::TargetFlags::OTHER_APP, 0),
        gtk::TargetEntry::new("text/plain", gtk::TargetFlags::OTHER_APP, 0),
        gtk::TargetEntry::new("STRING", gtk::TargetFlags::OTHER_APP, 0),
    ];

    window.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);

    window.connect_drag_data_received(move |win, _, _, _, s, _, _| {
        match s.get_data_type().name().as_str() {
            "text/uri-list" => {
                let uris: Vec<String> = s
                    .get_uris()
                    .into_iter()
                    .map(|uri| uri.to_string())
                    .collect();

                let out = uris.join("\n");
                print!("{}", out);

                win.close();
            }
            _ => {
                let text = s.get_text().map(|gs| gs.to_string()).unwrap_or_default();
                print!("{}", text);

                win.close();
            }
        }
    });

    let root = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .expand(true)
        .spacing(6)
        .parent(&window)
        .build();

    let _image = gtk::ImageBuilder::new()
        .icon_name("insert-image")
        .pixel_size(48)
        .parent(&root)
        .build();

    let _label = gtk::LabelBuilder::new()
        .label("Drop here")
        .parent(&root)
        .build();

    window.show_all();

    window
}
