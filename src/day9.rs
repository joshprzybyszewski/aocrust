#[inline(always)]
fn convert_byte(a: u8) -> u64 {
    return (a - b'0') as u64;
}

#[inline(always)]
fn chunk_sum(index: u64, size: u64, offset: u64) -> u64 {
    // 8 * (8 + 9 + 10)
    // 8 * 27
    // 27 = (offset = 8) (size = 3)
    let mut n: u64 = 0;
    for i in 0..offset {
        n += offset + i;
    }
    return index * n;
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    // 20,000 blocks
    let input = input.as_bytes();

    let mut passed: u64 = 0;
    let mut l: u64 = 0;
    let mut i: usize = 0;
    let mut j: usize = input.len() - 1; // the last char is a newline.
    if input[j] == b'\n' {
        j -= 1;
    }
    if j % 2 != 1 {
        // it must be an even number
        unreachable!();
    }
    let mut r: u64 = (j as u64 / 2) + 1; // should start at 10,001

    let mut total: u64 = 0;
    let mut right_chunk: u64 = 0;

    while i < j {
        let left_chunk = convert_byte(input[i]);
        i += 1;

        total += chunk_sum(l, left_chunk, passed);

        l += 1;
        passed += left_chunk;
        if i >= j {
            break;
        }

        let mut empty = convert_byte(input[i]);
        i += 1;

        // fill the empty block with entries from the end
        if right_chunk > 0 {
            let n = if right_chunk < empty {
                right_chunk
            } else {
                empty
            };

            total += chunk_sum(r, n, passed);
            passed += n;
            empty -= n;
        }

        while empty > 0 && j > i {
            // get a block from the end
            j -= 1; // skip past the empty space
            right_chunk = convert_byte(input[j]);
            j -= 1;
            r -= 1;

            let n = if right_chunk < empty {
                right_chunk
            } else {
                empty
            };

            total += chunk_sum(r, n, passed);
            passed += n;
            empty -= n;
        }
    }

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
        assert_eq!(part1("1910101010\n"), 0 + 4 * 1 + 3 * 2 + 2 * 3 + 1 * 4);
    }

    #[test]
    fn very_easy() {
        //                0 1
        assert_eq!(part1("1910\n"), 0 + 1 * 1);
        //                0 1 2
        assert_eq!(part1("191020\n"), 0 + 2 * 1 + 2 * 2 + 1 * 3);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), 0);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 0)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 0)
    }
}
