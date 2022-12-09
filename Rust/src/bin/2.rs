// https://adventofcode.com/2022/day/2

use std::{fs::File, io::Read};

// ======== ENUM TYPES ========

#[derive(PartialEq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl From<char> for Move {
    fn from(n: char) -> Self {
        match n {
            'A'|'X' => Self::Rock,
            'B'|'Y' => Self::Paper,
            'C'|'Z' => Self::Scissors,
            _ => panic!("Invalid Move integer")
        }
    }
}

impl From<Move> for u8 {
    fn from(m: Move) -> Self {
        match m {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }
}

enum GameResult {
    Win = 2,
    Draw = 1,
    Lose = 0
}

impl From<char> for GameResult {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Invalid game result")
        }
    }
}
// ============================================


fn roll_vec<T>(v: Vec<T>, k: usize) -> Vec<T> {
    let mut v = v;
    for _ in 0..k {
        let first = v.remove(0);
        v.push(first);
    }
    v
}

fn eval_round(move1_c: char, move2_c: char) -> u8 {
    let move1 = move1_c.into();
    let move2 = move2_c.into();

    if move1 == move2 {
        return <Move as Into<u8>>::into(move2) + 3
    }

    let moveset = (&move1, &move2);
    if (moveset == (&Move::Rock, &Move::Paper)) |
       (moveset == (&Move::Paper, &Move::Scissors)) |
       (moveset == (&Move::Scissors, &Move::Rock))
    {
        return <Move as Into<u8>>::into(move2) + 6u8
    }

    move2.into()
}

fn eval_round_2(move1_c: char, game_result_c: char) -> u8 {
    let move1 = move1_c.into();
    let game_result: GameResult = game_result_c.into();

    let moves_vec: Vec<char> = vec!['Z', 'X', 'Y'];
    let moves_vec_rot = roll_vec(moves_vec, game_result as usize);

    let correct_move: &char = moves_vec_rot
        .get((<Move as Into<u8>>::into(move1)-1) as usize)
        .unwrap();
    eval_round(move1_c, correct_move.clone())
}

fn parse_and_apply<F>(lines: &Vec<&str>, apply: F) -> u32
    where F: Fn(char, char) -> u8
{
    lines.iter().map(
        |&x| {
            let mut args_char = x.split_whitespace();
            apply(
                (args_char.next().unwrap().chars().next().unwrap()).into(),
                (args_char.next().unwrap().chars().next().unwrap()).into()
            ) as u32
        }
    ).sum()
}

fn main() {
    let mut file = File::open("../data/2.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");
    let lines: Vec<&str> = string_buf.split("\r\n").collect();

    println!("Part 1");
    let points = parse_and_apply(&lines, eval_round);
    println!("{}", points);

    println!("Part 2");
    let points_2 = parse_and_apply(&lines, eval_round_2);
    println!("{}", points_2);
}