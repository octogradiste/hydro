
use std::{io::Read, fmt::{Display, Formatter, self}};
use easy_scraper::Pattern;
use regex::Regex;

const DOMAIN: &str = "https://www.hydrodaten.admin.ch/de/";
const TEMPERATURE_TABLE: &str = "tabelle-der-wassertemperaturen.html";

#[derive(Debug)]
pub enum ScrapingError {
    ServerNotReachable,
    BadResponse,
    CorruptedBody,
}

impl Display for ScrapingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ScrapingError::ServerNotReachable => write!(f, "Server not reachable."),
            ScrapingError::BadResponse => write!(f, "Bad response."),
            ScrapingError::CorruptedBody => write!(f, "Corrupted body."),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Station {
    pub id: u16,
    pub name: String,
    pub water: String,
    pub measurement: f32,
    pub max: f32,
    pub time: String,
    pub url: String,
}

pub fn get_stations() -> Result<Vec<Station>, ScrapingError> {
    let url = format!("{}{}", DOMAIN, TEMPERATURE_TABLE);
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
        let id = m["id"].parse::<u16>().unwrap();

        let full_name = remove_tags(&m["full_name"]);
        let (water, name) = full_name.split_once('-').unwrap();
        let name = name.trim().to_string();
        let water = water.trim().to_string();

        let measurement = m["measurement"].parse::<f32>().unwrap();
        let max = m["max"].parse::<f32>().unwrap();
        let time = m["datetime"].trim().to_string();

        let url = format!("{}{}.html", DOMAIN, id);

        let station = Station { id, name, water, measurement, max, time, url };
        stations.push(station);
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

#[cfg(test)]
mod remove_tags_tests {
    use super::remove_tags;

    #[test]
    fn should_return_same_string_when_has_no_tags() {
        let body = "A string without tags.";
        let result = remove_tags(body);
        assert_eq!(result, body);
    }

    #[test]
    fn should_correctly_remove_tags() {
        let body = "<sometag>and</closingtag>";
        let result = remove_tags(body);
        assert_eq!(result, "and");
    }
}

#[cfg(test)]
mod extract_tests {
    use crate::scraping::Station;

    use super::extract;

    #[test]
    fn should_correctly_return_all_stations() {
        let body = r#"
<table>
    <tbody>
        <tr data-station-id="2135" data-station-name="aare bern schonau">
            <td>
                2135
            </td>
            <td>
                <a href="/de/2135.html"><strong>Aare</strong> - Bern, Schönau</a>
            </td>
            <td>
                14.06.2023 10:20
            </td>
            <td class="text-right">
                16.7
            </td>
            <td class="text-right">
                18.5
            </td>
            <td>
                °C
            </td>
            <td>
                <p><a href="/lhg/az/dwh/plots/surface/7day/2135_7.PDF" target="_blank"><span class="glyphicon glyphicon-stats glyphicon-spaced-r"></span>7 Tage</a></p>
            </td>
        </tr>
    </tbody>
</table>
        "#;

        let stations = extract(body);
        let expected = vec![
            Station {
                id: 2135,
                name: "Bern, Schönau".to_string(),
                water: "Aare".to_string(),
                measurement: 16.7,
                max: 18.5,
                time: "14.06.2023 10:20".to_string(),
                url: "https://www.hydrodaten.admin.ch/de/2135.html".to_string(),
            }
        ];

        assert_eq!(stations, expected);
    }
}