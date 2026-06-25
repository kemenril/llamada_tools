use llamada_tools_common::{FileReq,from_json_str};
use std::fs;
use std::path::{PathBuf,Path};
use std::io::{self, Read};
use serde_json::json;
use anyhow::{anyhow,Result};

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let req = match from_json_str::<FileReq>(&buffer) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", json!({"error": format!("Input error: {}", e)}));
            return Err(e.into());
        }
    };

    fs::create_dir_all("/Trash")?; 
    let fname = Path::new(&req.path).file_name().unwrap().to_string_lossy();
    let mut trashdest = PathBuf::from(format!("{}/{}","/Trash",fname));

    let mut i = 1;
    loop {

        if !fs::exists(&trashdest).unwrap() { break; }

        //Really dumb, but we're storing a PathBuf and adding String (and an
        // integer by way of string conversion) onto the end.
        trashdest = PathBuf::from(format!("{}/{}.{}","/Trash",&fname,i));
        // I suppose one has to have limits.
        if i > 255 {
            println!("{}", json!({"error": format!("Error: Please empty the trash first.")}));
            return Err(anyhow!("Please empty the trash first."));
        }
        i+=1;
    }


    match fs::rename(&req.path,trashdest) {
        Ok(())  => println!("{}", json!({"success": format!("{} moved to trash.",&req.path)})),
        Err(e)  => {
            println!("{}", json!({"error": format!("Error: {}", e)}));
            return Err(e.into());
        }
    }

    Ok(())
}


