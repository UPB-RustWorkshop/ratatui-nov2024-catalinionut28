use chrono::{DateTime, Utc, FixedOffset, TimeZone};
use reqwest;
use serde_json::{self, error};
use std::{error::Error, ptr::null};




macro_rules!  define_constant {
    ($name: ident, $val: expr) => {
        const $name: f64 = $val; 
        
    };
}


define_constant!(ZERO_CELSIUS, 273.15);

#[derive(Debug)]
 pub struct MyError(String);

#[derive(Debug)]
pub struct CityInfo {
    pub name: String,
    pub description: String,
    pub humidity: i64,
    pub temperature: f64,
    pub feels_like: f64,
    pub date_time: DateTime<FixedOffset>,
}

impl Default for CityInfo {
    fn default() -> Self {
        let now_utc = Utc::now();
        let fixed_offset = FixedOffset::east(0);
        let date_time = now_utc.with_timezone(&fixed_offset);

        Self {
            name: "Unknown".to_string(),
            description: "No description available.".to_string(),
            humidity: 0,
            temperature: 0.0,
            feels_like: 0.0,
            date_time,
        }
    }
}



pub async fn get_data(city: String) -> Result<CityInfo, MyError> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid=d4989698b462d50088ca919c1d9e154d",
        city
    );
    match reqwest::get(&url).await {    
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await.map_err(|err| MyError(err.to_string()))?;
                let parsed: serde_json::Value = serde_json::from_str(&body)
                    .map_err(|err| MyError(err.to_string()))?;

                let description = parsed["weather"]
                    .get(0)
                    .and_then(|w| w["description"].as_str())
                    .unwrap_or("Unknown")
                    .to_string();

                let feels_like = parsed["main"]["feels_like"].as_f64().unwrap_or(0.0);
                let temperature = parsed["main"]["temp"].as_f64().unwrap_or(0.0);
                let humidity = parsed["main"]["humidity"].as_i64().unwrap_or(0);

                let utc_time: DateTime<Utc> = Utc::now();
                let _dt = parsed["dt"].as_i64().unwrap_or(0);
                let tz = parsed["timezone"].as_i64().unwrap_or(0) as i32;

                let offset = if tz < 0 {
                    FixedOffset::west(-tz)
                } else {
                    FixedOffset::east(tz)
                };

                let city_time = offset
                    .from_utc_datetime(&utc_time.naive_utc());

                let city_info = CityInfo {
                    name: city.clone(),
                    description: description,
                    humidity: humidity,
                    temperature: temperature - ZERO_CELSIUS,
                    feels_like: feels_like - ZERO_CELSIUS,
                    date_time: city_time,
                };
                Ok(city_info)
            } else {
            return Err(MyError("The response has failed, try again!".to_string()));
            }
        }
        Err(error) => Err(MyError(error.to_string())),
    }
}