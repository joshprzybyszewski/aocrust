#[aoc(day25, part1)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut i = 0;

    let mut locks: [u32; 1024] = [0; 1024];
    let mut lock_index = 0;
    let mut keys: [u32; 1024] = [0; 1024];
    let mut key_index = 0;

    loop {
        if i >= input.len() || input[i] == b'\n' {
            break;
        }

        let is_lock = input[i] == b'#';
        i += 6;
        let mut my_val = 0u32;
        let mut offset = 0;
        for _ in 0..5 {
            for _ in 0..5 {
                if input[i] == b'#' {
                    my_val |= 1 << offset;
                }
                offset += 1;
                i += 1;
            }
            i += 1;
        }
        i += 7;

        if is_lock {
            locks[lock_index] = my_val;
            lock_index += 1;
        } else {
            keys[key_index] = my_val;
            key_index += 1;
        }
    }

    let mut total = 0;

    for k in 0..key_index {
        for l in 0..lock_index {
            if keys[k] & locks[l] == 0 {
                total += 1;
            }
        }
    }

    return total;
}

#[aoc(day25, part2)]
pub fn part2(_: &str) -> u32 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day25.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_example_input() -> &'static str {
        return "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
    }

    fn get_example_input_2() -> &'static str {
        return "";
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 3);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 3301)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input_2()), 1);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 1);
    }
}
