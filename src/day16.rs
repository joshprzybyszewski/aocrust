#[aoc(day16, part1)]
pub fn part1(input: &str) -> i32 {
    return 0;
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day16.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 0);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 0);
    }
}
