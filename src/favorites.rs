use std::{collections::HashSet, fs::{File, OpenOptions, create_dir_all}, io::{BufReader, BufWriter}, path::Path};


const FAVORITES_FILE: &str = "~/.config/hydro/favorites.json";

pub fn load() -> Result<HashSet<u16>, String> {
    let file = open_favs_file()?;

    let reader = BufReader::new(file);
    let favs: Vec<u16> = serde_json::from_reader(reader)
        .unwrap_or_default();

    Ok(HashSet::from_iter(favs))
}

pub fn save(favs: &HashSet<u16>) -> Result<(), String> {
    let favs = Vec::from_iter(favs);

    let file = open_favs_file()?;

    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &favs)
        .map_err(|err| format!("Could not write file {}: {}", FAVORITES_FILE, err))?;

    Ok(())
}

fn open_favs_file() -> Result<File, String> {
    let path = Path::new(FAVORITES_FILE);
    let dir = path.parent().unwrap();

    if !dir.exists() {
        create_dir_all(dir)
            .map_err(|err| format!("Could not create directory {}: {}", dir.display(), err))?;
    }

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .map_err(|err| format!("Could not open file {}: {}", FAVORITES_FILE, err))
}