// https://adventofcode.com/2022/day/11

use std::fs::File;
use std::io::Read;
use std::time::Instant;

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: String,
    divisibility_test: u64,
    truthy_monkey_id: u8,
    falsy_monkey_id: u8,
    inspect_count: usize,
    worry_level_reduction: u64,
}


impl Monkey {
    fn receive_item(&mut self, worry_level: u64) {
        self.items.push(worry_level)
    }

    fn inspect_item(&mut self, index: usize, common_multiple: u64) -> u8 {
        self.inspect_count += 1;

        let old = self.items[index];

        // self.operation = e.g. '+ 6' or '* 13' or '* old'
        let op_digit_parse_attempt: Result<u64, _> = self.operation[2..].parse();

        let worry_level;
        if op_digit_parse_attempt.is_ok() {
            worry_level = match self.operation.chars().nth(0).unwrap() {
                '+' => old + op_digit_parse_attempt.unwrap(),
                '*' => old * op_digit_parse_attempt.unwrap(),
                _ => panic!("Unidentified operation")
            };
        } else {
            // assumed to be old * old if second thing isnt a number
            worry_level = old.pow(2);
        }

        let bored_level = (worry_level/self.worry_level_reduction) % common_multiple;

        self.items[index] = bored_level;

        if (bored_level % self.divisibility_test) == 0 {
            return self.truthy_monkey_id
        }
        return self.falsy_monkey_id
    }

    fn execute_turn(&mut self, common_multiple: u64) -> (Vec<u8>, Vec<u64>) {
        let mut receiving_monkeys = vec![];
        for i in 0..self.items.len() {
            receiving_monkeys.push(self.inspect_item(i, common_multiple))
        }
        let to_return = (receiving_monkeys, self.items.clone());
        self.items.clear();
        to_return
    }
}

fn parse_monkeys(lines: &Vec<&str>, worry_level_reduction: u64) -> Vec<Monkey> {
    let mut monkey_lines_data: Vec<Vec<&str>> = vec![];
    let mut current_monkey: Vec<&str> = Vec::with_capacity(6);

    for &line in lines {
        let line = line.trim();
        if line.len() == 0 {
            monkey_lines_data.push(current_monkey);
            current_monkey = vec![];
        } else {
            current_monkey.push(line)
        }
    }
    monkey_lines_data.push(current_monkey);

    let mut monkeys: Vec<Monkey> = vec![];
    for monkey_data in monkey_lines_data {
        let starting_items_string = &monkey_data[1][16..];
        let start_items_parsed: Vec<u64> = starting_items_string.split(", ").map(|x| x.parse().unwrap()).collect();
        let operation_string = &monkey_data[2][21..];
        let divisible_by_test = monkey_data[3][19..].parse().unwrap();
        let truthy_monkey: u8 = monkey_data[4].chars().nth(25).unwrap().to_digit(10).unwrap() as u8;
        let falsy_monkey: u8 = monkey_data[5].chars().nth(26).unwrap().to_digit(10).unwrap() as u8;

        monkeys.push(
            Monkey {
                items: start_items_parsed,
                operation: operation_string.to_string(),
                divisibility_test: divisible_by_test,
                truthy_monkey_id: truthy_monkey,
                falsy_monkey_id: falsy_monkey,
                inspect_count: 0,
                worry_level_reduction
            }
        )
    }

    monkeys
}

fn simulate_monkeys(monkeys: Vec<Monkey>, rounds: usize) -> usize {
    let mut common_multiple = 1;
    for monkey in &monkeys {
        common_multiple *= monkey.divisibility_test;
    }

    let mut monkeys = monkeys;
    for _ in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            let (receiving_monkeys, items) = monkeys.get_mut(monkey_index).unwrap().execute_turn(common_multiple);

            for (&receiving_monkey, &item) in receiving_monkeys.iter().zip(items.iter()) {
                monkeys[receiving_monkey as usize].receive_item(item);
            }
        }
    }

    let mut inspects: Vec<usize> = monkeys.iter().map(|x| x.inspect_count).collect();
    inspects.sort_unstable();
    return inspects.last().unwrap() * inspects.get(inspects.len() - 2).unwrap()
}

fn main() {
    let start = Instant::now();

    let mut file = File::open("../data/11.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");
    let lines: Vec<&str> = string_buf.split("\r\n").collect();

    let monkeys = parse_monkeys(&lines, 3);
    let monkey_business = simulate_monkeys(monkeys, 20);
    println!("Part 1");
    println!("{}", monkey_business);

    let monkeys = parse_monkeys(&lines, 1);
    let monkey_business_2 = simulate_monkeys(monkeys, 10000);

    println!("Part 2");
    println!("{}", monkey_business_2);

    // 163 times faster than python (27ms vs 4.4s)
    println!("Took: {:?}", Instant::now() - start);
}
