// https://adventofcode.com/2022/day/8

use std::ops::Mul;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

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

fn row_visibility_scores(row: &Vec<u8>) -> Vec<u32> {
    let mut right_visibility_score = vec![];
    let mut left_visibility_score = vec![];

    for (position, &elem) in row.iter().enumerate() {
        if (position == 0) | (position == row.len()-1) {
            right_visibility_score.push(0);
            continue;
        }

        let mut blocked = false;
        for (forward_pos, &forward_height) in row[position+1..].iter().enumerate() {
            if forward_height >= elem {
                right_visibility_score.push(forward_pos+1);
                blocked = true;
                break;
            }
        }
        if !blocked {
            right_visibility_score.push(row.len()-position-1)
        }
    };

    for (position, &elem) in row.iter().enumerate() {
        if (position == 0) | (position == row.len()-1) {
            left_visibility_score.push(0);
            continue;
        }

        let mut blocked = false;
        for (back_pos, &back_height) in row[..position].iter().rev().enumerate() {
            if back_height >= elem {
                left_visibility_score.push(back_pos+1);
                blocked = true;
                break;
            }
        }
        if !blocked {
            left_visibility_score.push(position)
        }
    };

    let horizontal_scores = left_visibility_score.iter().zip(right_visibility_score.iter()).map(
        |(&x, &y)| (x*y) as u32
    ).collect();

    horizontal_scores
}

fn element_wise_matrix_multiplication<T: Clone + Mul<T, Output = T>>(matrix1: &Vec<Vec<T>>, matrix2: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    matrix1.iter().zip(matrix2.iter()).map(
        |(row1, row2)| {
            row1.iter().zip(row2.iter()).map(
                |(elem1, elem2)| {
                    elem1.clone() * elem2.clone()
                }
            ).collect()
        }
    ).collect()
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

    let horizontal_scores: Vec<Vec<u32>> = height_matrix.iter().map(
        |row| {
            row_visibility_scores(row)
        }
    ).collect();

    let vertical_scores: Vec<Vec<u32>> = transpose(
        &transpose(&height_matrix).iter().map(
            |row| {
                row_visibility_scores(row)
            }
        ).collect()
    );
    
    let overall_scores = element_wise_matrix_multiplication(&horizontal_scores, &vertical_scores);
    // find max of nested Vecs
    let max = overall_scores.iter().fold(0, |max_row, row| max_row.max( row.iter().fold(0, |max, &elem| max.max(elem)) ));

    println!("Part 2\n{}", max);
    // about 26 times faster than python
    println!("Took: {:?}", Instant::now() - start);
}
