use std::collections::HashMap;

const MAX_ITERATION: usize = 75;

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
struct Input {
    val: u64,
    num_blinks: u32,
}

struct StoneChanger {
    cache: HashMap<Input, usize>,
}

impl StoneChanger {
    fn iterate(&self, input: Input) -> usize {
        if input.num_blinks == 0 {
            return 1;
        }
        if input.val == 0 {
            return self.iterate(Input {
                val: 1,
                num_blinks: input.num_blinks - 1,
            });
        }
        let s: String = input.val.to_string();
        if s.len() % 2 == 0 {
            let tens = 10u64.pow(s.len() as u32 / 2);
            let left = input.val / tens;
            let right = input.val % tens;
            return self.iterate(Input {
                val: left,
                num_blinks: input.num_blinks - 1,
            }) + self.iterate(Input {
                val: right,
                num_blinks: input.num_blinks - 1,
            });
        }

        return self.iterate(Input {
            val: input.val * 2024,
            num_blinks: input.num_blinks - 1,
        });
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    return get_stones_after_blinks(input, 25);
}

fn get_stones_after_blinks(input: &str, num_blinks: u32) -> usize {
    let mut stones = get_stones(input);
    stones.sort();

    let changer: StoneChanger = StoneChanger {
        cache: HashMap::with_capacity(256),
    };
    let mut sum = 0;
    for stone in stones {
        sum += changer.iterate(Input {
            val: stone,
            num_blinks: num_blinks,
        });
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
    fn part2_example() {
        assert_eq!(part2(""), 0);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 183435)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 0)
    }
}
