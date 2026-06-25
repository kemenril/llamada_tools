use std::io::{self,Read};

fn echo(msg: &str) -> String {
    format!(r#"{{"echo": {}}}"#, msg)
}

pub fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    if buf.trim().is_empty() {
        println!("{}",echo("{}"));
    } else {
        println!("{}",echo(&buf));
    }
    Ok(())
}

