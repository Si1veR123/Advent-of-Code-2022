// https://adventofcode.com/2022/day/13

use std::fs::File;
use std::io::Read;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone)]
enum PacketValue {
    Int(u8),
    List(Vec<PacketValue>)
}

fn parse_line(line: &str) -> PacketValue {
    // recursively parse nested lists of ints
    let mut current_vec = vec![];

    let mut char_iter = line[1..].chars();

    let mut i = 0;
    loop {
        let next_char = char_iter.next();
        if next_char.is_none() {
            break
        }

        let next_value: Option<PacketValue> = match next_char.unwrap() {
            '[' => {
                // add one to start because first square bracket is skipped but isnt included in i
                let index_of_list_start = i.clone() + 1;
                let mut nested_list_level = 0;
                loop {
                    let next_char_list_check = char_iter.next().expect("Error trying to find a ] after a [");

                    if next_char_list_check == '[' {
                        nested_list_level += 1
                    }
                    else if next_char_list_check == ']' {
                        if nested_list_level == 0 {
                            i += 1;
                            break
                        } else {
                            nested_list_level -= 1
                        }
                    }

                    i += 1;
                }
                
                Some(parse_line(&line[index_of_list_start..(i+2)]))
            },
            ']' => None, // should be end of string after this
            ',' => None,
            ' ' => None,
            c => {
                // should be a number
                let digit = c.to_digit(10).expect("Number not found where expected");
                let next_char = char_iter.next().unwrap();
                i += 1;

                (|| {
                    if let Some(digit_2) = next_char.to_digit(10) {
                        return Some(PacketValue::Int((digit*10 + digit_2) as u8))
                    }
                    return Some(PacketValue::Int(digit as u8))
                })()
            }
        };

        if next_value.is_some() {
            current_vec.push(next_value.unwrap())
        }

        i += 1;
    }
    PacketValue::List(current_vec)
}

fn compare_values(val1: &PacketValue, val2: &PacketValue) -> Option<bool> {
    let type_1_is_int = match &val1 {
        PacketValue::Int(_) => true,
        _ => false
    };

    let type_2_is_int = match &val2 {
        PacketValue::Int(_) => true,
        _ => false
    };

    if type_1_is_int & type_2_is_int {
        if let PacketValue::Int(type_1_i) = val1 {
            if let PacketValue::Int(type_2_i) = val2 {
                if type_1_i == type_2_i {
                    return None
                } else {
                    return Some(type_1_i < type_2_i)
                }
            }
        }
    }

    if type_1_is_int {
        let val = compare_values(&PacketValue::List(vec![val1.clone()]), val2);
        return val;
    }

    if type_2_is_int {
        let val = compare_values(val1, &PacketValue::List(vec![val2.clone()]));
        return val;
    }

    if let PacketValue::List(val1_as_list) = val1 {
        if let PacketValue::List(val2_as_list) = val2 {
            for i in 0..val1_as_list.len().max(val2_as_list.len()) {
                let left_list_value = val1_as_list.get(i);
                if left_list_value.is_none() {
                    return Some(true)
                }
                let right_list_value = val2_as_list.get(i);
                if right_list_value.is_none() {
                    return Some(false)
                }
        
                let result = compare_values(left_list_value.unwrap(), right_list_value.unwrap());
                if result.is_none() {
                    continue;
                } else {
                    return result
                }
            }
        }
    }
    None
}


fn main() {
    let start = Instant::now();

    let mut file = File::open("../data/13.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");
    let lines: Vec<&str> = string_buf.split("\r\n").collect();

    let mut parsed_packets = Vec::new();
    lines.iter().for_each(
        |&line| {
            if line.len() > 0 {
                parsed_packets.push(parse_line(line))
            }
        }
    );

    let mut count = 0;
    for i in 0..(parsed_packets.len() / 2) {
        let first_of_pair = &parsed_packets[i*2];
        let second_of_pair = &parsed_packets[(i*2)+1];
        if compare_values(first_of_pair, second_of_pair).unwrap() {
            count += i+1;
        }
    }
    println!("Part 1");
    println!("{}", count);

    let packet1 = PacketValue::List(vec![PacketValue::List(vec![PacketValue::Int(2)])]);
    let packet2 = PacketValue::List(vec![PacketValue::List(vec![PacketValue::Int(6)])]);
    parsed_packets.push(packet1.clone());
    parsed_packets.push(packet2.clone());

    let mut ordered = false;
    while !ordered {
        ordered = true;
        for i in 0..parsed_packets.len()-1 {
            let (left, right) = parsed_packets.split_at_mut(i+1);
            let first_val = left.last_mut().unwrap();
            let second_val = right.first_mut().unwrap();

            if !compare_values(first_val, second_val).unwrap() {
                std::mem::swap(first_val, second_val);
                ordered = false;
            }
        }
    }

    let packet_1_loc = parsed_packets.iter().position(|x| x == &packet1).unwrap()+1;
    let packet_2_loc = parsed_packets.iter().position(|x| x == &packet2).unwrap()+1;
    println!("Part 2");
    println!("{}", packet_1_loc*packet_2_loc);

    println!("Took: {:?}", Instant::now() - start);
}
