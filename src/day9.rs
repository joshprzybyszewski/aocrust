#[inline(always)]
fn convert_byte(a: u8) -> u64 {
    return (a - b'0') as u64;
}

#[inline(always)]
fn contiguous_chunk_sum(index: u64, size: u64, offset: u64) -> u64 {
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
        // TODO remove
        unreachable!();
    }
    let mut r: u64 = (j as u64 / 2) + 1; // should start at 10,001

    let mut total: u64 = 0;
    let mut right_chunk: u64 = 0;
    let mut right_total;

    while i <= j {
        let left_chunk = convert_byte(input[i]);
        i += 1;

        let left_total = contiguous_chunk_sum(l, left_chunk, passed);
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

#[derive(Copy, Clone)]
struct Chunk {
    size: u64,
    offset: u64,
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let max_i = input.len() - 2;

    let mut chunks: [Chunk; 9_999] = [Chunk { size: 0, offset: 0 }; 9_999];
    let mut spaces: [Chunk; 9_998] = [Chunk { size: 0, offset: 0 }; 9_998];
    let mut i: usize = 0;
    let mut c_i: usize = 0;
    chunks[c_i].size = convert_byte(input[i]);
    i += 1;
    while i < max_i {
        spaces[c_i].offset = chunks[c_i].offset + chunks[c_i].size;
        spaces[c_i].size = convert_byte(input[i]);
        i += 1;
        c_i += 1;

        chunks[c_i].offset = spaces[c_i - 1].offset + spaces[c_i - 1].size;
        chunks[c_i].size = convert_byte(input[i]);
        i += 1;
    }
    let c_i = c_i;

    let mut total = 0;
    for i in (1..=c_i).rev() {
        // if chunks[i].offset == 0 {
        //     unreachable!();
        // }
        for j in 0..=i {
            if spaces[j].size < chunks[i].size {
                continue;
            }
            // if spaces[j].offset >= chunks[i].offset || spaces[j].offset == 0 {
            //     unreachable!();
            // }
            chunks[i].offset = spaces[j].offset;
            spaces[j].size -= chunks[i].size;
            spaces[j].offset += chunks[i].size;
            break;
        }

        total += contiguous_chunk_sum(i as u64, chunks[i].size, chunks[i].offset);
    }

    // 8507923463167 is too high.
    // 6350606246106 is too low.

    return total;
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
    fn part2_contrived() {
        // 0...112.3344.55
        // 055211..3344
        //                0 1 2 3 4 5
        assert_eq!(
            part2("13201120212\n"),
            0 + 5 * 1 + 5 * 2 + 2 * 3 + 1 * 4 + 1 * 5 + 3 * 8 + 3 * 9 + 4 * 10 + 4 * 11
        );
        // 0.........1.22.333.4444.5
        // 0544443331..22
        assert_eq!(
            part2("19112131411\n"),
            0 + 5 * 1
                + 4 * 2
                + 4 * 3
                + 4 * 4
                + 4 * 5
                + 3 * 6
                + 3 * 7
                + 3 * 8
                + 1 * 9
                + 0 * 10
                + 0 * 11
                + 2 * 12
                + 2 * 13
        );
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
