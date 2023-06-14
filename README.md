# Hydro

A simple rust CLI to retrieve information about water temperature from the [Federal Office for the environment](https://www.hydrodaten.admin.ch/en/water-temperature-table.html) website.

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
