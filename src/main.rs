use std::{io::Read};
use clap::{Parser, Subcommand};
use easy_scraper::Pattern;
use regex::Regex;
use cli_table::{format::Justify, Cell, Style, Table};

const DOMAIN: &str = "https://www.hydrodaten.admin.ch/de/";
const LIST: &str = "stationen-und-daten.html";

#[derive(Debug)]
enum ScrapeError {
    ServerNotReachable,
    BadResponse,
    CorruptedBody,
}

#[derive(Debug)]
struct Station {
    id: u16,
    name: String,
    water: String,
    url: String,
}

#[derive(Debug, Parser)]
#[command(name = "hydro", version = "0.1.0", author = "octogradiste")]
#[command(about = "A simple rust CLI to retrieve information from hydrodaten.admin.ch")]
struct CLI {
    #[command(subcommand)]
    command: Commands
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

    let url = format!("{}{}", DOMAIN, LIST);
    let body = scrape(&url);
    let body = body.unwrap();
    let stations = extract(&body);

    match args.command {
        Commands::List { first, url } => {
            print_stations(stations, first, url)
        }
    }
}

fn print_stations(stations: Vec<Station>, first: Option<usize>, url: bool) {
    let table = stations.into_iter().map(|station| {
        let mut row = vec![
            station.id.cell().justify(Justify::Right),
            station.name.cell(),
            station.water.cell(),
        ];
        if url {
            row.push(station.url.cell());
        }
        row
    }).take(first.unwrap_or(usize::MAX));

    let mut titles = vec![
        "ID".cell().bold(true),
        "Name".cell().bold(true),
        "Water".cell().bold(true),
    ];
    if url {
        titles.push("URL".cell().bold(true));
    }

    let table = table.table().title(titles).bold(true);
    let table = table.display().unwrap();
    println!("{}", table);
}

fn remove_tags(body: &str) -> String {
    let regex = Regex::new(r"<[^>]*>").unwrap();
    regex.replace_all(body, "").to_string()
}

fn extract(body: &str) -> Vec<Station> {
    let mut stations = Vec::new();

    let pat = Pattern::new(r#"
    <table subseq>
        <tbody>
            <tr>
                <td>{{id}}</td>
                <td><a>{{full_name:*}}</a></td>
                <td>{{datetime}}</td>
                <td>{{measurement}}</td>
                <td>{{max}}</td>
                <td>{{unit:*}}</td>
                <td>{{graphic:*}}</td>
            </tr>
        </tbody>
    </table>
    "#).unwrap();

    let ms = pat.matches(body);

    println!("Length: {}", ms.len());
    for m in ms {
        let full_name = remove_tags(&m["full_name"]);
        let full_name = full_name.split_once('-');

        if let Some((water, name)) = full_name  {
            let id = m["id"].parse::<u16>().unwrap();
            let name = name.trim().to_string();
            let water = water.trim().to_string();
            let url = format!("{}{}.html", DOMAIN, id);
            let station = Station { id, name, water, url };

            stations.push(station);
        }
    }

    stations
}

fn scrape(url: &str) -> Result<String, ScrapeError> {
    let response = reqwest::blocking::get(url);
    match response {
        Err(_) => Err(ScrapeError::ServerNotReachable),
        Ok(mut response) => {
            let mut body = String::new();
            match response.status().is_success() {
                false => Err(ScrapeError::BadResponse),
                true => {
                    let read = response.read_to_string(&mut body);
                    match read {
                        Ok(_) => Ok(body),
                        Err(_) => Err(ScrapeError::CorruptedBody),
                    }
                },
            }
        },
    }
}
