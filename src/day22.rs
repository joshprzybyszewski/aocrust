use std::collections::HashMap;

// decimal: 16777216
// hex:     0x1000000
// 25 binary digits = 0b1000000000000000000000000
const MODULO_MASK: i32 = 0xFFFFFF;
const NUM_ITERATIONS: usize = 2000;
const DIFF_SPACE: u32 = 19; // -9 through 9
const DIFF_SPACE_CUBED_U32: u32 = DIFF_SPACE * DIFF_SPACE * DIFF_SPACE;
const DIFF_SPACE_CUBED: usize = DIFF_SPACE_CUBED_U32 as usize;
const DIFF_SPACE_QUAD: usize = DIFF_SPACE_CUBED * (DIFF_SPACE as usize);

#[inline(always)]
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

#[inline(always)]
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
            if i >= input.len() {
                break;
            }
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

// pair is the value, and the diff.
#[inline(always)]
fn consider_part2(
    secret: i32,
    totals: &mut [u32; DIFF_SPACE_QUAD],
    has_seen: &mut [u32; DIFF_SPACE_CUBED],
) -> u32 {
    let mut val = secret;
    let mut i = 0;
    let mut last_3_diffs: u32 = 0;
    let mut best: u32 = 0;
    let mut prev_ones = val % 10;
    loop {
        if i == 3 {
            // the first three don't add to the hash map.
            break;
        }
        let next = generate(val);
        let ones = next % 10;
        let diff = ((9 + ones) - prev_ones) as u32;
        last_3_diffs *= DIFF_SPACE;
        last_3_diffs += diff;
        prev_ones = ones;
        val = next;
        i += 1;
    }
    if last_3_diffs > DIFF_SPACE_CUBED_U32 {
        unreachable!();
    }

    loop {
        if i == NUM_ITERATIONS {
            break;
        }
        let next = generate(val);
        let ones = next % 10;
        let diff = ((9 + ones) - prev_ones) as u32;

        //
        if has_seen[last_3_diffs as usize] & (1 << diff) == 0 {
            has_seen[last_3_diffs as usize] |= 1 << diff;

            last_3_diffs *= DIFF_SPACE;
            last_3_diffs += diff;
            // use the last four diffs to group.
            totals[last_3_diffs as usize] += ones as u32;
            if totals[last_3_diffs as usize] > best {
                best = totals[last_3_diffs as usize];
            }
        } else {
            last_3_diffs *= DIFF_SPACE;
            last_3_diffs += diff;
        }

        last_3_diffs %= DIFF_SPACE_CUBED_U32;
        prev_ones = ones;
        val = next;
        i += 1;
    }

    return best;
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut sums: [u32; DIFF_SPACE_QUAD] = [0; DIFF_SPACE_QUAD];
    let mut has_seen: [u32; DIFF_SPACE_CUBED];
    let mut best: u32 = 0;
    let mut i: usize = 0;
    let mut val = 0;
    loop {
        if input[i] == b'\n' {
            has_seen = [0; DIFF_SPACE_CUBED];
            let my_best = consider_part2(val, &mut sums, &mut has_seen);
            if my_best > best {
                best = my_best;
            }
            i += 1;
            if i >= input.len() {
                break;
            }
            val = 0;
        }
        val *= 10;
        val += (input[i] - b'0') as i32;
        i += 1;
        if i >= input.len() {
            has_seen = [0; DIFF_SPACE_CUBED];
            let my_best = consider_part2(val, &mut sums, &mut has_seen);
            if my_best > best {
                best = my_best;
            }

            break;
        }
    }
    return best;
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
        assert_eq!(generate_times(1), 8685429);
        assert_eq!(generate_times(10), 4700978);
        assert_eq!(generate_times(100), 15273692);
        assert_eq!(generate_times(2024), 8667524);
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
        assert_eq!(part2(&get_input()), 1896)
    }
}
