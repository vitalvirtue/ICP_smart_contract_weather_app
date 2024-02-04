extern crate serde;
use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpMethod};
use serde_json::Value;
use std::cell::RefCell;
use ic_cdk_macros;
use candid::{CandidType, Deserialize}; // Enables the availability of external libraries

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
async fn update_weather_data() -> Result<(),  String> {
    let api_endpoint = "http://api.openweathermap.org/data/2.5/weather?q=";
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

    let response = match http_request(request, 1_603_079_600).await {
        Ok((response,)) => { 
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err((code, message)) => {
            return Err(format!(
                "The http_request resulted in an error. Code: {:?}, Message: {}",
                code, message
            ).into());
        }
    };

    let weather_data: Value = match serde_json::from_str(&response) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to parse weather data: {}", e)),
    };
    
    let data = WeatherData {
        temperature: (weather_data["main"]["temp"].as_f64().unwrap() * 100.0),
        pressure: weather_data["main"]["pressure"].as_f64().unwrap(),
        humidity: weather_data["main"]["humidity"].as_f64().unwrap(),
        wind_speed: (weather_data["wind"]["speed"].as_f64().unwrap() * 100.0),
    };

    STATE.with(|state| {
        *state.borrow_mut() = State {
            weather_data: Some(data),
        };
    });

    Ok(())
}

#[ic_cdk_macros::query]
fn get_weather_data() -> Option<WeatherData> {
    STATE.with(|state| state.borrow().weather_data.clone())
}

#[allow(dead_code)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    update_weather_data().await?;

    if let Some(data) = get_weather_data() {
        println!("Temperature: {}", data.temperature as f64 / 100.0);
        println!("Pressure: {}", data.pressure);
        println!("Humidity: {}", data.humidity);
        println!("Wind Speed: {}", data.wind_speed as f64 / 100.0);
    }

    Ok(())
}
