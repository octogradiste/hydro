use scraping::Station;

use cli_table::{format::Justify, Cell, Style, Table};

pub mod scraping;

pub fn display_stations(stations: Vec<Station>, first: Option<usize>, url: bool) {
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