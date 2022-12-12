// https://adventofcode.com/2022/day/8

use std::fs::File;
use std::io::Read;

use std::time::Instant;

fn visible_in_row(row: &Vec<u8>) -> u128 {
    let mut binary_encoding: u128 = 0;

    let mut current_max_height: i8 = -1;
    let _ = row.iter().enumerate().for_each(
        |(index, &tree)| {
            if tree as i8 > current_max_height {
                current_max_height = tree as i8;
                binary_encoding |= 1 << (row.len()-index-1);
            }
        }
    );

    current_max_height = -1;
    let _ = row.iter().rev().enumerate().for_each(
        |(index, &tree)| {
            if tree as i8 > current_max_height {
                current_max_height = tree as i8;
                binary_encoding |= 1 << index;
            }
        }
    );

    binary_encoding
}

// https://stackoverflow.com/questions/29669287/how-can-i-zip-more-than-two-iterators
fn transpose<T: Clone>(rows: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed: Vec<Vec<T>> = vec![Vec::new(); rows[0].len()];

    for row in rows {
        for (index, element) in row.iter().enumerate() {
            transposed[index].push(element.clone());
        }
    }

    transposed
}

fn main() {
    let start = Instant::now();

    let mut file = File::open("../data/8.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");
    let lines: Vec<&str> = string_buf.split("\r\n").collect();

    let height_matrix: Vec<Vec<u8>> = lines.iter().map(
        |&trees_str| {
            trees_str.chars().map(
                |int_char| {
                    int_char.to_digit(10).unwrap() as u8
                }
            ).collect()
        }
    ).collect();

    let row_visibilities: Vec<u128> = height_matrix.iter().map(
        |row| {
            visible_in_row(row)
        }
    ).collect();

    // matrix of 1s and 0s, before transposing back to columns
    let col_visibilities_untransposed: Vec<Vec<u8>> = transpose(&height_matrix).iter().map(
        |row| {
            format!("{:b}", visible_in_row(row))
                .chars()
                .map(
                    |i| i.to_digit(10).unwrap() as u8
                ).collect()
        }
    ).collect();

    let col_visibilities: Vec<u128> = transpose(&col_visibilities_untransposed).iter().map(
        |row| {
            let mut bin_str = String::new();
            let _ = row.iter().for_each(|&x| bin_str.push((x+48) as char));
            u128::from_str_radix(&bin_str, 2).unwrap()
        }
    ).collect();

    let overall_visibilities_bin_string: Vec<String> = col_visibilities.iter().zip(row_visibilities.iter()).map(
        |(&a, &b)| {
            format!("{:b}", a | b)
        }
    ).collect();

    let ones_count: u32 = overall_visibilities_bin_string.iter().map(
        |x| {
            x.chars().filter(|&i| i == '1').count() as u32
        }
    ).sum();

    println!("Part 1\n{}", ones_count);
    println!("Took (remember to compile with --release): {:?}", Instant::now() - start);
}
