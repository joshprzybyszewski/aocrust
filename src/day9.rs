#[inline(always)]
fn convert_byte(a: u8) -> u64 {
    return (a - b'0') as u64;
}

const MAGIC_NUM: [u64; 10] = [
    0,
    0,
    1,
    1 + 2,
    1 + 2 + 3,
    1 + 2 + 3 + 4,
    1 + 2 + 3 + 4 + 5,
    1 + 2 + 3 + 4 + 5 + 6,
    1 + 2 + 3 + 4 + 5 + 6 + 7,
    1 + 2 + 3 + 4 + 5 + 6 + 7 + 8,
];

#[inline(always)]
fn contiguous_chunk_sum(index: u64, size: u64, offset: u64) -> u64 {
    let n = size * offset + MAGIC_NUM[size as usize];
    return index * n;
}

#[inline(always)]
fn right_chunk_sum(empty: u64, index: u64, size: u64, offset: u64) -> (u64, u64, u64, u64) {
    if size <= empty {
        return (
            contiguous_chunk_sum(index, size, offset),
            empty - size,
            size,
            0,
        );
    }

    return (
        contiguous_chunk_sum(index, empty, offset),
        0,
        empty,
        size - empty,
    );
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

#[inline(always)]
fn set_best_spaces(c_i: usize, spaces: &[Chunk; 10_000]) -> [usize; 10] {
    let mut best_spaces: [usize; 10] = [10_000; 10];
    let mut max_size = 0;
    for s_i in 0..c_i {
        if best_spaces[spaces[s_i].size as usize] < s_i {
            // already set.
            continue;
        }
        if spaces[s_i].size > max_size {
            max_size = spaces[s_i].size
        }
        for size in (0..=spaces[s_i].size as usize).rev() {
            if best_spaces[size] < s_i {
                break;
            }
            best_spaces[size] = s_i;
        }
        if max_size >= 9 {
            // completely set all sizes
            return best_spaces;
        }
    }

    return best_spaces;
}

#[inline(always)]
fn reset_best_spaces(
    c_i: usize,
    spaces: &[Chunk; 10_000],
    best_spaces: &mut [usize; 10],
    prev_s_i: usize,
    prev_space_size: u64,
) {
    for s_i in 0..best_spaces.len() {
        if best_spaces[s_i] == prev_s_i {
            best_spaces[s_i] = 10_001;
        }
    }

    let mut max_size = 0;
    for s_i in 0..c_i {
        if best_spaces[spaces[s_i].size as usize] < s_i {
            // already set.
            continue;
        }
        if spaces[s_i].size > max_size {
            max_size = spaces[s_i].size
        }
        for size in (0..=spaces[s_i].size as usize).rev() {
            if best_spaces[size] < s_i {
                break;
            }
            best_spaces[size] = s_i;
        }
        if max_size >= 9 {
            // completely set all sizes
            return;
        }
    }
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let max_i = input.len() - 2;

    let mut chunks: [Chunk; 10_000] = [Chunk { size: 0, offset: 0 }; 10_000];
    let mut spaces: [Chunk; 10_000] = [Chunk { size: 0, offset: 0 }; 10_000];
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

    let mut best_spaces = set_best_spaces(c_i, &spaces);

    let mut total = 0;
    for i in (1..=c_i).rev() {
        let s_i = best_spaces[chunks[i].size as usize];
        if s_i < c_i && spaces[s_i].offset < chunks[i].offset {
            if spaces[s_i].size < chunks[i].size {
                unreachable!();
            }

            let prev_space_size = spaces[s_i].size;
            chunks[i].offset = spaces[s_i].offset;

            spaces[s_i].size -= chunks[i].size;
            spaces[s_i].offset += chunks[i].size;

            // best_spaces = set_best_spaces(c_i, &spaces);
            reset_best_spaces(c_i, &spaces, &mut best_spaces, s_i, prev_space_size);
        }

        total += contiguous_chunk_sum(i as u64, chunks[i].size, chunks[i].offset);
    }

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
        assert_eq!(part2(&get_input()), 6353648390778)
    }
}
