// https://adventofcode.com/2022/day/3

use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn get_priority(c: char) -> u8 {
    if c.is_uppercase() {
        return (c as u8) - 38
    } else {
        return (c as u8) - 96
    }
}

fn main() {
    let start = Instant::now();

    let mut file = File::open("../data/3.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");
    let lines: Vec<&str> = string_buf.split("\r\n").collect();

    println!("Part 1");
    let sum_priorities: u32 = lines.iter().map(
        |&x| {
            let chars: Vec<char> = x.chars().collect();
            let half_point = (chars.len()/2) as usize;
            let first_chars_slice = &chars[..half_point];
            let second_chars_slice = &chars[half_point..];
            let first_compartment: HashSet<&char, RandomState> = HashSet::from_iter(first_chars_slice.iter());
            let second_compartment: HashSet<&char, RandomState> = HashSet::from_iter(second_chars_slice.iter());

            let &&item = first_compartment.intersection(&second_compartment).next().unwrap();
            get_priority(item) as u32
        }
    ).sum();
    println!("{}", sum_priorities);

    println!("Part 2");
    let mut accumulated_groups = vec![];

    let mut next_vec: Vec<&str> = Vec::with_capacity(3);
    for &line in &lines {
        next_vec.push(line);

        if next_vec.len() == 3 {
            accumulated_groups.push(next_vec.clone());
            next_vec.clear();
        }
    }

    let badges_sum: u32 = accumulated_groups.iter().map(
        |x| {
            // assumed length of x is always 3, or will panic
            let sets: [HashSet<char, RandomState>; 3] = [
                HashSet::from_iter(x.get(0).unwrap().chars()),
                HashSet::from_iter(x.get(1).unwrap().chars()),
                HashSet::from_iter(x.get(2).unwrap().chars()),
            ];

            let first_intersect = HashSet::from_iter(sets[1].intersection(&sets[2]).cloned());
            let all_intersect = sets[0].intersection(&first_intersect);

            let badge = all_intersect.last().unwrap().clone();
            get_priority(badge) as u32
        }
    ).sum();

    println!("{}", badges_sum);

    println!("Took: {:?}", Instant::now() - start);
}
