# Hydro

A simple rust CLI to retrieve information about water temperature from the [Federal Office for the environment](https://www.hydrodaten.admin.ch/en/water-temperature-table.html) website.

## Installation

The easiest way install it is to use cargo:
```bash
cargo install --git https://github.com/octogradiste/hydro.git
```
Note that this will build the full project and might take a few seconds.

## Usage

To list all available stations use the following command:
```bash
hydro list
```

To list the first 10 stations use the following command:
```bash
hydro list --first 10
```

To filter by name and water use the following flags:
```bash
hydro list --name "Bern" --water "Aare"
```
Those filters are case-insensitive and will match any station that contains the given string.

To get the water temperature for a specific station use the following command:
```bash
hydro get <ID>
```

To add one or more station to your favorites use the following command:
```bash
hydro fav add <IDS>...
```

To remove one or more station from your favorites use the following command:
```bash
hydro fav rm <IDS>...
```

To list all your favorite stations use the following command:
```bash
hydro fav
```
