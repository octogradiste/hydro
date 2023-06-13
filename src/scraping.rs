
use std::{io::Read};
use easy_scraper::Pattern;
use regex::Regex;

const DOMAIN: &str = "https://www.hydrodaten.admin.ch/de/";
const LIST: &str = "stationen-und-daten.html";

#[derive(Debug)]
pub enum ScrapingError {
    ServerNotReachable,
    BadResponse,
    CorruptedBody,
}

#[derive(Debug)]
pub struct Station {
    pub id: u16,
    pub name: String,
    pub water: String,
    pub url: String,
}

pub fn get_stations() -> Result<Vec<Station>, ScrapingError> {
    let url = format!("{}{}", DOMAIN, LIST);
    let body = scrape(&url)?;
    let stations = extract(&body);

    Ok(stations)
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

fn remove_tags(body: &str) -> String {
    let regex = Regex::new(r"<[^>]*>").unwrap();
    regex.replace_all(body, "").to_string()
}

fn scrape(url: &str) -> Result<String, ScrapingError> {
    let response = reqwest::blocking::get(url);
    match response {
        Err(_) => Err(ScrapingError::ServerNotReachable),
        Ok(mut response) => {
            let mut body = String::new();
            match response.status().is_success() {
                false => Err(ScrapingError::BadResponse),
                true => {
                    let read = response.read_to_string(&mut body);
                    match read {
                        Ok(_) => Ok(body),
                        Err(_) => Err(ScrapingError::CorruptedBody),
                    }
                },
            }
        },
    }
}