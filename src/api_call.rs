use std::env;
use reqwest::blocking;
use serde_json::Value;

pub fn get_weather(cty: &Vec<String>, st: &Vec<String>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = "https://api.openweathermap.org/data/2.5/weather";
    let api_key = env::var("OPEN_WEATHER_API_KEY")?;
    let city:String = cty.join(" ");
    let state: String = st.join("");
    let params = [
        ("q", city + "," + &state),
        ("appid", api_key),
        ("units", String::from("imperial"))
    ];
    let url = reqwest::Url::parse_with_params(url, &params)?;
    let res = blocking::get(url)?.text()?;
    let v: Value = serde_json::from_str(&res)?;
    Ok(v)
}