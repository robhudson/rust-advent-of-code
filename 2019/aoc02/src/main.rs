use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Could not read file");
    let input: Vec<u32> = input.split(",").map(|s| s.parse().unwrap_or(0)).collect();

    let mut intcode = input.to_vec();
    part1(&mut intcode);

    intcode = input.to_vec();
    part2(&mut intcode);
}

fn part1(intcode: &mut Vec<u32>) {
    // Reset our intcode to the state it had before the fire.
    intcode[1] = 12;
    intcode[2] = 2;

    // Process the intcode.
    for i in (0..intcode.len()).step_by(4) {
        if intcode[i] == 99 {
            break;
        }
        let (opcode, in1, in2, output) = (
            intcode[i],
            intcode[intcode[i + 1] as usize],
            intcode[intcode[i + 2] as usize],
            intcode[i + 3],
        );
        let val: u32 = process(opcode, in1, in2);
        intcode[output as usize] = val;
    }

    println!("Value at position zero: {}", intcode[0]);
}

fn part2(intcode: &mut Vec<u32>) {
    'outer: for x in 0..100 {
        for y in 0..100 {
            let mut code = intcode.to_vec();

            code[1] = x;
            code[2] = y;

            // Process the intcode.
            for i in (0..code.len()).step_by(4) {
                if code[i] == 99 {
                    break;
                }
                let (opcode, in1, in2, output) = (
                    code[i],
                    code[code[i + 1] as usize],
                    code[code[i + 2] as usize],
                    code[i + 3],
                );
                if output as usize >= code.len() {
                    println!("Output index exceeds length: {}", output);
                    break;
                }
                let val: u32 = process(opcode, in1, in2);
                code[output as usize] = val;
            }

            if code[0] == 19690720 {
                println!(
                    "noun={}, verb={}, 100 * noun + verb = {}",
                    x,
                    y,
                    100 * x + y
                );
                break 'outer;
            }
        }
    }
}

fn process(opcode: u32, in1: u32, in2: u32) -> u32 {
    match opcode {
        1 => return in1 + in2,
        2 => return in1 * in2,
        _ => return 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process(1, 2, 3), 5);
        assert_eq!(process(2, 4, 5), 20);
        assert_eq!(process(100, 1, 1), 0);
    }
}
