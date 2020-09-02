use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let start = 246540;
    let end = 787419;

    let passed: Vec<i32> = (start..end)
        .filter(|d| has_six_digits(d) && has_non_decreasing_digits(d) && has_doubles(d))
        .collect();

    println!("Total possible passwords, part 1: {}", passed.len());
}

fn part2() {
    let start = 246540;
    let end = 787419;

    let passed: Vec<i32> = (start..end)
        .filter(|d| {
            has_six_digits(d) && has_non_decreasing_digits(d) && has_only_adjacent_doubles(d)
        })
        .collect();

    println!("Total possible passwords, part 2: {}", passed.len());
}

fn has_six_digits(d: &i32) -> bool {
    d.to_string().len() == 6
}

fn has_non_decreasing_digits(d: &i32) -> bool {
    let numbers: String = d.to_string();
    let mut prev: i32 = 0;
    let mut digit: i32;

    for i in 0..numbers.len() {
        digit = numbers[i..i + 1].parse().unwrap_or(0);
        if prev > digit {
            return false;
        }
        prev = digit;
    }

    true
}

fn has_doubles(d: &i32) -> bool {
    let numbers: String = d.to_string();
    let mut prev = '\0';

    for ch in numbers.chars() {
        if prev == ch {
            return true;
        }
        prev = ch;
    }

    false
}

fn has_only_adjacent_doubles(d: &i32) -> bool {
    let numbers: String = d.to_string();
    let mut counts: HashMap<char, i32> = HashMap::new();

    for ch in numbers.chars() {
        let counter = counts.entry(ch).or_insert(0);
        *counter += 1;
    }
    counts.values().any(|&i| i == 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_six_digits() {
        assert_eq!(has_six_digits(&1), false);
        assert_eq!(has_six_digits(&123456), true);
        assert_eq!(has_six_digits(&1234567890), false);
    }

    #[test]
    fn test_non_decreasing_digits() {
        assert_eq!(has_non_decreasing_digits(&123), true);
        assert_eq!(has_non_decreasing_digits(&132), false);
    }

    #[test]
    fn test_has_double() {
        assert_eq!(has_doubles(&1234), false);
        assert_eq!(has_doubles(&1223), true);
    }

    #[test]
    fn test_has_only_adjacent_doubles() {
        assert_eq!(has_only_adjacent_doubles(&123444), false);
        assert_eq!(has_only_adjacent_doubles(&112233), true);
        assert_eq!(has_only_adjacent_doubles(&111122), true);
    }
}
