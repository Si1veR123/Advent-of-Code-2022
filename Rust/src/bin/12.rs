// https://adventofcode.com/2022/day/12

use std::ops::{Index, IndexMut};
use std::fs::File;
use std::io::Read;
use std::time::Instant;

type Coord = (isize, isize);

#[derive(Clone, Debug)]
struct Matrix<T: Clone> {
    matrix: Vec<Vec<T>>
}

impl<T: Clone + Eq> Matrix<T> {
    fn find(&self, num: T) -> Vec<Coord> {
        let mut occurences = vec![];

        for (row_n, row) in self.matrix.iter().enumerate() {
            for (col_n, col) in row.iter().enumerate() {
                if col.clone() == num {
                    occurences.push((row_n as isize, col_n as isize))
                }
            }
        }

        occurences
    }

    fn with_shape(shape: (usize, usize), init_val: T) -> Self {
        let mut matrix_vec = vec![];
        for _row_n in 0..shape.0 {
            let mut row_vec = vec![];
            for _col_n in 0..shape.1 {
                row_vec.push(init_val.clone());
            }
            matrix_vec.push(row_vec);
        }
        Self { matrix: matrix_vec }
    }

    fn get_shape(&self) -> (usize, usize) {
        (self.matrix.len(), self.matrix.get(0).unwrap().len())
    }
}

impl<T: Clone> Index<Coord> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        self.matrix.get(index.0 as usize).unwrap().get(index.1 as usize).unwrap()
    }
}

impl<T: Clone> IndexMut<Coord> for Matrix<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        self.matrix.get_mut(index.0 as usize).unwrap().get_mut(index.1 as usize).unwrap()
    }
}

struct HeightMapPathFinder {
    heightmap: Matrix<u8>,
    start_index: Coord,
    end_index: Coord
}

impl HeightMapPathFinder {
    fn valid_step(&self, from_pos: Coord, to_pos: Coord) -> bool {
        if (to_pos.0 < 0) | (to_pos.1 < 0) {
            return false
        }

        let shape = self.heightmap.get_shape();
        if ((to_pos.0 as usize) >= shape.0) | ((to_pos.1 as usize) >= shape.1) {
            return false
        }

        let next_val = self.heightmap[to_pos] as i8;
        let first_val = self.heightmap[from_pos] as i8;
        return (next_val - first_val) <= 1
    }

    fn dijkstras(&self) -> Option<usize> {
        let mut distances = Matrix::with_shape(self.heightmap.get_shape(), i16::MIN);
        distances[self.start_index] = 0;

        let mut current_max: usize = 0;

        loop {
            let occurences = distances.find(current_max as i16);
            if occurences.len() == 0 {
                return None
            }

            // bad nesting but im lazy
            for o in occurences {
                for surrounding_pos in [(o.0+1, o.1), (o.0-1, o.1), (o.0, o.1+1), (o.0, o.1-1)] {
                    if self.valid_step(o.clone(), surrounding_pos) {
                        if distances[surrounding_pos] < 0 {
                            if surrounding_pos == self.end_index {
                                return Some(current_max + 1)
                            }
                            distances[surrounding_pos] = (current_max.clone() + 1) as i16;
                        }
                    }
                }
            }
            current_max += 1;
        }
    }
}

fn main() {
    let start = Instant::now();

    let mut file = File::open("../data/12.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");
    let lines: Vec<&str> = string_buf.split("\r\n").collect();

    let heightmap_data: Vec<Vec<u8>> = lines.iter().map(
        |&x| {
            x.chars().map(
                |c| {
                    c as u8
                }
            ).collect()
        }
    ).collect();

    let mut heightmap = Matrix {matrix: heightmap_data};
    let start_pos = heightmap.find('S' as u8).get(0).unwrap().clone();
    let end_pos = heightmap.find('E' as u8).get(0).unwrap().clone();

    heightmap[start_pos] = 'a' as u8;
    heightmap[end_pos] = 'z' as u8;

    println!("Part 1");
    let pathfinder = HeightMapPathFinder {heightmap: heightmap.clone(), start_index: start_pos, end_index: end_pos};
    let distance = pathfinder.dijkstras().unwrap();
    println!("{}", distance);

    println!("Part 2");
    let mut best_distance = usize::MAX;
    let a_positions = heightmap.find('a' as u8);
    for a_pos in a_positions {
        let pathfinder = HeightMapPathFinder {heightmap: heightmap.clone(), start_index: a_pos, end_index: end_pos};
        let distance = pathfinder.dijkstras();
        
        if distance.is_some() {
            let dist = distance.unwrap();
            if distance.unwrap() < best_distance {
                best_distance = dist;
            }
        }
    }
    println!("{}", best_distance);

    // ~200 times faster than python
    println!("Took: {:?}", Instant::now() - start);
}
