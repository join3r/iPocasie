#![allow(non_snake_case)]
use std::io::{self, Write};

use anyhow::Result;
mod forecast;
mod one_call_api;
use one_call_api::{get_weather, get_location};
use forecast::Forecast;
use one_call_api::RequestType;

fn main() -> Result<()> {

    let geo_mesto = get_location();
    print!("Zadaj mesto ({}): ", geo_mesto?);
    io::stdout().flush()?;
    let mut mesto = String::new();
    io::stdin().read_line(&mut mesto)?;
    let text = get_weather(&mesto, RequestType::Forecast)?;
    let text_deserialized: Forecast = serde_json::from_str(&text)?;
    println!("V tvojom meste {} je pocasie: ", mesto);
    Ok(())
}

