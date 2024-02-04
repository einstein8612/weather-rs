mod ipapi;
mod openmeteo;

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

    let weather = get_current_weather(location.lat, location.lon);
    println!("Weather:");
    println!("\tForecast: {}", weather.weather_code);
    println!("\tTemperature: {}", weather.temperature_2m);
    println!("\tApparent Temperature: {}", weather.apparent_temperature);
    println!("\tHumidity: {}%", weather.relative_humidity_2m);
    println!("\tPrecipitation: {}mm", weather.precipitation);
    println!("\tWind Speed: {}m/s", weather.wind_speed_10m);
    println!("\tLast fetched: {}", weather.time);
}
