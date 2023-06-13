use hydro::display_stations;
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

        /// Display the station's URL
        #[arg(short)]
        url: bool,
    },
}

fn main() {
    let args = CLI::parse();

    let stations = get_stations().unwrap();

    match args.command {
        Commands::List { first, url } => display_stations(stations, first, url),
    }
}
