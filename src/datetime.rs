use std::io::{self,Read};
use std::str::FromStr;
use chrono::{Utc, Datelike, Timelike};
use chrono_tz::{Tz,UTC};
use llamada_tools_common::{DateTime,DateTimeReq,from_json_str};
use serde_json::json;
use anyhow::Result;


//Throw together a LocalTime struct
fn get_lts(zstr: &str) -> DateTime {
    // We may use this and apply a UTC zone to an already UTC time. 
    // I guess that's ok.
    let tz: Tz = Tz::from_str(zstr).unwrap_or(UTC);
    let now = Utc::now().with_timezone(&tz);
    
    DateTime {
        iso_8601: now.to_rfc3339(),
        human_readable: now.format("%A, %B %d, %Y, %I:%M %p").to_string(),
        zone: now.format("%Z").to_string(), // Falls back to offset if name is unavailable
        year: now.year(),
        month: now.month(),
        day: now.day(),
        hour_24h: now.hour(),
        minute: now.minute(),
        second: now.second(),
        weekday: now.weekday().to_string(),
        ordinal_day_of_year: now.ordinal(),
    }
}


fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let req = match from_json_str::<DateTimeReq>(&buffer) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", json!({"error": format!("Input error: {}", e)}));
            return Err(e.into());
        }
    };


    match serde_json::to_string(&get_lts(&req.zone)) {
        Ok(json_str) => println!("{}", json_str),
        Err(e) => {
            println!("{}",json!({"error": format!("Problem writing JSON output: {}",e)}));
            return Err(e.into());
        }
    };
    Ok(())
}


