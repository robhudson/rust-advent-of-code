use std::io::{self, Read};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Could not read file");

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut wire1: Vec<Point> = Vec::new();
    let mut wire2: Vec<Point> = Vec::new();
    let mut intersections: Vec<Point> = Vec::new();

    let lines: Vec<&str> = input.split_whitespace().collect();

    to_coordinates(lines[0], &mut wire1);
    to_coordinates(lines[1], &mut wire2);

    for x in &wire1 {
        if wire2.iter().any(|y| x.x == y.x && x.y == y.y) {
            intersections.push(*x);
        }
    }

    let mut distances: Vec<i32> = Vec::new();
    for i in &intersections {
        distances.push(i.x.abs() + i.y.abs());
    }

    println!(
        "The Manhattan distance is: {}",
        distances.iter().min().unwrap_or(&0)
    );
}

fn part2(input: &str) {
    let mut wire1: Vec<Point> = Vec::new();
    let mut wire2: Vec<Point> = Vec::new();
    let mut intersections: Vec<Point> = Vec::new();

    let lines: Vec<&str> = input.split_whitespace().collect();

    to_coordinates(lines[0], &mut wire1);
    to_coordinates(lines[1], &mut wire2);

    for x in &wire1 {
        if wire2.iter().any(|y| x.x == y.x && x.y == y.y) {
            intersections.push(*x);
        }
    }

    let mut distances: Vec<u32> = Vec::new();
    for i in &intersections {
        distances.push(len_to_intersection(&wire1, &i) + len_to_intersection(&wire2, &i))
    }

    println!(
        "The fewest combined steps is: {}",
        distances.iter().min().unwrap_or(&0)
    )
}

fn len_to_intersection(wire: &[Point], intersection: &Point) -> u32 {
    let mut wirelen: u32 = 0;

    // Calculate wire distances to each intersection.
    for p in wire {
        if intersection.x == p.x && intersection.y == p.y {
            wirelen += 1;
            break;
        } else {
            wirelen += 1;
        }
    }

    wirelen
}

// Note: The part1 implementation in a prior commit used `HashSet` which was
// fast, but was changed for part2 so that I could walk the wire to determine
// its length up to an intersection, which is much less efficient at finding
// intersections since it is O(n^2). But I'm just running with it.
fn to_coordinates(input: &str, wire: &mut Vec<Point>) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for s in input.split(',') {
        let mut c = s.chars();
        let direction = c.next();
        let length: u32 = s[1..s.len()].parse().unwrap_or(0);

        match direction {
            Some('D') => {
                for _i in 0..length {
                    y -= 1;
                    wire.push(Point { x, y });
                }
            }
            Some('L') => {
                for _i in 0..length {
                    x -= 1;
                    wire.push(Point { x, y });
                }
            }
            Some('R') => {
                for _i in 0..length {
                    x += 1;
                    wire.push(Point { x, y });
                }
            }
            Some('U') => {
                for _i in 0..length {
                    y += 1;
                    wire.push(Point { x, y });
                }
            }
            _ => {
                eprintln!("Unknown direction: {:?}", direction);
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates() {
        let mut actual: Vec<Point> = Vec::new();
        to_coordinates("R4,U3,L2,D1", &mut actual);
        let expected = vec![
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 4, y: 1 },
            Point { x: 4, y: 2 },
            Point { x: 4, y: 3 },
            Point { x: 3, y: 3 },
            Point { x: 2, y: 3 },
            Point { x: 2, y: 2 },
        ];
        for i in 0..expected.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn test_length() {
        let mut wire: Vec<Point> = Vec::new();
        to_coordinates("R4,U3,L2,D1", &mut wire);
        let intersection = Point { x: 4, y: 3 };
        assert_eq!(len_to_intersection(&wire, &intersection), 7);
    }
}
