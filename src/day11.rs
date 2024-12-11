use std::collections::HashMap;

// u64 only supports 20 digit numbers.
const TEN_POWERS: [u64; 20] = [
    1,
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
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
];

const MAX_ITERATION: usize = 76;

fn get_stones(input: &str) -> Vec<u64> {
    let mut stones: Vec<u64> = Vec::with_capacity(16);
    let mut val: u64 = 0;

    input.bytes().into_iter().for_each(|c| match c {
        b'0'..=b'9' => {
            val *= 10;
            val += (c - b'0') as u64;
        }
        b' ' => {
            stones.push(val);
            val = 0;
        }
        b'\n' => {}
        _ => unreachable!(),
    });
    stones.push(val);
    return stones;
}

struct StoneChanger {
    cache: HashMap<u64, [usize; MAX_ITERATION]>,
}

impl StoneChanger {
    fn iterate(&mut self, val: u64, num_blinks: usize) -> usize {
        if !self.cache.contains_key(&val) {
            let mut progress: [usize; MAX_ITERATION] = [0; MAX_ITERATION];
            self.populate(val, &mut progress, num_blinks);
            self.cache.insert(val, progress);
            return progress[num_blinks];
        }
        let progress: &[usize; MAX_ITERATION] = self.cache.get(&val).unwrap();
        if progress[num_blinks] == 0 {
            // it wasn't populated far enough out.
            let mut progress: [usize; MAX_ITERATION] = [0; MAX_ITERATION];
            self.populate(val, &mut progress, num_blinks);
            self.cache.insert(val, progress);
            return progress[num_blinks];
        }
        return progress[num_blinks];
    }

    fn populate(&mut self, initial: u64, progress: &mut [usize; MAX_ITERATION], needs: usize) {
        progress[0] = 1;
        for i in 1..=needs {
            if progress[i] != 0 {
                continue;
            }
            progress[i] = self.get(initial, i);
        }
    }

    fn get(&mut self, val: u64, num_blinks: usize) -> usize {
        if num_blinks == 0 {
            return 1;
        }
        if val == 0 {
            return self.iterate(1, num_blinks - 1);
        }

        let mut ten_i = 0;
        loop {
            if val < TEN_POWERS[ten_i + 1] && val >= TEN_POWERS[ten_i] {
                // between
                // [   1 ->   10 )
                // [ 100 -> 1000 )
                // ...
                return self.get(val * 2024, num_blinks - 1);
            }
            ten_i += 1;
            if val < TEN_POWERS[ten_i + 1] && val >= TEN_POWERS[ten_i] {
                // between:
                // [   10 ->   100 )
                // [ 1000 -> 10000 )
                // ...
                break;
            }
            ten_i += 1;

            if ten_i >= TEN_POWERS.len() {
                unreachable!();
            }
        }

        let div = TEN_POWERS[(ten_i + 1) / 2];
        let left = val / div;
        let right = val % div;
        let left_answer = self.iterate(left, num_blinks - 1);
        let right_answer = self.iterate(right, num_blinks - 1);
        return left_answer + right_answer;
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    return get_stones_after_blinks(input, 25);
}

fn get_stones_after_blinks(input: &str, num_blinks: usize) -> usize {
    let mut stones = get_stones(input);
    stones.sort();

    let mut changer: StoneChanger = StoneChanger {
        cache: HashMap::with_capacity(256),
    };
    let mut sum = 0;
    for stone in stones {
        sum += changer.iterate(stone, num_blinks);
    }
    return sum;
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    return get_stones_after_blinks(input, 75);
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day11.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(get_stones_after_blinks("0 1 10 99 999", 1), 7);
        assert_eq!(get_stones_after_blinks("125 17", 6), 22);
        assert_eq!(get_stones_after_blinks("125 17", 25), 55312);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 183435)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 218279375708592)
    }
}
