use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Weather
{
   main: Main,
   weather: Vec<WeatherDesc>,
}

#[derive(Debug, Deserialize)]
struct Main
{
   temp: f32,
}

#[derive(Debug, Deserialize)]
struct WeatherDesc
{
   description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
   let api_key = "ВАШ_API_КЛЮЧ"; // Замените на свой ключ
   let city = "Moscow";
   let url = format!(
      "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
      city, api_key
   );

   let weather: Weather = reqwest::get(&url).await?.json().await?;
   println!(
      "Погода в {}: {}°C, {}",
      city, weather.main.temp, weather.weather[0].description
   );
   Ok(())
}
