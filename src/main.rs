mod drag;
mod drop;
mod ffi;

use ffi::GPrint;

use gio::prelude::*;
use gtk::prelude::*;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "1.0", author = "Poly <marynczak.bartlomiej@gmail.com>", setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(Clap)]
enum Commands {
    #[clap(version = "1.0", setting = AppSettings::ColoredHelp)]
    Drag(DragCommand),
    #[clap(version = "1.0", setting = AppSettings::ColoredHelp)]
    Drop(DropCommand),
}

/// Drag source mode
#[derive(Clap)]
struct DragCommand {
    /// List of files
    source: Vec<String>,
}
/// Drop target mode
#[derive(Clap)]
struct DropCommand;

fn main() {
    let application = gtk::Application::new(
        Some("io.github.polymeiles.dady"),
        gio::ApplicationFlags::HANDLES_COMMAND_LINE,
    )
    .expect("Initialization failed...");

    application.connect_command_line(|app, cli| {
        let opts = if cli.get_is_remote() {
            let opts: Result<Opts, clap::Error> = Opts::try_parse_from(cli.get_arguments());

            if let Err(err) = &opts {
                cli.print_err(format!("{}", err));
            }

            opts.ok()
        } else {
            let opts = Opts::parse_from(cli.get_arguments());
            Some(opts)
        };

        if let Some(opts) = opts {
            let win = match &opts.subcmd {
                Commands::Drag(c) => drag::build(app, &c.source),
                Commands::Drop(_) => drop::build(app),
            };

            win.connect_key_press_event(move |win, key| {
                let kv = key.get_keyval();

                use gdk::keys::constants::{q, Escape};
                if kv == Escape || kv == q {
                    win.close();
                }

                gtk::Inhibit(true)
            });
        }

        0
    });

    let args: Vec<String> = std::env::args().collect();
    application.run(&args);
}
