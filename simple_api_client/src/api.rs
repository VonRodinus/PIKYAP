//api.rs

use reqwest::blocking::get;
use serde_json::from_str;
use anyhow::{Result, Context};

use crate::models::WeatherResponse;

const API_URL: &str = "https://api.openweathermap.org/data/2.5/weather";
const API_KEY: &str = "d44c06d4464ffc840a547bcb50a434a2";

pub fn fetch_weather_data(city: &str) -> Result<WeatherResponse> {
    let url = format!("{}?q={}&appid={}&units=metric", API_URL, city, API_KEY);
    let response = get(&url).context("Failed to send request to the API")?;
    let body = response.text().context("Failed to read response body")?;
    
    let weather_data: WeatherResponse = from_str(&body).context("Failed to parse JSON response")?;
    Ok(weather_data)
}