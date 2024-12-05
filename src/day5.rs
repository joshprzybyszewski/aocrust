fn convert_bytes(a: u8, b: u8) -> usize {
    // b'0' is value 48 in ascii. = 0b00110000
    // (byte >> 4) - 3 => converts ascii to num?
    // let b: usize = input[i + 3] as usize * 10 + input[1 + 4] as usize - 48 * 11;
    return ((a - b'0') * 10 + (b - b'0')) as usize;
}

const GRID_SIZE: usize = 100;

fn check_valid(requires: [[bool; GRID_SIZE]; GRID_SIZE], line: &Vec<usize>, b: usize) -> bool {
    for a in line {
        if requires[b][*a] {
            return false;
        }
    }
    return true;
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    // 1176 rules

    let input = input.as_bytes();
    let mut i: usize = 0;

    let mut requires: [[bool; GRID_SIZE]; GRID_SIZE] = [[false; GRID_SIZE]; GRID_SIZE];

    while i < input.len() {
        let a: usize = convert_bytes(input[i], input[i + 1]);
        // input[i+ 2] is b'|'
        let b: usize = convert_bytes(input[i + 3], input[i + 4]);
        // input[i+5] is b'\n'
        requires[a][b] = true;

        i += 6;
        if input[i] == b'\n' {
            i += 1;
            break;
        }
    }

    let mut line: Vec<usize> = Vec::with_capacity(23);
    let mut is_valid = true;
    let mut sum = 0;
    loop {
        if i > 16400 {
            println!("i = {i}");
        }
        if is_valid {
            let a: usize = convert_bytes(input[i], input[i + 1]);
            if i > 16400 {
                println!("a = {a}");
            }

            is_valid = check_valid(requires, &line, a);
            // println!("a, is_valid = {a}, {is_valid}");
            if !is_valid {
                println!("INVALID: cause {a}");
            }

            line.push(a);
        }

        i += 2;
        if i >= input.len() - 1 || input[i] == b'\n' {
            if is_valid {
                println!("line valid. middle element {}", line[line.len() / 2]);
                sum += line[line.len() / 2]
            } else {
                println!(" INVALID. middle element {}", line[line.len() / 2]);
            }
            line.clear();
            is_valid = true;
            if i >= input.len() - 1 {
                return sum;
            }
        }
        i += 1;
    }

    // 5664 is too low
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_example_input() -> String {
        let input_path = "input/2024/examples/day5.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_input() -> String {
        let input_path = "input/2024/day5.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), 0);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 153469856)
    }

    // #[test]
    // fn part2_real_input() {
    //     assert_eq!(part2(&get_input()), 77055967)
    // }
}
