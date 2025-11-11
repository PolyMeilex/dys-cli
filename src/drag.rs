use adw::prelude::*;

fn build_drag_source(win: adw::ApplicationWindow, files: &[gio::File]) -> gtk::DragSource {
    let files = gdk::FileList::from_array(files);
    let content = gdk::ContentProvider::for_value(&files.to_value());

    let source = gtk::DragSource::new();
    source.connect_drag_end(move |_, _, _| {
        win.close();
    });

    source.set_content(Some(&content));
    source
}

fn build_file_row(win: adw::ApplicationWindow, file: &gio::File) -> adw::ActionRow {
    let base = file.basename().expect("Path Error");
    let title = base.to_str().expect("Path Error");
    let row = adw::ActionRow::builder()
        .title(title)
        .activatable(true)
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
        row.add_prefix(&gtk::Image::builder().gicon(&icon).pixel_size(48).build());
    }

    row.add_controller(build_drag_source(win, std::slice::from_ref(file)));
    row
}

fn build_all_files_button(
    win: adw::ApplicationWindow,
    files: &[gio::File],
) -> adw::PreferencesGroup {
    let grp = adw::PreferencesGroup::new();

    let row = adw::ButtonRow::builder().title("Drag All").build();

    row.set_start_icon_name(Some("edit-select-all"));
    row.add_controller(build_drag_source(win, files));

    grp.add(&row);
    grp
}

pub fn build(application: &adw::Application, sources: &[String]) -> adw::ApplicationWindow {
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title("DYS")
        .default_width(200)
        .build();

    let files: Vec<_> = sources.iter().map(gio::File::for_path).collect();

    if files.len() == 1 {
        let grp = adw::PreferencesGroup::new();
        grp.add(&build_file_row(window.clone(), &files[0]));

        window.set_content(Some(&grp));
        window.present();
    } else if files.len() > 1 {
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .margin_bottom(10)
            .margin_top(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        let grp = adw::PreferencesGroup::new();

        for file in files.iter() {
            grp.add(&build_file_row(window.clone(), file));
        }

        vbox.append(&build_all_files_button(window.clone(), &files));
        vbox.append(&grp);

        window.set_content(Some(&vbox));
        window.present();
    } else {
        window.application().unwrap().quit();
    }

    window
}
