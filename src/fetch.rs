use crate::config::Config;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub current: Current,
    pub hourly: Vec<Hourly>,
    pub daily: Vec<Daily>,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    pub dt: i64,
    pub temp: f64,
    pub humidity: u8,
    pub weather: Vec<WeatherDescription>,
}

#[derive(Debug, Deserialize)]
pub struct Hourly {
    pub dt: i64,
    pub temp: f64,
    pub humidity: u8,
    pub weather: Vec<WeatherDescription>,
}

#[derive(Debug, Deserialize)]
pub struct Daily {
    pub dt: i64,
    pub temp: DailyTemp,
    pub humidity: u8,
    pub weather: Vec<WeatherDescription>,
}

#[derive(Debug, Deserialize)]
pub struct DailyTemp {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Deserialize)]
pub struct WeatherDescription {
    // pub main: String,
    pub description: String,
}

pub fn fetch_weather(cfg: &Config) -> anyhow::Result<WeatherResponse> {
    let endpoint = "https://api.openweathermap.org/data/3.0/onecall";

    let client = Client::new();
    let resp = client
        .get(endpoint)
        .query(&[
            ("lat", &cfg.lat.to_string()),
            ("lon", &cfg.lon.to_string()),
            ("appid", &cfg.api_key),
            ("exclude", &"minutely".to_string()),
            ("units", &cfg.units),
            ("lang", &cfg.lang),
        ])
        .timeout(Duration::from_secs(10))
        .send()?
        .error_for_status()?;

    let weather: WeatherResponse = resp.json()?;
    Ok(weather)
}
