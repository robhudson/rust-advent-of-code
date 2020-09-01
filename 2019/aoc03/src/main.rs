use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Could not read file");

    part1(&input);
}

fn part1(input: &str) {
    let mut wire1: HashSet<(i32, i32)> = HashSet::new();
    let mut wire2: HashSet<(i32, i32)> = HashSet::new();
    let mut intersections: HashSet<(i32, i32)> = HashSet::new();

    let lines: Vec<&str> = input.split_whitespace().collect();

    to_coordinates(lines[0], &mut wire1);
    to_coordinates(lines[1], &mut wire2);

    for x in wire1.intersection(&wire2) {
        intersections.insert(*x);
    }

    let mut distances: Vec<i32> = Vec::new();

    for i in &intersections {
        distances.push(i.0.abs() + i.1.abs());
    }

    println!(
        "The Manhattan distance is: {}",
        distances.iter().min().unwrap_or(&0)
    );
}

fn to_coordinates(input: &str, wire: &mut HashSet<(i32, i32)>) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for s in input.split(',') {
        let mut c = s.chars();
        let direction = c.next();
        let length: u32 = s[1..s.len()].parse::<u32>().unwrap_or(0);

        match direction {
            Some('D') => {
                for _i in 0..length {
                    y -= 1;
                    wire.insert((x, y));
                }
            }
            Some('L') => {
                for _i in 0..length {
                    x -= 1;
                    wire.insert((x, y));
                }
            }
            Some('R') => {
                for _i in 0..length {
                    x += 1;
                    wire.insert((x, y));
                }
            }
            Some('U') => {
                for _i in 0..length {
                    y += 1;
                    wire.insert((x, y));
                }
            }
            _ => {
                eprintln!("Unknown direction: {:?}", direction);
                std::process::exit(1);
            }
        }
    }
}
