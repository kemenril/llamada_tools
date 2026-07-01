use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
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


//For the http_request tool
#[derive(Deserialize,Debug)]
pub struct HttpRequest {
    // ... as it says.
    pub url: String,
    // One of GET, POST, PUT, DELETE... Are there any others?
    pub method: String,
    // A set of extra headers to send with the request.
    #[serde(default)]
    pub headers: HashMap<String,String>,
    // The body of the request.
    #[serde(default)]
    pub body: String,
    // One of "immediate" or "file"; maybe we could optionally include "quiet"
    // or some such thing to allow only the status information to be taken and
    // the actual output ignored and dumped.
    #[serde(default)]
    pub output_mode: String,
    //If output goes to a file, where is that file?
    #[serde(default)]
    pub output_file: String,
}

#[derive(Serialize,Debug)]
pub struct HttpResponse {
    //Only the numeric code; 404, 200, or the like...
    pub http_status_code: u16,
    //The textual status that goes along with the numeric code above.
    pub http_status_message: String,
    //If we have a message for the LLM or the user, we can put it in here.
    pub local_tool_status_message: String,
    //The headers received with the response 
    pub headers: HashMap<String,String>,
    //Body of the response, which should only be populated in immediate mode,
    //or if the file for some reason can't be written and we fall back to 
    //immediate mode.
    pub body: String,
    // If we've been asked to write a file, and we have succeeded, where is it?
    pub file: String,
}


//Maybe the only part of the inport process it makes sense to put here yet.
pub fn from_json_str<T>(json_str: &str) -> Result<T,serde_json::Error> where
    T: DeserializeOwned,
{
        let req: T = serde_json::from_str(json_str)?;
        Ok(req)
}

