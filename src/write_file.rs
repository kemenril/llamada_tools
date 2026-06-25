use llamada_tools_common::{FileSub,from_json_str};
use std::fs::OpenOptions;
use std::io::{self, Read,Write};
use serde_json::json;
use anyhow::{anyhow,Result};

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let req = match from_json_str::<FileSub>(&buffer) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", json!({"error": format!("Input error: {}", e)}));
            return Err(e.into());
        }
    };


    let Ok(mut outfile) = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&req.path) else {
            println!("{}", json!({"error": format!("Can not create file: {}.  Does it already exist?  You could create a new file under a different name, or delete this one before writing it.", &req.path)}));
            return Err(anyhow!("Can not create file.  Does it already exist?"));
        };

    match outfile.write_all(req.content.as_bytes()) {
        Ok(_) => { // Success is always() here, but we count the bytes up.
            println!("{}", json!({"success": format!("Wrote {} bytes. to {}.", req.content.len(),&req.path)}));
        }
        Err(e) => {
            println!("{}", json!({"error": format!("Created, but can not write data into file: {}.  There may be a disk space problem.  You can try again, but consider aborting until you know why this happened.", &req.path)}));
            return Err(e.into());
        }
    };

    Ok(())
}


