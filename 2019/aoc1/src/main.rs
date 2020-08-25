use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Could not read file");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut total: i32 = 0;

    for line in input.lines() {
        let mass: i32 = line.parse().unwrap_or(0);
        total += (mass / 3) - 2;
    }
    println!("Fuel requirements for all modules: {}", total);
}

fn part2(input: &str) {
    let mut total: i32 = 0;

    for line in input.lines() {
        let mass: i32 = line.parse().unwrap_or(0);

        let mut subtotal: i32 = 0;
        let mut fuel: i32 = (mass / 3) - 2;

        loop {
            if fuel <= 0 {
                break;
            }
            subtotal += fuel;
            fuel = (fuel / 3) - 2;
        }

        total += subtotal;
    }
    println!("Fuel requirements for all modules + fuel: {}", total);
}
