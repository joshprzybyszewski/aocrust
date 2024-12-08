const GRID_SIZE: usize = 50;
// const GRID_SIZE: usize = 12;
const GRID_SIZE_i32: i32 = GRID_SIZE as i32;
const ANTENNA_CARDINALITY: usize = 10 + 26 + 26;
const MAX_ANTENNA_OF_TYPE: usize = 100;

#[inline(always)]
fn convert_byte(val: u8) -> u8 {
    if val < b'A' {
        return val - b'0';
    }
    if val < b'a' {
        return val - b'A' + 10;
    }
    return val - b'a' + 36;
}

#[derive(Copy, Clone)]
struct Coord {
    r: i32,
    c: i32,
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut i: usize = 0;
    let mut antennas: [Vec<Coord>; ANTENNA_CARDINALITY] = [
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
        Vec::with_capacity(MAX_ANTENNA_OF_TYPE),
    ];

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            if input[i] == b'.' {
                i += 1;
                continue;
            }

            let a = convert_byte(input[i]);
            i += 1;

            // input[i] must be an antenna
            antennas[a as usize].push(Coord {
                r: r as i32,
                c: c as i32,
            });
        }
        i += 1; // input[i] is a newline
    }

    let mut grid: [[bool; GRID_SIZE]; GRID_SIZE] = [[false; GRID_SIZE]; GRID_SIZE];
    let mut total = 0;

    for group in antennas {
        for i in 0..group.len() {
            for j in i + 1..group.len() {
                let a = group[i];
                let b = group[j];
                // we know a and be are either in the same row, OR b is below a.
                let dr = (a.r - b.r).abs();
                let dc = (a.c - b.c).abs();

                if a.c < b.c {
                    // an1 is left of a (could be above or the same row)
                    if a.r - dr >= 0 && a.c - dc >= 0 {
                        if !grid[(a.r - dr) as usize][(a.c - dc) as usize] {
                            total += 1;
                            grid[(a.r - dr) as usize][(a.c - dc) as usize] = true;
                        }
                    }
                    // an2 is right of b (could be below or the same row)
                    if b.r + dr < GRID_SIZE_i32 && b.c + dc < GRID_SIZE_i32 {
                        if !grid[(b.r + dr) as usize][(b.c + dc) as usize] {
                            total += 1;
                            grid[(b.r + dr) as usize][(b.c + dc) as usize] = true;
                        }
                    }
                } else if a.c == b.c {
                    // same column; they are above and below.
                    if a.r - dr >= 0 {
                        if !grid[(a.r - dr) as usize][a.c as usize] {
                            total += 1;
                            grid[(a.r - dr) as usize][a.c as usize] = true;
                        }
                    }
                    // an2 is right of b (could be below or the same row)
                    if b.r + dr < GRID_SIZE_i32 {
                        if !grid[(b.r + dr) as usize][b.c as usize] {
                            total += 1;
                            grid[(b.r + dr) as usize][b.c as usize] = true;
                        }
                    }
                } else {
                    // a MUST be above b.
                    // an1 is above and right of a
                    if a.r - dr >= 0 && a.c + dc < GRID_SIZE_i32 {
                        if !grid[(a.r - dr) as usize][(a.c + dc) as usize] {
                            total += 1;
                            grid[(a.r - dr) as usize][(a.c + dc) as usize] = true
                        }
                    }
                    // an2 is below and left of b
                    if b.r + dr < GRID_SIZE_i32 && b.c - dc >= 0 {
                        if !grid[(b.r + dr) as usize][(b.c - dc) as usize] {
                            total += 1;
                            grid[(b.r + dr) as usize][(b.c - dc) as usize] = true;
                        }
                    }
                }
            }
        }
    }

    // 291 is wrong
    return total;
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u64 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_example_input() -> String {
        let input_path = "input/2024/examples/day8.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_input() -> String {
        let input_path = "input/2024/day8.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn my_byte_conversion() {
        assert_eq!(convert_byte(b'0'), 0);
        assert_eq!(convert_byte(b'9'), 9);
        assert_eq!(convert_byte(b'A'), 10);
        assert_eq!(convert_byte(b'Z'), 35);
        assert_eq!(convert_byte(b'a'), 36);
        assert_eq!(convert_byte(b'z'), 61);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 14);
    }

    #[test]
    fn bounds_checking() {
        assert_eq!(
            part1(
                ".h.h.....igi
.b........g.
............
...b........
..........k.
.j........j.
.k....l...ff
.....l......
ee..........
............
...d......c.
...d.....c..
"
            ),
            11
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), 420);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 261)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 0)
    }
}
