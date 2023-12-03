use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Sum of the power of the sets: {:?}", process(&input));
}

fn ints(input: &str) -> Vec<i32> {
    input
        .split(|c: char| !c.is_numeric())
        .filter_map(|x| {
            if x.is_empty() {
                None
            } else {
                x.trim().parse().ok()
            }
        })
        .collect()
}

fn process(input: &str) -> i32 {
    let mut game_sum = 0;

    for line in input.lines() {
        let mut reds = 0;
        let mut greens = 0;
        let mut blues = 0;

        let (_, data) = line.split_once(": ").unwrap();
        for round in data.split("; ") {
            for set in round.split(", ") {
                let (count, color) = set.split_once(" ").unwrap();
                let count = ints(count)[0];
                if color == "red" {
                    reds = std::cmp::max(reds, count);
                } else if color == "green" {
                    greens = std::cmp::max(greens, count);
                } else if color == "blue" {
                    blues = std::cmp::max(blues, count);
                } else {
                    println!("Error");
                }
            }
        }

        game_sum += reds * greens * blues;
    }
    return game_sum;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";        
        assert_eq!(process(sample), 2286);
    }
}