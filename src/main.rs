use std::io::Read;

use easy_scraper::Pattern;

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
    url: String,
}

fn main() {
    let url = format!("{}{}", DOMAIN, LIST);
    let body = scrape(&url);
    let body = body.unwrap();
    let stations = extract(&body);
    for station in stations {
        println!("{:?}", station);
    }
}

fn extract(body: &str) -> Vec<Station> {
    let mut stations = Vec::new();

    let pat = Pattern::new(r#"
    <table subseq>
        <tbody>
            <tr>
                <td>{{id}}</td>
                <td>{{name:*}}</td>
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
        let id = m["id"].parse::<u16>().unwrap();
        let url = format!("{}{}.html", DOMAIN, id);
        let station = Station { id, url };
        stations.push(station);
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
