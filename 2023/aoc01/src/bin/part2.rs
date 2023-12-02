use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("part1.txt").unwrap();

    println!("Sum of the calibration values: {:?}", process(&input));
}

fn process(input: &str) -> i32 {

    let mut sum = 0;
    let numbers: HashMap<_, _> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].iter().cloned().collect();

    for line in input.lines() {
        let chars: Vec<_> = line.char_indices().collect();
        let mut digits: Vec<i32> = Vec::new();

        for (i, c) in chars.iter() {
            if c.is_ascii_digit() {
                digits.push(c.to_digit(10).unwrap_or(0) as i32);
            } else {
                for (key, value) in numbers.iter() {
                    if line[*i..].starts_with(key) {
                        digits.push(*value);
                        break;
                    }
                }
            }
        }

        if let (Some(&first), Some(&last)) = (digits.first(), digits.last()) {
            sum += first * 10 + last;
        }
    }

    return sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let sample = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        
        assert_eq!(process(sample), 281);
    }
}