use scraping::Station;

use cli_table::{format::{Justify, Border}, Cell, Style, Table};

pub mod scraping;

pub fn filter_stations(stations: &mut Vec<Station>, first: Option<usize>, name: Option<String>, water: Option<String>) {
    if let Some(name) = name {
        stations.retain(|station| station.name.to_lowercase().contains(&name.to_lowercase()));
    }

    if let Some(water) = water {
        stations.retain(|station| station.water.to_lowercase().contains(&water.to_lowercase()));
    }

    if let Some(first) = first {
        stations.truncate(first);
    }
}

pub fn display_stations(stations: Vec<Station>, url: bool) {
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
    });

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

pub fn display_info(stations: Vec<Station>, id: u16) {
    let station = stations.into_iter().find(|station| station.id == id);

    if let Some(station) = station {
        let table = vec![
            vec!["ID".cell().bold(true), station.id.cell()],
            vec!["Name".cell().bold(true), station.name.cell()],
            vec!["Water".cell().bold(true), station.water.cell()],
            vec!["URL".cell().bold(true), station.url.cell()],
        ].table().bold(true).border(Border::builder().build());

        let table = table.display().unwrap();
        println!("{}", table);
    } else {
        eprintln!("Station {id} not found.");
    }
}