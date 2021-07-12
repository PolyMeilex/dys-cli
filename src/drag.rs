use gio::prelude::*;
use gtk::prelude::*;

fn build_files_box(
    win: gtk::ApplicationWindow,
    files: &[gio::File],
    targets: &[gtk::TargetEntry],
) -> gtk::Box {
    let vbox = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .build();

    for file in files.iter() {
        let base = file.basename().expect("Path Error");
        let base = base.to_str().expect("Path Error");

        let button = gtk::ButtonBuilder::new().expand(true).parent(&vbox).build();

        button.drag_source_set(
            gdk::ModifierType::BUTTON1_MASK,
            &targets,
            gdk::DragAction::COPY | gdk::DragAction::LINK | gdk::DragAction::ASK,
        );

        let uri = file.uri().to_string();
        button.connect_drag_data_get(move |_, _, s, _, _| {
            s.set_uris(&[&uri]);
        });

        let win = win.clone();
        button.connect_drag_end(move |_, _| {
            win.close();
        });

        let hbox = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(6)
            .parent(&button)
            .build();

        let icon = file
            .query_info(
                "*",
                gio::FileQueryInfoFlags::empty(),
                Some(&gio::Cancellable::new()),
            )
            .ok()
            .and_then(|info| info.icon());

        if let Some(icon) = icon {
            gtk::ImageBuilder::new()
                .gicon(&icon)
                .pixel_size(48)
                .parent(&hbox)
                .build();
        }

        gtk::LabelBuilder::new().label(&base).parent(&hbox).build();
    }

    vbox
}

fn build_all_files_button(
    win: gtk::ApplicationWindow,
    files: &[gio::File],
    targets: &[gtk::TargetEntry],
) -> gtk::ToggleButton {
    let button = gtk::ToggleButtonBuilder::new().expand(true).build();

    // Button
    {
        // Button content
        {
            let hbox = gtk::BoxBuilder::new()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(6)
                .parent(&button)
                .build();

            gtk::ImageBuilder::new()
                .icon_name("edit-select-all")
                .pixel_size(48)
                .parent(&hbox)
                .build();

            gtk::LabelBuilder::new()
                .label("Drag All")
                .parent(&hbox)
                .build();
        }

        let uris: Vec<_> = files.iter().map(|file| file.uri().to_string()).collect();

        button.connect_drag_data_get(move |_, _, s, _, _| {
            let refs: Vec<&str> = uris.iter().map(|u| u.as_str()).collect();
            s.set_uris(&refs);
        });

        button.connect_drag_end(move |_, _| {
            win.close();
        });

        button.drag_source_set(
            gdk::ModifierType::BUTTON1_MASK,
            &targets,
            gdk::DragAction::COPY | gdk::DragAction::LINK | gdk::DragAction::ASK,
        );
    }

    button
}

pub fn build(application: &gtk::Application, sources: &Vec<String>) -> gtk::ApplicationWindow {
    #[cfg(feature = "dock")]
    let type_hint = gdk::WindowTypeHint::Dock;
    #[cfg(not(feature = "dock"))]
    let type_hint = gdk::WindowTypeHint::Dialog;

    let window = gtk::ApplicationWindowBuilder::new()
        .application(application)
        .title("DYS")
        .default_width(200)
        .type_hint(type_hint)
        .window_position(gtk::WindowPosition::Center)
        .events(gdk::EventMask::KEY_PRESS_MASK)
        .build();

    let files: Vec<_> = sources
        .into_iter()
        .map(|s| gio::File::for_path(s))
        .collect();

    let targets = vec![gtk::TargetEntry::new(
        "text/uri-list",
        gtk::TargetFlags::SAME_APP,
        0,
    )];

    if files.len() == 1 {
        let files_box = build_files_box(window.clone(), &files, &targets);
        window.add(&files_box);
        window.show_all();
    } else if files.len() > 1 {
        let root = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .expand(true)
            .spacing(6)
            .parent(&window)
            .build();

        let files_button = build_all_files_button(window.clone(), &files, &targets);
        let files_box = build_files_box(window.clone(), &files, &targets);

        {
            let window = window.clone();
            let files_box = files_box.clone();
            files_button.connect_clicked(move |btn| {
                if btn.is_active() {
                    files_box.show_all();
                } else {
                    files_box.hide();
                    window.resize(200, 58);
                }
            });

            files_button.show_all();
        }

        root.add(&files_button);
        root.add(&files_box);

        root.show();
        window.show();
    } else {
        // For some reason app will not exit if window is not shown first
        window.show();

        window.close();
    }

    window
}
