use std::fmt::Display;

use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum WeatherCode {
    Clear = 0,
    MainlyClear = 1,
    PartlyCloudy = 2,
    Overcast = 3,
    Fog = 45,
    DepositingRimeFog = 48,
    DrizzleLight = 51,
    DrizzleModerate = 53,
    DrizzleDense = 55,
    FreezingDrizzleLight = 56,
    FreezingDrizzleDense = 57,
    RainSlight = 61,
    RainModerate = 63,
    RainHeavy = 65,
    FreezingRainLight = 66,
    FreezingRainHeavy = 67,
    SnowFallSlight = 71,
    SnowFallModerate = 73,
    SnowFallHeavy = 75,
    SnowGrains = 77,
    RainShowersSlight = 80,
    RainShowersModerate = 81,
    RainShowersViolent = 82,
    SnowShowersSlight = 85,
    SnowShowersHeavy = 86,
    ThunderstormSlight = 95,
    ThunderstormModerate = 96,
    ThunderstormHeavy = 99,
}

impl Display for WeatherCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeatherCode::Clear => write!(f, "Clear"),
            WeatherCode::MainlyClear => write!(f, "Mainly Clear"),
            WeatherCode::PartlyCloudy => write!(f, "Partly Cloudy"),
            WeatherCode::Overcast => write!(f, "Overcast"),
            WeatherCode::Fog => write!(f, "Fog"),
            WeatherCode::DepositingRimeFog => write!(f, "Depositing Rime Fog"),
            WeatherCode::DrizzleLight => write!(f, "Drizzle Light"),
            WeatherCode::DrizzleModerate => write!(f, "Drizzle Moderate"),
            WeatherCode::DrizzleDense => write!(f, "Drizzle Dense"),
            WeatherCode::FreezingDrizzleLight => write!(f, "Freezing Drizzle Light"),
            WeatherCode::FreezingDrizzleDense => write!(f, "Freezing Drizzle Dense"),
            WeatherCode::RainSlight => write!(f, "Rain Slight"),
            WeatherCode::RainModerate => write!(f, "Rain Moderate"),
            WeatherCode::RainHeavy => write!(f, "Rain Heavy"),
            WeatherCode::FreezingRainLight => write!(f, "Freezing Rain Light"),
            WeatherCode::FreezingRainHeavy => write!(f, "Freezing Rain Heavy"),
            WeatherCode::SnowFallSlight => write!(f, "Snow Fall Slight"),
            WeatherCode::SnowFallModerate => write!(f, "Snow Fall Moderate"),
            WeatherCode::SnowFallHeavy => write!(f, "Snow Fall Heavy"),
            WeatherCode::SnowGrains => write!(f, "Snow Grains"),
            WeatherCode::RainShowersSlight => write!(f, "Rain Showers Slight"),
            WeatherCode::RainShowersModerate => write!(f, "Rain Showers Moderate"),
            WeatherCode::RainShowersViolent => write!(f, "Rain Showers Violent"),
            WeatherCode::SnowShowersSlight => write!(f, "Snow Showers Slight"),
            WeatherCode::SnowShowersHeavy => write!(f, "Snow Showers Heavy"),
            WeatherCode::ThunderstormSlight => write!(f, "Thunderstorm Slight"),
            WeatherCode::ThunderstormModerate => write!(f, "Thunderstorm Moderate"),
            WeatherCode::ThunderstormHeavy => write!(f, "Thunderstorm Heavy"),
        }
    }
}