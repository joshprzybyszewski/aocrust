#[inline(always)]
fn convert_byte(a: u8) -> u64 {
    return (a - b'0') as u64;
}

#[inline(always)]
fn left_chunk_sum(index: u64, size: u64, offset: u64) -> u64 {
    // TODO remove loop
    let mut n: u64 = 0;
    for i in 0..size {
        n += offset + i;
    }
    return index * n;
}

#[inline(always)]
fn right_chunk_sum(empty: u64, index: u64, size: u64, offset: u64) -> (u64, u64, u64, u64) {
    if size <= empty {
        let mut n: u64 = 0;
        // TODO remove loop
        for i in 0..size {
            n += offset + i;
        }
        return (index * n, empty - size, size, 0);
    }

    let mut n: u64 = 0;
    // TODO remove loop
    for i in 0..empty {
        n += offset + i;
    }
    return (index * n, 0, empty, size - empty);
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    // 20,000 blocks
    let input = input.as_bytes();

    let mut passed: u64 = 0;
    let mut l: u64 = 0;
    let mut i: usize = 0;
    let mut j: usize = input.len() - 1;
    if input[j] == b'\n' {
        // the last char is a newline.
        j -= 1;
    }
    if j % 2 != 0 {
        // it must be an odd number
        unreachable!();
    }
    let mut r: u64 = (j as u64 / 2) + 1; // should start at 10,001

    let mut total: u64 = 0;
    let mut right_chunk: u64 = 0;
    let mut right_total;

    while i <= j {
        let left_chunk = convert_byte(input[i]);
        i += 1;

        let left_total = left_chunk_sum(l, left_chunk, passed);
        total += left_total;

        l += 1;
        passed += left_chunk;
        if i >= j {
            break;
        }

        let mut empty = convert_byte(input[i]);
        i += 1;
        if right_chunk == 0 {
            // get a block from the end
            right_chunk = convert_byte(input[j]);
            j -= 2; // skip the space to the next chunk
            r -= 1;
        }

        let mut right_passed;
        (right_total, empty, right_passed, right_chunk) =
            right_chunk_sum(empty, r, right_chunk, passed);
        total += right_total;
        passed += right_passed;

        while empty > 0 && right_chunk == 0 && j >= i {
            // get a block from the end
            right_chunk = convert_byte(input[j]);
            j -= 2;
            r -= 1;

            (right_total, empty, right_passed, right_chunk) =
                right_chunk_sum(empty, r, right_chunk, passed);
            total += right_total;
            passed += right_passed;
        }
    }

    (right_total, _, _, _) = right_chunk_sum(9, r, right_chunk, passed);
    total += right_total;

    return total;
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_example_input() -> String {
        let input_path = "input/2024/examples/day9.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_input() -> String {
        let input_path = "input/2024/day9.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn easy() {
        //                0 1 2 3 4
        assert_eq!(part1("191010101\n"), 0 + 4 * 1 + 3 * 2 + 2 * 3 + 1 * 4);
    }

    #[test]
    fn medium() {
        // 00..11..22..3333333
        // 0033113322333......
        //                0 1 2 3
        assert_eq!(
            part1("2222227\n"),
            0 + 3 * 2
                + 3 * 3
                + 1 * 4
                + 1 * 5
                + 3 * 6
                + 3 * 7
                + 2 * 8
                + 2 * 9
                + 3 * 10
                + 3 * 11
                + 3 * 12
        );
    }

    #[test]
    fn very_easy() {
        //                0 1
        assert_eq!(part1("191\n"), 0 + 1 * 1);
        //                0 1 2
        assert_eq!(part1("19102\n"), 0 + 2 * 1 + 2 * 2 + 1 * 3);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), 2858);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 6332189866718)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 0)
    }
}
