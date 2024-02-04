use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GeoLocation {
    pub country: String,
    #[serde(rename(deserialize = "regionName"))]
    pub region_name: String,
    pub city: String,
    pub lat: f64,
    pub lon: f64,
}

pub fn get_ip_geo_location() -> GeoLocation {
    let response = reqwest::blocking::get("http://ip-api.com/json").unwrap();
    let location = response.json().unwrap();
    location
}