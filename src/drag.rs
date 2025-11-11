use gio::prelude::*;
use gtk::prelude::*;

fn build_drag_source(win: gtk::ApplicationWindow, files: &[gio::File]) -> gtk::DragSource {
    let files = gdk::FileList::from_array(files);
    let content = gdk::ContentProvider::for_value(&files.to_value());

    let source = gtk::DragSource::new();
    source.connect_drag_end(move |_, _, _| {
        win.close();
    });

    source.set_content(Some(&content));
    source
}

fn build_file_row(win: gtk::ApplicationWindow, file: &gio::File) -> gtk::Button {
    let base = file.basename().expect("Path Error");
    let base = base.to_str().expect("Path Error");

    let button = gtk::Button::builder().build();

    let icon = file
        .query_info(
            "*",
            gio::FileQueryInfoFlags::empty(),
            Some(&gio::Cancellable::new()),
        )
        .ok()
        .and_then(|info| info.icon());

    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .build();

    if let Some(icon) = icon {
        hbox.append(&gtk::Image::builder().gicon(&icon).pixel_size(48).build());
    }

    hbox.append(&gtk::Label::builder().label(base).build());
    button.set_child(Some(&hbox));

    button.add_controller(build_drag_source(win, std::slice::from_ref(file)));
    button
}

fn build_all_files_button(win: gtk::ApplicationWindow, files: &[gio::File]) -> gtk::ToggleButton {
    let button = gtk::ToggleButton::builder().build();

    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .build();

    button.set_child(Some(&hbox));

    hbox.append(
        &gtk::Image::builder()
            .icon_name("edit-select-all")
            .pixel_size(48)
            .build(),
    );

    hbox.append(&gtk::Label::builder().label("Drag All").build());

    button.add_controller(build_drag_source(win, files));

    button
}

pub fn build(application: &gtk::Application, sources: &[String]) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("DYS")
        .default_width(200)
        .build();

    let files: Vec<_> = sources.iter().map(gio::File::for_path).collect();

    if files.len() == 1 {
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        vbox.append(&build_file_row(window.clone(), &files[0]));

        window.set_child(Some(&vbox));
        window.present();
    } else if files.len() > 1 {
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        vbox.append(&build_all_files_button(window.clone(), &files));

        for file in files.iter() {
            vbox.append(&build_file_row(window.clone(), file));
        }

        window.set_child(Some(&vbox));
        window.present();
    } else {
        window.application().unwrap().quit();
    }

    window
}
