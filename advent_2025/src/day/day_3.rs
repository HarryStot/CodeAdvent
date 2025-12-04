use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_3() -> io::Result<()> {
    let path = Path::new("src/day/input/input_3.txt");
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    
    let mut total_joltage_1 = 0;
    let mut total_joltage_2: u64 = 0;
    
    for line in lines {
        let line: String = line?;
        let chars: Vec<char> = line.chars().collect();
        
        // --- First star ---
        let mut max_joltage = 0;
        for i in 0..chars.len() {
            for j in (i + 1)..chars.len() {
                if let (Some(d1), Some(d2)) = (chars[i].to_digit(10), chars[j].to_digit(10)) {
                    let joltage = d1 * 10 + d2;
                    max_joltage = max_joltage.max(joltage);
                }
            }
        }
        total_joltage_1 += max_joltage;

        // --- Second star ---
        let digits: Vec<u32> = chars
            .iter()
            .filter_map(|&c| c.to_digit(10))
            .collect();
        
        let k = digits.len() - 12;
        let mut stack: Vec<u32> = Vec::new();
        let mut removals_left = k;
        
        for &digit in &digits {
            while !stack.is_empty() && removals_left > 0 && *stack.last().unwrap() < digit {
                stack.pop();
                removals_left -= 1;
            }
            stack.push(digit);
        }
        
        while removals_left > 0 {
            stack.pop();
            removals_left -= 1;
        }
        
        let joltage_str: String = stack.iter().map(|&d| char::from_digit(d, 10).unwrap()).collect();
        let joltage_12: u64 = joltage_str.parse().unwrap();
        
        total_joltage_2 += joltage_12;
    }
    
    println!("Total output joltage (2 batteries): {}", total_joltage_1);
    println!("Total output joltage (12 batteries): {}", total_joltage_2);

    // First star:  17281
    // Second star: 171388730430281

    Ok(())
}
