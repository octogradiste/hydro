use hydro::{display_stations, display_info, filter_stations};
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

        /// Display stations containing NAME
        #[arg(short, long)]
        name: Option<String>,

        /// Display stations containing WATER
        #[arg(short, long)]
        water: Option<String>,

        /// Display the station URL
        #[arg(short)]
        url: bool,
    },

    /// Information about a station
    Get {
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
        Ok(mut stations) =>  match args.command {
            Commands::List { first, name, water, url } => {
                filter_stations(&mut stations, first, name, water);
                display_stations(stations, url);
            },
            Commands::Get { id } => display_info(stations, id),
        },
    }
}
