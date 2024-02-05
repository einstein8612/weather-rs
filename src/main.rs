mod ipapi;
mod openmeteo;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ipapi::get_ip_geo_location;
use openmeteo::get_current_weather;

fn main() {
    let location = get_ip_geo_location();
    println!("Location:");
    println!("\tCountry: {}", location.country);
    println!("\tCity: {}/{}", location.region_name, location.city);
    println!("\tLatitude: {}", location.lat);
    println!("\tLongtitude: {}", location.lon);
    println!("");

    let weather = get_current_weather(location.lat, location.lon, &location.timezone);
    println!("Weather:");
    println!("\tForecast: {}", weather.weather_code);
    println!("\tTemperature: {}°C", weather.temperature_2m);
    println!("\tApparent Temperature: {}°C", weather.apparent_temperature);
    println!("\tHumidity: {}%", weather.relative_humidity_2m);
    println!("\tPrecipitation: {}mm", weather.precipitation);
    println!("\tWind Speed: {}km/h", weather.wind_speed_10m);

    let last_updated = SystemTime::now()
        .duration_since(UNIX_EPOCH + Duration::from_secs(weather.time))
        .expect("Time went backwards");
    println!("\tLast updated: {} minutes ago", last_updated.as_secs()/60);
}
