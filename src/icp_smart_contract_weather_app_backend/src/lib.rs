extern crate serde;
use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpMethod};
use std::cell::RefCell;
use ic_cdk_macros;
use candid::{CandidType, Deserialize}; // Enables the availability of external libraries
use serde_json::Value;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct WeatherData {
    temperature: f64,
    pressure: f64,
    humidity: f64,
    wind_speed: f64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct State {
    weather_data: Option<WeatherData>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        weather_data: None,
    });
}

#[ic_cdk_macros::update]
async fn get_weather_data() -> Result<WeatherData, String> {
    let api_endpoint = "https://api.openweathermap.org/data/2.5/weather?q=";
    let city = "Ankara";
    let country_code = "TR";
    let open_weather_map_api_key = "edb255292600fae328e811b1554ff324";

    let full_url = format!("{}{},{}&APPID={}", api_endpoint, city, country_code, open_weather_map_api_key);

    let request = CanisterHttpRequestArgument {
        url: full_url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: vec![],
    };

    match http_request(request, 1_603_099_200).await {
        Ok((response,)) => {
            let v: Value = serde_json::from_slice(&response.body).map_err(|_| "Failed to parse weather data")?;
            let weather = WeatherData {
                temperature: v["main"]["temp"].as_f64().unwrap_or_default(),
                pressure: v["main"]["pressure"].as_f64().unwrap_or_default(),
                humidity: v["main"]["humidity"].as_f64().unwrap_or_default(),
                wind_speed: v["wind"]["speed"].as_f64().unwrap_or_default(),
            };
            Ok(weather)
        }
        Err((code, msg)) => Err(format!("Failed to make HTTP request: {} - {}", code as u16, msg)),
    }
}
