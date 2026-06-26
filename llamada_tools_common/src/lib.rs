use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use anyhow::Result;

//For reading file data
#[derive(Deserialize, Debug)]
pub struct FileReq {
    pub path: String,
}

//For writing
#[derive(Deserialize, Debug)]
pub struct FileSub {
    pub path: String,
    pub content: String,
}

//Things we may want to pass *in* to get time data
#[derive(Deserialize,Debug)]
pub struct DateTimeReq {
    #[serde(default)]
    pub zone: String,
}


#[derive(Serialize, Debug)]
pub struct DateTime {
    /// Full ISO 8601 string including local offset (e.g., "2026-06-23T14:34:00-05:00")
    pub iso_8601: String,
    pub human_readable: String,
    pub zone: String,

    // Explicit components to prevent the LLM from parsing strings
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour_24h: u32,
    pub minute: u32,
    pub second: u32,
    pub weekday: String,
    pub ordinal_day_of_year: u32,
}


//Maybe the only part of the inport process it makes sense to put here yet.
pub fn from_json_str<T>(json_str: &str) -> Result<T,serde_json::Error> where
    T: DeserializeOwned,
{
        let req: T = serde_json::from_str(json_str)?;
        Ok(req)
}

