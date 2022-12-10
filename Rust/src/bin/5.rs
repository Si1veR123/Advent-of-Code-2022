// https://adventofcode.com/2022/day/5

use std::fs::File;
use std::io::Read;

struct State {
    stacks: Vec<Vec<char>>
}

impl State {
    fn move_crate(&mut self, n: usize, from_stack: usize, to_stack: usize) {
        for _ in 0..n {
            let char_to_move = self.stacks.get_mut(from_stack-1).unwrap().pop().unwrap();
            let target_stack: &mut Vec<char> = self.stacks.get_mut(to_stack-1).unwrap();
            target_stack.push(char_to_move);
        }
    }

    fn move_multiple(&mut self, n: usize, from_stack: usize, to_stack: usize) {
        let mut holding_temp: Vec<char> = Vec::with_capacity(n); // use with capacity to prevent any more allocating memory
        for _ in 0..n {
            holding_temp.push(self.stacks.get_mut(from_stack-1).unwrap().pop().unwrap())
        }

        for held_crate in holding_temp.iter().rev() {
            self.stacks.get_mut(to_stack-1).unwrap().push(held_crate.clone())
        }
    }
}

fn main() {
    let mut file = File::open("../data/5.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");
    let lines: Vec<&str> = string_buf.split("\r\n").skip(10).collect();

    let parsed_args: Vec<(usize, usize, usize)> = lines.iter().map(
        |&x| {
            let mut split = x.split_ascii_whitespace();
            (split.nth(1).unwrap().parse().unwrap(), split.nth(1).unwrap().parse().unwrap(), split.nth(1).unwrap().parse().unwrap())
        }
    ).collect();

    let initial = vec![
        vec!['S', 'C', 'V', 'N'],
        vec!['Z', 'M', 'J', 'H', 'N', 'S'],
        vec!['M', 'C', 'T', 'G', 'J', 'N', 'D'],
        vec!['T', 'D', 'F', 'J', 'W', 'R', 'M'],
        vec!['P', 'F', 'H'],
        vec!['C', 'T', 'Z', 'H', 'J'],
        vec!['D', 'P', 'R', 'Q', 'F', 'S', 'L', 'Z'],
        vec!['C', 'S', 'L', 'H', 'D', 'F', 'P', 'W'],
        vec!['D', 'S', 'M', 'P', 'F', 'N', 'G', 'Z']
    ];

    let mut state = State { stacks: initial.clone() };

    for arg in parsed_args.clone() {
        state.move_crate(arg.0, arg.1, arg.2)
    }

    let mut out_str = String::new();
    for stack in state.stacks {
        out_str += &stack.last().unwrap().to_string();
    }
    println!("Part 1\n{}", out_str);
    

    let mut state = State { stacks: initial.clone() };

    for arg in parsed_args {
        state.move_multiple(arg.0, arg.1, arg.2)
    }

    let mut out_str = String::new();
    for stack in state.stacks {
        out_str += &stack.last().unwrap().to_string();
    }
    println!("Part 2\n{}", out_str);
}
