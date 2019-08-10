use std::io::{self, BufReader, BufRead };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(io::stdin());
    let mut buf = String::new();

    while reader.read_line(&mut buf)? > 0 {
        println!("{}", buf);
        buf.clear();
    }

    Ok(())
}
