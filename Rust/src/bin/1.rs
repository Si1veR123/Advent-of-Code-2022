// https://adventofcode.com/2022/day/1

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("../data/1.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");
    let lines: Vec<&str> = string_buf.split("\r\n").collect();

    println!("Part 1");
    let mut sums: Vec<u32> = vec![];

    lines.iter()
        .cloned()
        .fold(0u32, 
            |acc, e| 
            if e == "" {
                // new elf
                sums.push(acc);
                0
            } else {
                // accumulate current elf
                acc + e.parse::<u32>().expect("Invalid data found")
            });
    
    sums.sort_unstable();
    println!("{}", sums.last().unwrap());

    println!("Part 2");
    let top3 = &sums[sums.len()-3..];
    println!("{}", top3.iter().sum::<u32>());
}
