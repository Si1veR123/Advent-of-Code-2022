// https://adventofcode.com/2022/day/4

use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Range {
    start: usize,
    stop: usize
}

impl Range {
    fn overlap_complete(&self, other: &Range) -> bool {
        (self.start <= other.start && self.stop >= other.stop) | (other.start <= self.start && other.stop >= self.stop)
    }

    fn overlap_any(&self, other: &Range) -> bool {
        (self.start <= other.start && other.start <= self.stop && other.stop >= self.stop) | (other.start <= self.start && self.start <= other.stop && self.stop >= other.stop) | self.overlap_complete(other)
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        // format "start-stop"
        let (start, stop) = s.split_once('-').unwrap();
        Self {
            start: start.parse().unwrap(),
            stop: stop.parse().unwrap()
        }
    }
}

fn main() {
    let mut file = File::open("../data/4.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");
    let lines: Vec<&str> = string_buf.split("\r\n").collect();

    let mut sum_part1: u32 = 0;
    let mut sum_part2: u32 = 0;

    for line in lines {
        let (range1, range2) = line.split_once(',').unwrap();
        let (range1_parsed, range2_parsed): (Range, Range) = (range1.into(), range2.into());

        sum_part1 += range1_parsed.overlap_complete(&range2_parsed) as u32;
        sum_part2 += range1_parsed.overlap_any(&range2_parsed) as u32;
    }

    println!("Part 1\n{}", sum_part1);
    println!("Part 2\n{}", sum_part2);
}
