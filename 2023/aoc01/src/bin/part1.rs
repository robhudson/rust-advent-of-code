use std::fs;

fn main() {
    let input = fs::read_to_string("part1.txt").unwrap();

    println!("Sum of the calibration values: {:?}", process(&input));
}

fn process(input: &str) -> i32 {

    let mut sum = 0;

    for line in input.lines() {
        let digits: Vec<_> = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap_or(0) as i32)
            .collect();

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
        let sample = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(process(sample), 142);
    }
}