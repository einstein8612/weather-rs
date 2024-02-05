mod ipapi;
mod openmeteo;
mod tui;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ipapi::get_ip_geo_location;
use openmeteo::get_current_weather;

use tui::TUI;

fn main() {
    let mut tui = TUI::new();

    let location = get_ip_geo_location();
    tui.add_header(String::from("Location"));
    tui.add_line(format!("Country: {}", location.country));
    tui.add_line(format!("City: {}/{}", location.region_name, location.city));
    tui.add_line(format!("Latitude: {}", location.lat));
    tui.add_line(format!("Longtitude: {}", location.lon));
    tui.add_footer();

    let weather = get_current_weather(location.lat, location.lon, &location.timezone);
    tui.add_header(String::from("Weather"));
    tui.add_line(format!("Forecast: {}", weather.weather_code));
    tui.add_line(format!("Temperature: {}°C ", weather.temperature_2m));
    tui.add_line(format!("Apparent Temperature: {}°C", weather.apparent_temperature));
    tui.add_line(format!("Humidity: {}%", weather.relative_humidity_2m));
    tui.add_line(format!("Precipitation: {}mm", weather.precipitation));
    tui.add_line(format!("Wind Speed: {}km/h", weather.wind_speed_10m));

    let last_updated = SystemTime::now()
        .duration_since(UNIX_EPOCH + Duration::from_secs(weather.time))
        .expect("Time went backwards");
    tui.add_line(format!("Last updated: {} minutes ago", last_updated.as_secs()/60));
    tui.add_footer();
    tui.flush();
}
