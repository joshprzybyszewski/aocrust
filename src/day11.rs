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

const MAX_FUTURE_CACHE: usize = 1500;
const MAX_FUTURE_CACHE_U64: u64 = MAX_FUTURE_CACHE as u64;
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

#[derive(Copy, Clone)]
struct NextSplit {
    num_blinks: usize,
    left: u64,
    right: u64,
}

impl NextSplit {
    fn new(num_blinks: usize, left: u64, right: u64) -> Self {
        NextSplit {
            num_blinks: num_blinks,
            left: left,
            right: right,
        }
    }

    fn one_farther(&self) -> NextSplit {
        NextSplit {
            num_blinks: self.num_blinks + 1,
            left: self.left,
            right: self.right,
        }
    }
}

struct StoneChanger {
    cache: HashMap<u64, NextSplit>,

    future: [[usize; MAX_ITERATION]; MAX_FUTURE_CACHE],
}

impl StoneChanger {
    fn new() -> Self {
        StoneChanger {
            cache: HashMap::with_capacity(256),
            future: [[0; MAX_ITERATION]; MAX_FUTURE_CACHE],
        }
    }

    fn get_stones_after_blinks(&mut self, val: u64, num_blinks: usize) -> usize {
        return self.iterate(val, num_blinks);
    }

    fn iterate(&mut self, val: u64, remaining: usize) -> usize {
        if val < MAX_FUTURE_CACHE_U64 && self.future[val as usize][remaining] != 0 {
            return self.future[val as usize][remaining];
        }

        if remaining == 0 {
            if val < MAX_FUTURE_CACHE_U64 {
                self.future[val as usize][remaining] = 1;
            }
            return 1;
        }

        let split = self.get_next_split(val);
        if val < MAX_FUTURE_CACHE_U64 {
            for i in 0..split.num_blinks {
                self.future[val as usize][i] = 1;
            }
        }
        if split.num_blinks > remaining {
            return 1;
        }

        let steps = remaining - split.num_blinks;
        let answer = self.iterate(split.left, steps) + self.iterate(split.right, steps);
        if val < MAX_FUTURE_CACHE_U64 {
            self.future[val as usize][remaining] = answer;
        }
        return answer;
    }

    fn get_next_split(&mut self, val: u64) -> NextSplit {
        if self.cache.contains_key(&val) {
            return *self.cache.get(&val).unwrap();
        }

        if val == 0 {
            let next = self.get_next_split(1);
            let mine = next.one_farther();
            self.cache.insert(val, mine);
            return mine;
        }

        let mut ten_i = 1;
        loop {
            if val < TEN_POWERS[ten_i] {
                // between
                // [   1 ->   10 )
                // [ 100 -> 1000 )
                // ...
                let next = self.get_next_split(val * 2024);
                let mine = next.one_farther();
                self.cache.insert(val, mine);
                return mine;
            }
            ten_i += 1;
            if val < TEN_POWERS[ten_i] {
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

        let div = TEN_POWERS[ten_i / 2];
        let left = val / div;
        let right = val % div;

        let mine = NextSplit::new(1, left, right);
        self.cache.insert(val, mine);
        return mine;
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    return get_stones_after_blinks(input, 25);
}

fn get_stones_after_blinks(input: &str, num_blinks: usize) -> usize {
    let mut stones = get_stones(input);
    stones.sort();

    let mut changer: StoneChanger = StoneChanger::new();
    let mut sum = 0;
    for stone in stones {
        sum += changer.get_stones_after_blinks(stone, num_blinks);
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
