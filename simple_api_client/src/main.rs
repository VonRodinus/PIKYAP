//main.rs

mod api;
mod models;
mod cli;
mod cache;

use anyhow::Result;
use cli::get_user_input;
use api::fetch_weather_data;
use cache::Cache;


const CACHE_FILE: &str = "weather_cache.json";

fn main() -> Result<()> {
    println!("Welcome to the Weather CLI App!");

    // Загрузка кэша из файла
    let mut cache = Cache::load_from_file(CACHE_FILE).unwrap_or_else(|_| Cache::new());

    loop {
        let city = get_user_input("Enter the city name: ")?;

        // Проверяем, есть ли данные в кэше
        if let Some(cached_data) = cache.get(&city) {
            println!("\nCached Weather for {}:\n{}", city, cached_data);
        } else {
            // Данные не найдены в кэше, делаем запрос к API
            match fetch_weather_data(&city) {
                Ok(weather) => {
                    let weather_info = format!(
                        "Temperature: {}°C\nHumidity: {}%\nWind Speed: {} m/s\nDescription: {}",
                        weather.main.temp, weather.main.humidity, weather.wind.speed, weather.weather[0].description
                    );

                    println!("\nWeather for {}:\n{}", city, weather_info);

                    // Сохраняем данные в кэш
                    cache.insert(city.clone(), weather_info);
                    cache.save_to_file(CACHE_FILE).unwrap_or_else(|err| {
                        eprintln!("Failed to save cache: {}", err);
                    });
                }
                Err(err) => {
                    eprintln!("Error fetching weather data: {}", err);
                }
            }
        }

        // Спрашиваем пользователя, хочет ли он продолжить
        loop {
            let continue_choice = get_user_input("Do you want to check another city? (yes/no): ")?;
            match continue_choice.to_lowercase().as_str() {
                "yes" => break,
                "no" => {
                    println!("Goodbye!");
                    return Ok(());
                }
                _ => println!("Invalid input. Please type 'yes' or 'no'."),
            }
        }
    }
}
