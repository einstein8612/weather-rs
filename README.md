# weather-rs

![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/einstein8612/weather-rs)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/einstein8612/weather-rs/total)
![rust workflow](https://github.com/einstein8612/weather-rs/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## About 💭
Simple weather cli to check for weather based on current IP geolocation or an explicitly stated city. Written in Rust.

## Features ✨ 
* Shows the following information
  * Location
    * Country
    * City
    * Latitude
    * Longtitude
  * Weather
    * Forecast (Clear, Partly Cloudy, etc.)
    * Temperature
    * Apparent Temperature
    * Humidity
    * Precipitation
    * Wind Speed
  * Time of last update
* Gets weather by your IP's location
* Gets weather for a specific city

## Building 🔨
### For the following distributions, it's available for install here:
  - ArchLinux, you can use [aur](https://aur.archlinux.org/packages/weather-rs).

### For other distributions, or people that want to do a manual install:
#### Dependencies
In order to build from source, you must have the following installed:
  -  [rust](https://www.rust-lang.org/)

#### Build commands
```console
# Clone the source code
git clone git@github.com:einstein8612/weather-rs.git
cd weather-rs

# Build the binary
cargo build --release
# Optional: Move to bin, if you want to use it everywhere
sudo mv target/release/weather-rs /usr/bin/weather
```

## Usage 🕹️
### Example #1:
Showcases IP geolocation guessing current location.
```console
$ weather
╭─Location──────────────────────╮
│ Country: The Netherlands      │
│ City: North Holland/Amsterdam │
│ Latitude: 52.3667             │
│ Longtitude: 4.9               │
╰───────────────────────────────╯
╭─Weather──────────────────────╮
│ Forecast: Overcast           │
│ Temperature: 9.2°C           │
│ Apparent Temperature: 3.2°C  │
│ Humidity: 81%                │
│ Precipitation: 0mm           │
│ Wind Speed: 35km/h           │
│ Last updated: 4  minutes ago │
╰──────────────────────────────╯
```
### Example #2:
Showcases overwriting the city
```console
$ weather -c NYC
╭─Location──────────────────────╮
│ Country: United States        │
│ City: New York                │
│ Latitude: 40.71427            │
│ Longtitude: -74.00597         │
│ Population: 8175133           │
╰───────────────────────────────╯
╭─Weather───────────────────────╮
│ Forecast: Clear               │
│ Temperature: 1.2°C            │
│ Apparent Temperature: -4.1°C  │
│ Humidity: 41%                 │
│ Precipitation: 0mm            │
│ Wind Speed: 13.3km/h          │
│ Last updated: 6 minutes ago   │
╰───────────────────────────────╯
```

## APIs used
* For IP to geolocation, it uses [ip-api.com](https://ip-api.com/).
* For weather and city search, it uses [open-meteo.com](https://open-meteo.com/).