use std::io::Read;
use clap::{Parser, Subcommand};
use easy_scraper::Pattern;
use regex::Regex;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

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
#[command(about = "Simple rust CLI to retrieve information from hydrodaten.admin.ch")]
struct CLI {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    // List all stations
    List
}

fn main() {
    let args = CLI::parse();

    let url = format!("{}{}", DOMAIN, LIST);
    let body = scrape(&url);
    let body = body.unwrap();
    let stations = extract(&body);

    let table: Vec<_> = stations.into_iter().map(|station| {
        vec![
            station.id.cell().justify(Justify::Right),
            station.name.cell(),
            station.water.cell(),
            station.url.cell(),
        ]
    }).collect();

    let table = table.table().title(vec![
        "ID".cell().bold(true),
        "Name".cell().bold(true),
        "Water".cell().bold(true),
        "URL".cell().bold(true),
    ]).bold(true);

    let table = table.display().unwrap();

    match args.command {
        Commands::List => println!("{}", table),
    }
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
