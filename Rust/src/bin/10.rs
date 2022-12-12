// https://adventofcode.com/2022/day/10

use std::fs::File;
use std::io::Read;

fn what_to_print(x: i16, cycle: i16) -> String {
    let newline: bool = cycle % 40 == 0;

    let mut sprite_binary_mask = (1 << (39-x)) as u32;
    sprite_binary_mask |= (1 << (40-x)) as u32;
    sprite_binary_mask |= (1 << (38-x).abs()) as u32;

    let x_pos = (cycle-1) % 40;

    let cycle_binary_mask = (1 << (39-x_pos)) as u32;

    if (sprite_binary_mask & cycle_binary_mask) > 0 {
        return match newline {
            true => "█\n".to_string(),
            false => "█".to_string()
        }
    }
    match newline {
        true => " \n".to_string(),
        false => " ".to_string()
    }
}

fn main() {
    let mut file = File::open("../data/10.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");
    let lines: Vec<&str> = string_buf.split("\r\n").collect();

    let mut signals: Vec<i16> = vec![];
    // using signed ints make math easier
    let mut current_cycle: i16 = 1;
    let mut x: i16 = 1;

    for &instruction in &lines {
        if instruction == "noop" {
            signals.push(x * current_cycle);
            current_cycle += 1;
        }
        else if instruction.starts_with("addx") {
            signals.push(x * current_cycle);
            current_cycle += 1;
            signals.push(x * current_cycle);
            current_cycle += 1;
            let amount: i16 = (instruction[5..]).parse().unwrap();
            x += amount;
        }
    }

    println!("Part 1");
    println!("{}", signals[19] + signals[59] + signals[99] + signals[139] + signals[179] + signals[219]);

    println!("Part 2");

    let mut current_cycle = 1;
    let mut x = 1;
    for &instruction in &lines {
        if instruction == "noop" {
            print!("{}", what_to_print(x, current_cycle));
            current_cycle += 1;
        }

        else if instruction.starts_with("addx") {
            print!("{}", what_to_print(x, current_cycle));
            current_cycle += 1;
            print!("{}", what_to_print(x, current_cycle));
            current_cycle += 1;

            let amount: i16 = instruction[5..].parse().unwrap();
            x += amount;
        }
    }
}
