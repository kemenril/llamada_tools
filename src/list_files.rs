use llamada_tools_common::{FileReq,from_json_str};
use std::fs;
use std::path::Path;
use std::io::{self, Read};
use serde_json::json;
use anyhow::{anyhow,Result};

fn main() -> Result<()> {

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let req = match from_json_str::<FileReq>(&buffer) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", json!({"error": format!("Input error: {e}")}));
            return Err(e.into());
        }
    };

    let dir = Path::new(&req.path);

    if !dir.is_dir() {
        println!("{}", json!({"error": format!("Path '{}' is not a directory or does not exist.  Do you need to create it first?", req.path)}));
        return Err(anyhow!("Target path is not a directory"));
    }

    // Maybe there's a better way, but this way we catch errors for read_dir
    // in particular, before we continue on.
    let entries = match fs::read_dir(dir) {
        Ok(iter) => iter,
        Err(e) => {
            println!("{}", json!({"error": format!("Could not read directory contents: {e}")}));
            return Err(e.into());
        }
    };

    let mut file_list = Vec::new();

    for entry in entries {
        match entry {
            Ok(file_entry) => {
                let path = file_entry.path();
                if let Some(path_str) = path.to_str() {
                    // Visual anchor: Add a trailing slash if it's a sub-directory
                    if path.is_dir() {
                        file_list.push(format!("{}/", path_str));
                    } else {
                        file_list.push(path_str.to_string());
                    }
                }
            }
            Err(e) => {
                println!("{}", json!({"error": format!("Error reading individual directory entry: {}",e)}));
                return Err(e.into());
            }
        }
    }

    // ... and the output is actually pretty reasonably simple for now.
    match serde_json::to_string(&file_list) {
        Ok(json_str) => println!("{}", json_str),
        Err(e) => {
            println!("{}", json!({"error": format!("Serialization error: {e}")}));
            return Err(e.into());
        }
    }

    Ok(())
}


