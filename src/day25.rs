#[aoc(day25, part1)]
pub fn part1(input: &str) -> u32 {
    return 0;
}

#[aoc(day25, part2)]
pub fn part2(input: &str) -> u32 {
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
        assert_eq!(part1(&get_input()), 1)
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
