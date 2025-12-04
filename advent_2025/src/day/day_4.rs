use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_4() -> io::Result<()> {
    let path = Path::new("src/day/input/input_4.txt");
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    
    for line in lines {

    }
    
    // First star:  
    // Second star: 

    Ok(())
}
