use anyhow::Result;
mod forecast;
mod one_call_api;
use one_call_api::get_weather;

use forecast::Forecast;

fn main() -> Result<()> {
    let text = get_weather("Roznava".to_string());
    Ok(())
}
