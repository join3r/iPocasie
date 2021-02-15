use anyhow::Result;

const API_KEY: &str = include_str!("../api-key.secret");

pub fn get_weather(mesto: String) -> Result<String> {
    Ok(reqwest::blocking::get(&format!(
        "http://api.openweathermap.org/data/2.5/forecast?q={}&units=metric&lang=en&appid={}",
        mesto, API_KEY
    ))?
    .text()?)
}
