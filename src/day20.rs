const GRID_SIZE: usize = 141;
const BUFFER: usize = 20;
const TOTAL_GRID_SIZE: usize = BUFFER + GRID_SIZE + BUFFER;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(r: usize, c: usize) -> Self {
        return Coord { row: r, col: c };
    }

    #[inline(always)]
    fn up(&self) -> Coord {
        return Coord {
            row: self.row - 1,
            col: self.col,
        };
    }

    #[inline(always)]
    fn right(&self) -> Coord {
        return Coord {
            row: self.row,
            col: self.col + 1,
        };
    }

    #[inline(always)]
    fn down(&self) -> Coord {
        return Coord {
            row: self.row + 1,
            col: self.col,
        };
    }

    #[inline(always)]
    fn left(&self) -> Coord {
        return Coord {
            row: self.row,
            col: self.col - 1,
        };
    }
}

fn solve<const PART1: bool>(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut start: Option<Coord> = None;
    let mut goal: Option<Coord> = None;
    let mut grid: [[u32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE] =
        [[0; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE];

    let mut i: usize = 0;

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            match input[i] {
                b'#' => {}
                b'.' => {
                    grid[r + BUFFER][c + BUFFER] = u32::MAX;
                }
                b'S' => {
                    grid[r + BUFFER][c + BUFFER] = u32::MAX;
                    start = Some(Coord::new(r + BUFFER, c + BUFFER));
                }
                b'E' => {
                    grid[r + BUFFER][c + BUFFER] = u32::MAX;
                    goal = Some(Coord::new(r + BUFFER, c + BUFFER));
                }
                _ => unreachable!(),
            }
            i += 1;
        }
        i += 1; // input[i] is a newline
    }
    let start = start.unwrap();
    let goal = goal.unwrap();

    return dfs::<PART1>(&mut grid, 1, start, &goal);
}

fn dfs<const PART1: bool>(
    grid: &mut [[u32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE],
    pos: u32,
    current: Coord,
    goal: &Coord,
) -> u32 {
    // println!("dfs({pos}, {:?})", current);
    grid[current.row][current.col] = pos;

    if current == *goal {
        return 0;
    }

    let next: Coord;
    if grid[current.row - 1][current.col] == u32::MAX {
        next = current.up();
    } else if grid[current.row + 1][current.col] == u32::MAX {
        next = current.down();
    } else if grid[current.row][current.col - 1] == u32::MAX {
        next = current.left();
    } else if grid[current.row][current.col + 1] == u32::MAX {
        next = current.right();
    } else {
        println!("current: {:?}", current);
        unreachable!();
    }
    // do dfs first, then count cheats.
    let prev = dfs::<PART1>(grid, pos + 1, next, goal);
    if grid[current.row][current.col] + 100 > grid[goal.row][goal.col] {
        // no reason to look at the 100 steps closest to the goal. There are
        // no cheats greater than 100.
        return 0;
    }

    if PART1 {
        return prev + count_cheats_2(grid, current);
    }

    return prev + count_cheats_20(grid, current);
}

fn count_cheats_2(grid: &[[u32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE], current: Coord) -> u32 {
    let mut cheats = 0;
    let min_val = grid[current.row][current.col] + 100;
    if grid[current.row - 2][current.col] > min_val {
        cheats += 1;
    }
    if grid[current.row + 2][current.col] > min_val {
        cheats += 1;
    }
    if grid[current.row][current.col - 2] > min_val {
        cheats += 1;
    }
    if grid[current.row][current.col + 2] > min_val {
        cheats += 1;
    }
    return cheats;
}

fn count_cheats_20(grid: &[[u32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE], current: Coord) -> u32 {
    let mut cheats = 0;
    let min_val = grid[current.row][current.col] + 100;

    // in (0, 0) -> buf (20, 20)
    // check:
    //  buf                     (0, 20)
    //  buf           (1, 19),  (1, 20),  (1, 21)
    //  buf (2, 18),  (2, 19),  (2, 20),  (2, 21), (2, 22)
    //  buf ...
    //  buf          (39, 19), (39, 20), (39, 21)
    //  buf                    (40, 20)
    //
    let mut row = current.row - 20;
    let mut min_dc = current.col;
    let mut max_dc = current.col + 1;
    for _ in 0..20 {
        for col in min_dc..max_dc {
            if current.row - row + ((current.col - col) as i32).abs() as usize > 20 {
                unreachable!();
            }
            if grid[row][col] > min_val {
                cheats += 1;
            }
        }
        row += 1;
        min_dc -= 1;
        max_dc += 1;
    }

    for col in min_dc..max_dc {
        if ((current.col - col) as i32).abs() as usize > 20 {
            unreachable!();
        }

        if grid[row][col] > min_val {
            cheats += 1;
        }
    }
    row += 1;
    min_dc += 1;
    max_dc -= 1;

    for _ in 0..20 {
        for col in min_dc..max_dc {
            if row - current.row + ((current.col - col) as i32).abs() as usize > 20 {
                unreachable!();
            }

            if grid[row][col] > min_val {
                cheats += 1;
            }
        }
        row += 1;
        min_dc += 1;
        max_dc -= 1;
    }

    return cheats;
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> u32 {
    return solve::<true>(input);
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> u32 {
    return solve::<false>(input);
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day20.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_example_input() -> &'static str {
        return "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(get_example_input()), 0)
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 1417)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(get_example_input()), 0)
    }

    #[test]
    fn part2_real_input() {
        // 1058211 is not right.
        // 1059628 is too high
        // 1062395 is too high.
        // 1122816
        assert_eq!(part2(&get_input()), 1)
    }
}
