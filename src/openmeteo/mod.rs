mod weathercodes;

use serde::Deserialize;
use weathercodes::WeatherCode;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub current: WeatherData,
}

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    pub time: u64,
    pub interval: u16,
    pub temperature_2m: f64,
    pub relative_humidity_2m: i64,
    pub apparent_temperature: f64,
    pub precipitation: f64,
    pub weather_code: WeatherCode,
    pub wind_speed_10m: f64,
}

pub fn get_current_weather(latitude: f64, longtitude: f64, time_zone: &str) -> WeatherData {
    let url = format!("https://api.open-meteo.com/v1/forecast?\
        latitude={}&longitude={}\
        &timezone={}&timeformat=unixtime\
        &current=temperature_2m,relative_humidity_2m,apparent_temperature,\
        precipitation,weather_code,wind_speed_10m",
        latitude, longtitude, time_zone);

    let response = reqwest::blocking::get(url).unwrap();
    let weather_response: WeatherResponse = response.json().unwrap();
    weather_response.current
}
