use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_2() -> io::Result<()> {
    let path = Path::new("src/day/input/input_2.txt");
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    let line: String = lines.into_iter().next().unwrap()?;
    let range: Vec<String> = line.split(',').map(|s| s.to_string()).collect();

    let mut count: i64 = 0;

    for r in range {
        let bounds: Vec<i64> = r.split('-').map(|s| s.parse::<i64>().unwrap()).collect();

        for i in bounds[0]..=bounds[1] {
            let s = i.to_string();
            
            // --- Fisrt star ---
            // let lenght_s = s.len();
            // if lenght_s % 2 != 0 {
            //     continue;
            // }

            // if s[0..lenght_s / 2] == s[lenght_s / 2..lenght_s] {
            //     count += i;
            // }

            // --- Second star ---
            if is_repeated_pattern(&s) {
                count += i;
            }
        }
    }

    println!("{}", count);

    // First star:  23039913998
    // Second star: 35950619148

    Ok(())
}

fn is_repeated_pattern(s: &str) -> bool {
    let len = s.len();

    for pattern_len in 1..=len / 2 {
        if len % pattern_len != 0 {
            continue;
        }

        let pattern = &s[0..pattern_len];
        let repetitions = len / pattern_len;

        if pattern.repeat(repetitions) == s {
            return true;
        }
    }

    false
}
