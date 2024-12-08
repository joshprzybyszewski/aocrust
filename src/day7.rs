// See https://docs.rs/tokio/latest/tokio/index.html#feature-flags
use tokio::runtime::Runtime;

const MAX_LINE_LEN: usize = 13;
const POWERS_OF_TEN: [u64; MAX_LINE_LEN] = [
    1, // the zero'th element will never be queried
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
];

async fn check1(target: u64, line: [u64; MAX_LINE_LEN]) -> u64 {
    if check1_inner(target, &line, line[0], 1) {
        return target;
    }
    return 0;
}

fn check1_inner(target: u64, line: &[u64; MAX_LINE_LEN], cur: u64, index: usize) -> bool {
    if cur > target {
        return false;
    }

    let next = line[index];
    if next == 0 {
        return target == cur;
    }

    // check the cheap cost first.
    // add
    if check1_inner(target, line, cur + next, index + 1) {
        return true;
    }

    // mul
    if check1_inner(target, line, cur * next, index + 1) {
        return true;
    }

    return false;
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    let rt = Runtime::new().unwrap();
    let mut total = 0;
    rt.block_on(async {
        let input = input.as_bytes();
        let mut i: usize = 0;
        let mut tasks = tokio::task::JoinSet::new();

        while i < input.len() {
            let mut target: u64 = 0;
            // yes, I know simd can do this way faster
            while input[i] != b':' {
                // end char is the ":" for the first elem.
                // b':'  = 58
                target *= 10;
                target += (input[i] - b'0') as u64;
                i += 1;
            }

            let target: u64 = target;

            // skip past ": "
            i += 2;

            let mut line: [u64; MAX_LINE_LEN] = [0; MAX_LINE_LEN];
            let mut l_i = 0;

            loop {
                // ending char is a space or a newline.
                // b'\n' = 10
                // b' '  = 32
                // b'0'  = 48
                // we only expect digits in this loop
                while input[i] >= b'0' {
                    line[l_i] *= 10;
                    line[l_i] += (input[i] - b'0') as u64;
                    i += 1;
                }
                l_i += 1;

                if input[i] == b'\n' {
                    // skip past the newline
                    i += 1;
                    break;
                }
                i += 1;
            }

            let line: [u64; MAX_LINE_LEN] = line;
            tasks.spawn(check1(target, line));
        }

        while let Some(res) = tasks.join_next().await {
            total += res.unwrap();
        }
    });

    return total;
}

async fn check2(target: u64, line: [u64; MAX_LINE_LEN], digits: [usize; MAX_LINE_LEN]) -> u64 {
    if check1_inner(target, &line, line[0], 1) {
        return target;
    }
    if check2_inner(target, &line, &digits, line[0], 1) {
        return target;
    }
    return 0;
}

fn check2_inner(
    target: u64,
    line: &[u64; MAX_LINE_LEN],
    digits: &[usize; MAX_LINE_LEN],
    cur: u64,
    index: usize,
) -> bool {
    if cur > target {
        return false;
    }

    let next = line[index];
    if next == 0 {
        return target == cur;
    }

    // check in the order that produces that highests cur value.

    // concat
    if check2_inner(
        target,
        line,
        digits,
        cur * POWERS_OF_TEN[digits[index]] + next,
        index + 1,
    ) {
        return true;
    }

    // mul
    if check2_inner(target, line, digits, cur * next, index + 1) {
        return true;
    }

    // add
    if check2_inner(target, line, digits, cur + next, index + 1) {
        return true;
    }

    return false;
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let rt = Runtime::new().unwrap();
    let mut total = 0;
    rt.block_on(async {
        let input = input.as_bytes();
        let mut i: usize = 0;
        let mut tasks = tokio::task::JoinSet::new();

        while i < input.len() {
            let mut target: u64 = 0;
            // yes, I know simd can do this way faster
            while input[i] != b':' {
                // end char is the ":" for the first elem.
                // b':'  = 58
                target *= 10;
                target += (input[i] - b'0') as u64;
                i += 1;
            }

            let target: u64 = target;

            // skip past ": "
            i += 2;

            // ending char is a space or a newline.
            // b'\n' = 10
            // b' '  = 32
            // b'0'  = 48
            // we only expect digits in this loop
            let mut line: [u64; MAX_LINE_LEN] = [0; MAX_LINE_LEN];
            let mut digits: [usize; MAX_LINE_LEN] = [0; MAX_LINE_LEN];
            let mut l_i = 0;
            loop {
                while input[i] >= b'0' {
                    digits[l_i] += 1;
                    line[l_i] *= 10;
                    line[l_i] += (input[i] - b'0') as u64;
                    i += 1;
                }
                l_i += 1;

                if input[i] == b'\n' {
                    i += 1; // skip the newline.

                    break;
                }
                i += 1; // skip the space.
            }

            let line: [u64; MAX_LINE_LEN] = line;
            let digits: [usize; MAX_LINE_LEN] = digits;
            tasks.spawn(check2(target, line, digits));
        }

        while let Some(res) = tasks.join_next().await {
            total += res.unwrap();
        }
    });

    return total;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_example_input() -> String {
        let input_path = "input/2024/examples/day7.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_input() -> String {
        let input_path = "input/2024/day7.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), 11387);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 66343330034722)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 637696070419031)
    }
}
