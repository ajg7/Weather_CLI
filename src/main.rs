use structopt::StructOpt;
use exitfailure::{ExitFailure};
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;

#[derive(StructOpt)]
struct Cli {
    city: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32
}
#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64
}
#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32, 
    country: String,
    sunrise: i32,
    sunset: i32
}

impl Forecast {
    async fn get(city: String) -> Result<Self,ExitFailure> {
        let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid=b1d1b7f31f0345a8b377d968d5e018c3", city);
        let url = Url::parse(&*url)?;

        let response = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;
        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let arguments = Cli::from_args();
    let place =  &arguments.city;
    let response = Forecast::get(place.to_string()).await?;
    let temp = kelvin_to_farenheit(response.main.temp);
    let feels_like = kelvin_to_farenheit(response.main.feels_like);
    let wind_speed = meters_per_sec_to_mph(response.wind.speed);
    println!("City: {}, Temp: {:.2} F, Feels Like: {:.2} F, Wind Speed: {:.2} mph", response.name, temp, feels_like, wind_speed);
    Ok(())
}

fn kelvin_to_farenheit(kelvin: f64) -> f64 {
    1.8 * (kelvin - 273.0) + 32.0
}

fn meters_per_sec_to_mph(speed: f64) -> f64 {
    speed * 2.237
}