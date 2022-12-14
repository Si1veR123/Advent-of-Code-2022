// https://adventofcode.com/2022/day/9

use std::fs::File;
use std::io::Read;
use std::time::Instant;

struct Knot {
    location: (i16, i16),
    child: Option<Box<Knot>>,
    location_cache: Vec<(i16, i16)>
}

impl Knot {
    fn new(child: Option<Box<Knot>>) -> Self {
        Self {
            location: (0, 0),
            child,
            location_cache: vec![(0, 0)]
        }
    }

    fn adjacent_to_child(&self) -> bool {
        if self.child.is_none() {
            return true
        }
        let child = self.child.as_ref().unwrap();
        return ( (self.location.0 - child.location.0).abs() <= 1 ) & ( (self.location.1-child.location.1).abs() <= 1 )
    }

    fn cache_loc(&mut self) {
        let found = self.location_cache.iter().position(|&x| x == self.location);
        if found.is_none() {
            self.location_cache.push(self.location.clone())
        }
    }

    fn calculate_next_move(&self) -> (i16, i16) {
        let child_ref = self.child.as_ref().unwrap();
        if &self.location == &child_ref.location {
            (0, 0)
        }

        else if self.location.0 == child_ref.location.0 {
            // same column

            if self.location.1 > child_ref.location.1 {
                return (0, 1)
            }
            else if self.location.1 < child_ref.location.1 {
                return (0, -1)
            }

            panic!("Shouldn't get here");
        }

        else if self.location.1 == child_ref.location.1 {
            // same row

            if self.location.0 > child_ref.location.0 {
                return (1, 0)
            }
            else if self.location.0 < child_ref.location.0 {
                return (-1, 0)
            }

            panic!("Shouldn't get here");
        }
        else {
            let vector = (self.location.0 - child_ref.location.0, self.location.1 - child_ref.location.1);
            return (if vector.0 < 0 {-1} else {1}, if vector.1 < 0 {-1} else {1})
        }
    }

    fn move_by(&mut self, rel_change: &(i16, i16)) {
        self.location = (self.location.0 + rel_change.0, self.location.1 + rel_change.1)
    }

    fn move_catch_up(&mut self, rel_change: (i16, i16)) {
        self.move_by(&rel_change);
        self.cache_loc();

        if !self.adjacent_to_child() {
            let next_move = self.calculate_next_move();
            self.child.as_mut().unwrap().move_catch_up(next_move)
        }
    }

    fn move_head(&mut self, direction: char, n: usize) {
        for _ in 0..n {
            let next_move;
            if direction == 'U' {
                next_move = (0, 1)
            }
            else if direction == 'D' {
                next_move = (0, -1)
            }
            else if direction == 'L' {
                next_move = (-1, 0)
            }
            else {
                // assumed right direction
                next_move = (1, 0)
            }
            self.move_catch_up(next_move)
        }
    }

    fn get_tail(&self) -> &Knot {
        if self.child.is_some() {
            return self.child.as_ref().unwrap().get_tail()
        }
        return self
    }
}

fn main() {
    let start = Instant::now();

    let mut file = File::open("../data/9.txt").expect("Data file not found");
    let mut string_buf = String::new();
    file.read_to_string(&mut string_buf).expect("Reading file failed");
    let lines: Vec<&str> = string_buf.split("\r\n").collect();

    // main function owns the first knot, which owns the second knot (through a boxed ref) etc.
    // reference to tail is found through the head
    let mut first_head;
    {
        let rope_tail = Knot::new(None);
        let rope_head = Knot::new(Some(Box::new(rope_tail)));
        first_head = rope_head;
    }

    for &line in &lines {
        let (direction, n) = line.split_once(' ').unwrap();
        first_head.move_head(direction.chars().next().unwrap(), n.parse().unwrap());
    }

    println!("Part 1");
    println!("{}", first_head.get_tail().location_cache.len());

    let mut second_head;
    {
        let mut last_knot = None;
        for _ in 0..10 {
            last_knot = Some(Box::new(Knot::new(last_knot)))
        }
        second_head = last_knot.unwrap();
    }

    for &line in &lines {
        let (direction, n) = line.split_once(' ').unwrap();
        second_head.move_head(direction.chars().next().unwrap(), n.parse().unwrap());
    }

    println!("Part 2");
    println!("{:?}", second_head.get_tail().location_cache.len());
    // about 50 times faster than python
    println!("Took {:?}", Instant::now() - start);
}
