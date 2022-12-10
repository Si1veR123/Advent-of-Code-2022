use std::fs::File;
use std::io::Read;

fn find_non_repeated(length: usize, buffer: &String) -> usize {
    for n in 0..(buffer.len()-(length-1)) {
        let slice: Vec<char> = buffer[n..n+length].chars().collect();
        if !(1..slice.len()).any(|i| (&slice[i..]).contains(&slice[i-1])) {
            return n+length
        }
    }
    0
}

fn main() {
    let mut file = File::open("../data/6.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");

    println!("Part 1");
    println!("{}", find_non_repeated(4, &string_buf));

    println!("Part 2");
    println!("{}", find_non_repeated(14, &string_buf));
}
