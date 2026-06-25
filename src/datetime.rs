use chrono::{Local, Datelike, Timelike};
use llamada_tools_common::{DateTime};
use serde_json::json;
use anyhow::Result;


//Throw together a LocalTime struct
fn get_lts() -> DateTime {
    let now = Local::now();
    
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
    match serde_json::to_string(&get_lts()) {
        Ok(json_str) => println!("{}", json_str),
        Err(e) => {
            println!("{}",json!({"error": format!("Problem writing JSON output: {}",e)}));
            return Err(e.into());
        }
    };
    Ok(())
}


