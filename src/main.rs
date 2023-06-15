use std::collections::HashSet;

use hydro::favorites::{load, save};
use hydro::{display_stations, display_info, filter_stations};
use hydro::scraping::{get_stations, Station};

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
    },

    /// Manage your favorite stations
    /// 
    /// Use the `add` and `remove` subcommands to manage your favorite stations.
    /// If no subcommand is provided, it will display your favorite stations.
    /// 
    /// Your favorite stations are stored in `~/.config/hydro/favorites.json`.
    Fav {
        #[command(subcommand)]
        command: Option<FavCommands>,

        /// Display the station URL
        #[arg(short)]
        url: bool,
    },
}

#[derive(Debug, Subcommand)]
enum FavCommands {
    /// Adds one or more stations to your favorites
    Add {
        /// The station IDs
        #[arg(required = true)]
        ids: Vec<u16>,
    },
    /// Removes one or more stations from your favorites
    Rm {
        /// The station IDs
        #[arg(required = true)]
        ids: Vec<u16>,
    },
}

fn main() {
    let args = CLI::parse();

    let stations = get_stations();

    match stations {
        Err(err) => eprintln!("{err}"),
        Ok(mut stations) =>  match args.command {
            Commands::List { first, name, water, url } => {
                filter_stations(&mut stations, first, name, water);
                display_stations(&stations, url);
            },
            Commands::Get { id } => display_info(stations, id),
            Commands::Fav { command, url} => {
                match load() {
                    Err(err) => eprintln!("{err}"),
                    Ok(mut favs) => fav_handler(&mut stations, &mut favs, command, url),
                }
            },
        },
    }
}

fn fav_handler(stations: &mut Vec<Station>, favs: &mut HashSet<u16>, command: Option<FavCommands>, url: bool) {
    match command {
        Some(FavCommands::Add { ids }) => {
            favs.extend(ids);
            if let Err(err) = save(favs) {
                eprintln!("{err}");
            }
        },
        Some(FavCommands::Rm { ids }) => {
            favs.retain(|id| !ids.contains(id));
            if let Err(err) = save(favs) {
                eprintln!("{err}");
            }
        },
        None =>  {
            stations.retain(|station| favs.contains(&station.id));
            display_stations(stations, url);
        },
    }
}