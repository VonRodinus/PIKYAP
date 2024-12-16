//models.rs

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub wind: Wind,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub humidity: u64,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
}