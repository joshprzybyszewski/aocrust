// decimal: 16777216
// hex:     0x1000000
// 25 binary digits = 0b1000000000000000000000000
const MODULO_MASK: i32 = 0xFFFFFF;

fn generate_times(secret: i32, num_times: usize) -> i32 {
    let mut val = secret;
    for _ in 0..num_times {
        val = generate(val);
    }
    return val;
}

fn generate(secret: i32) -> i32 {
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
            total += generate_times(val, 2000) as u64;
            i += 1;
            val = 0;
        }
        val *= 10;
        val += (input[i] - b'0') as i32;
        i += 1;
        if i >= input.len() {
            total += generate_times(val, 2000) as u64;
            break;
        }
    }
    return total;
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> u32 {
    return 0;
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

    #[test]
    fn part1_example() {
        assert_eq!(generate(123), 15887950);
        assert_eq!(generate_times(123, 1), 15887950);
        assert_eq!(generate_times(123, 2), 16495136);
        assert_eq!(generate_times(123, 3), 527345);
        assert_eq!(generate_times(123, 4), 704524);
        assert_eq!(part1(&get_example_input()), 37327623);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 16299144133)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), 1);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 1)
    }
}
