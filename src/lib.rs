use scraping::Station;

use cli_table::{format::{Justify, Border, Separator, VerticalLine}, Cell, Style, Table};

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
            format!("{:.1} °C", station.measurement).cell().justify(Justify::Right),
            format!("{:.1} °C", station.max).cell().justify(Justify::Right),
            station.time.cell(),
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
        "Measurement".cell().bold(true),
        "Max 24h".cell().bold(true),
        "Time".cell().bold(true),
    ];
    if url {
        titles.push("URL".cell().bold(true));
    }

    let table = table.table()
        .title(titles)
        .bold(true);

    let table = table.display().unwrap();
    println!("{}", table);
}

pub fn display_info(stations: Vec<Station>, id: u16) {
    let station = stations.into_iter().find(|station| station.id == id);

    if let Some(station) = station {
        let table = vec![
            vec!["ID".cell().bold(true).justify(Justify::Right), station.id.cell()],
            vec!["Name".cell().bold(true).justify(Justify::Right), station.name.cell()],
            vec!["Water".cell().bold(true).justify(Justify::Right), station.water.cell()],
            vec!["Measurement".cell().bold(true).justify(Justify::Right), format!("{:.1} °C", station.measurement).cell()],
            vec!["Max 24h".cell().bold(true).justify(Justify::Right), format!("{:.1} °C", station.max).cell()],
            vec!["Time".cell().bold(true).justify(Justify::Right), station.time.cell()],
            vec!["URL".cell().bold(true).justify(Justify::Right), station.url.cell()],
        ];
        let table = table.table()
            .bold(true)
            .border(Border::builder().build())
            .separator(Separator::builder().column(Some(VerticalLine::default())).build());

        let table = table.display().unwrap();
        println!("{}", table);
    } else {
        eprintln!("Station {id} not found.");
    }
}