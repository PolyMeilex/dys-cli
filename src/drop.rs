use gdk::FileList;
use gtk::prelude::*;

pub fn build(application: &gtk::Application) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("DYS")
        .default_height(200)
        .default_width(200)
        .build();

    let target = gtk::DropTarget::new(FileList::static_type(), gdk::DragAction::COPY);

    let root = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .spacing(6)
        .build();

    let image = gtk::Image::builder()
        .icon_name("insert-image")
        .pixel_size(48)
        .build();
    root.append(&image);

    let label = gtk::Label::builder().label("Drop here").build();
    root.append(&label);

    target.connect_drop(|_, value, _, _| {
        let Ok(list) = value.get::<FileList>() else {
            return false;
        };
        let files = list.files();

        let uris: Vec<_> = files.iter().map(|f| f.uri()).collect();

        let out = uris.join("\n");
        println!("{out}");

        true
    });

    window.add_controller(target);
    window.set_child(Some(&root));
    window.present();

    window
}
