mod drag;
mod drop;
mod ffi;

use ffi::GPrint;

use gio::prelude::*;
use gtk::prelude::*;

use clap::Parser as _;

#[derive(clap::Parser)]
#[clap(version = "1.0", author = "Poly <marynczak.bartlomiej@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(clap::Parser)]
enum Commands {
    #[clap(version = "1.0")]
    Drag(DragCommand),
    #[clap(version = "1.0")]
    Drop(DropCommand),
}

/// Drag source mode
#[derive(clap::Parser)]
struct DragCommand {
    /// List of files
    source: Vec<String>,
}
/// Drop target mode
#[derive(clap::Parser)]
struct DropCommand;

fn main() {
    let application = adw::Application::new(
        Some("io.github.polymeiles.dys"),
        gio::ApplicationFlags::HANDLES_COMMAND_LINE,
    );

    application.connect_command_line(|app, cli| {
        let opts = if cli.is_remote() {
            let opts: Result<Opts, clap::Error> = Opts::try_parse_from(cli.arguments());

            if let Err(err) = &opts {
                cli.print_err(err.to_string());
            }

            opts.ok()
        } else {
            let opts = Opts::parse_from(cli.arguments());
            Some(opts)
        };

        if let Some(opts) = opts {
            let win = match &opts.subcmd {
                Commands::Drag(c) => drag::build(app, &c.source),
                Commands::Drop(_) => drop::build(app),
            };

            let controller = gtk::EventControllerKey::new();
            controller.connect_key_pressed({
                let win = win.clone();
                move |_, key, _, _| {
                    if key == gdk::Key::Escape || key == gdk::Key::Q {
                        win.close();
                        glib::Propagation::Stop
                    } else {
                        glib::Propagation::Proceed
                    }
                }
            });
            win.add_controller(controller);
        }

        glib::ExitCode::SUCCESS
    });

    application.run();
}
