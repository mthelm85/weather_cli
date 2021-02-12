use std::env;
use reqwest::blocking;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    pub sunrise: i64,
    pub sunset: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: u64,
    pub gust: Option<f64>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub main: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Success {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub wind: Wind,
    pub sys: Sys,
    pub timezone: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub cod: String,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Response {
    Success(Success),
    Error(Error)
}

pub fn get_weather(cty: &Vec<String>, st: &Vec<String>) -> Result<Response, Box<dyn std::error::Error>> {
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
    let res: Response = blocking::get(url)?.json()?;
    Ok(res)
}