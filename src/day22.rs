// decimal: 16777216
// hex:     0x1000000
// 25 binary digits = 0b1000000000000000000000000
const MODULO_MASK: i32 = 0xFFFFFF;
const NUM_ITERATIONS: usize = 2000;

const fn generate_times(secret: i32) -> i32 {
    let mut val = secret;
    let mut i = 0;
    loop {
        if i == NUM_ITERATIONS {
            break;
        }
        val = generate(val);
        i += 1;
    }
    return val;
}

const fn generate(secret: i32) -> i32 {
    // multiplying the secret number by 64
    // mix this result, then prune
    let secret = (secret ^ (secret << 6)) & MODULO_MASK;
    // dividing the secret number by 32
    // mix, then prune
    let secret = (secret ^ (secret >> 5)) & MODULO_MASK;
    // multiplying the secret number by 2048
    // mix, then prune
    return (secret ^ (secret << 11)) & MODULO_MASK;
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut i: usize = 0;
    let mut val = 0;
    let mut total: u64 = 0;
    loop {
        if input[i] == b'\n' {
            total += generate_times(val) as u64;
            i += 1;
            val = 0;
        }
        val *= 10;
        val += (input[i] - b'0') as i32;
        i += 1;
        if i >= input.len() {
            total += generate_times(val) as u64;
            break;
        }
    }
    return total;
}

fn consider_part2(secret: i32) -> i32 {
    let mut val = secret;
    let mut i = 0;
    loop {
        if i == NUM_ITERATIONS {
            break;
        }
        val = generate(val);
        i += 1;
    }
    return val;
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> i32 {
    let input = input.as_bytes();
    let mut i: usize = 0;
    let mut val = 0;
    let mut total = 0;
    loop {
        if input[i] == b'\n' {
            total += consider_part2(val);
            i += 1;
            val = 0;
        }
        val *= 10;
        val += (input[i] - b'0') as i32;
        i += 1;
        if i >= input.len() {
            total += consider_part2(val);
            break;
        }
    }
    return total;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day22.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_example_input() -> &'static str {
        return "1
10
100
2024";
    }

    fn get_example_input_2() -> &'static str {
        return "1
2
3
2024";
    }

    #[test]
    fn part1_example() {
        assert_eq!(generate(123), 15887950);
        assert_eq!(generate(generate(123)), 16495136);
        assert_eq!(generate(generate(generate(123))), 527345);
        assert_eq!(generate(generate(generate(generate(123)))), 704524);
        assert_eq!(part1(&get_example_input()), 37327623);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 16299144133)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input_2()), 23);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 1)
    }
}
