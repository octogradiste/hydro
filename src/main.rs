use hydro::{display_stations, display_info};
use hydro::scraping::get_stations;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "hydro", version = "0.1.0", author = "octogradiste")]
#[command(about = "A simple rust CLI to retrieve information from hydrodaten.admin.ch")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List all stations
    List {
        /// Display the FIRST stations
        #[arg(short, long)]
        first: Option<usize>,

        /// Display the station URL
        #[arg(short)]
        url: bool,
    },

    /// Information about a station
    Info {
        /// The station ID
        #[arg(required = true)]
        id: u16,
    }
}

fn main() {
    let args = CLI::parse();

    let stations = get_stations();

    match stations {
        Err(err) => eprintln!("{err}"),
        Ok(stations) =>  match args.command {
            Commands::List { first, url } => display_stations(stations, first, url),
            Commands::Info { id } => display_info(stations, id),
        },
    }
}
