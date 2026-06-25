use llamada_tools_common::{FileReq,from_json_str};
use std::fs;
use std::io::{self, Read};
use serde_json::json;
use anyhow::Result;

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let req = match from_json_str::<FileReq>(&buffer) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", json!({"error": format!("Input error: {}", e)}));
            return Ok(()); // Or return Err(e) to stop the program
        }
    };

    match fs::create_dir_all(&req.path) {
        Ok(())  => println!("{}", json!({"success": format!("Created directory: {}",&req.path)})),
        Err(e)  => println!("{}", json!({"error": format!("File error: {}", e)})),
    }

    Ok(())
}


