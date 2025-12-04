use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_1() -> io::Result<()> {
    let path = Path::new("src/day/input/input_1.txt");
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    let mut counter: i32 = 50;
    let mut point_to_zero = 0;

    for line in lines {
        let line: String = line?;
        let mut chars = line.chars();
        
        if let Some(direction) = chars.next() {
            if let Ok(number) = line[1..].trim().parse::<i32>() {
                match direction {
                    'L' => {
                        for _ in 0..number {
                            counter -= 1;
                            counter = counter.rem_euclid(100);
                            if counter == 0 {
                                point_to_zero += 1;
                            }
                        }
                    },
                    'R' => {
                        for _ in 0..number {
                            counter += 1;
                            counter = counter.rem_euclid(100);
                            if counter == 0 {
                                point_to_zero += 1;
                            }
                        }
                    },
                    _ => continue,
                }
            }
        }
    }

    // First star result:  1076
    // Second star result: 6379
    println!("Number of times at zero: {}", point_to_zero);

    Ok(())
}