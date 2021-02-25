use anyhow::Result;
use ipgeolocate::{Locator, Service};

const API_KEY: &str = include_str!("../api-key.secret");

pub enum RequestType {
    Weather,
    Forecast,
}

pub fn get_location() -> Result<String> {
    let loc = Locator::get("1.1.1.1", Service::IpApi)?;
    Ok(loc.city)
}

pub fn get_weather(mesto: &str, req_type: RequestType) -> Result<String> {
    Ok(reqwest::blocking::get(&format!(
        "http://api.openweathermap.org/data/2.5/{}?q={}&units=metric&lang=en&appid={}",
        req_type, mesto, API_KEY
    ))?
    .text()?)
} 

impl std::fmt::Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestType::Forecast => write!(f, "forecast"),
            RequestType::Weather => write!(f, "weather"),
        }
    }
}