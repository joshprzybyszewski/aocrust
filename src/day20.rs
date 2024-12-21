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

fn solve<const SIZE: usize, const CHEAT: usize, const SAVE: u32>(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut start: Option<Coord> = None;
    let mut goal: Option<Coord> = None;
    let mut grid: [[u32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE] =
        [[0; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE];

    let mut i: usize = 0;

    for r in 0..SIZE {
        for c in 0..SIZE {
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

    return dfs::<CHEAT, SAVE>(&mut grid, 1, start, &goal);
}

fn dfs<const CHEAT: usize, const SAVE: u32>(
    grid: &mut [[u32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE],
    pos: u32,
    current: Coord,
    goal: &Coord,
) -> u32 {
    // println!("dfs({pos}, {:?})", current);
    grid[current.row][current.col] = pos;

    if current == *goal {
        // let mut total = 0;
        // for r_i in 0..TOTAL_GRID_SIZE {
        //     for c_i in 0..TOTAL_GRID_SIZE {
        //         if grid[r_i][c_i] > 0 {
        //             total += 1;
        //         }
        //     }
        // }
        // if total != pos {
        //     print_grid(grid);
        //     unreachable!();
        // }
        return 0;
    }

    let next: Coord;
    if grid[current.row - 1][current.col] == u32::MAX {
        next = current.up();
    } else if grid[current.row + 1][current.col] == u32::MAX {
        next = current.down();
    } else if grid[current.row][current.col - 1] == u32::MAX {
        next = current.left();
    } else {
        // assume (grid[current.row][current.col + 1] == u32::MAX)
        next = current.right();
        // } else {
        //     println!("current: {:?}", current);
        //     unreachable!();
    }
    // do dfs first, then count cheats.
    let prev = dfs::<CHEAT, SAVE>(grid, pos + 1, next, goal);
    if grid[current.row][current.col] + SAVE > grid[goal.row][goal.col] {
        // no reason to look at the 100 steps closest to the goal. There are
        // no cheats greater than 100.
        return 0;
    }

    // TODO there's some mad caching we could do if we know how to inspect only the
    // diff of the changed diamond.
    if CHEAT == 2 {
        return prev + count_cheats_2(grid, current);
    }

    return prev + count_cheats::<CHEAT, SAVE>(grid, current);
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

fn print_grid(grid: &[[u32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE]) {
    for r_i in 0..TOTAL_GRID_SIZE {
        for c_i in 0..TOTAL_GRID_SIZE {
            if grid[r_i][c_i] == u32::MAX {
                unreachable!();
            }
            if r_i < BUFFER
                || c_i < BUFFER
                || r_i >= TOTAL_GRID_SIZE - BUFFER
                || c_i >= TOTAL_GRID_SIZE - BUFFER
            {
                if grid[r_i][c_i] != 0 {
                    print!("!")
                } else {
                    print!(" ")
                }
            } else {
                if grid[r_i][c_i] == 0 {
                    print!(" ")
                } else {
                    print!("{}", grid[r_i][c_i] % 10)
                }
            }
        }
        print!("\n")
    }
}

fn count_cheats<const CHEAT: usize, const SAVE: u32>(
    grid: &[[u32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE],
    current: Coord,
) -> u32 {
    let mut cheats = 0;
    let min_val = grid[current.row][current.col] + SAVE - 1;

    let mut row = current.row - CHEAT;
    let mut min_dc = current.col;
    let mut max_dc = current.col + 1;

    for _ in 0..CHEAT {
        // if current.row - row + (current.col - min_dc) != CHEAT {
        //     unreachable!()
        // }
        // if current.row - row + (max_dc - current.col) != CHEAT + 1 {
        //     unreachable!()
        // }
        for col in min_dc..max_dc {
            // if current.row - row + (current.col as i32 - col as i32).abs() as usize > CHEAT {
            //     unreachable!();
            // }
            if grid[row][col] > min_val + delta(current, row, col) {
                // println!(
                //     "from ({}, {}) to ({row}, {col}) saves {} ({} to {})",
                //     current.row,
                //     current.col,
                //     grid[row][col] - grid[current.row][current.col],
                //     grid[row][col],
                //     grid[current.row][current.col]
                // );
                cheats += 1;
            }
        }
        row += 1;
        min_dc -= 1;
        max_dc += 1;
    }

    // if min_dc != current.col - CHEAT {
    //     unreachable!()
    // }
    // if max_dc != current.col + CHEAT + 1 {
    //     unreachable!()
    // }
    for col in min_dc..max_dc {
        // if (current.col as i32 - col as i32).abs() as usize > CHEAT {
        //     unreachable!();
        // }

        if grid[row][col] > min_val + delta(current, row, col) {
            // println!(
            //     "from ({}, {}) to ({row}, {col}) saves {} ({} to {})",
            //     current.row,
            //     current.col,
            //     grid[row][col] - grid[current.row][current.col],
            //     grid[row][col],
            //     grid[current.row][current.col]
            // );
            cheats += 1;
        }
    }
    row += 1;
    min_dc += 1;
    max_dc -= 1;

    for _ in 0..CHEAT {
        // if row - current.row + (current.col - min_dc) != CHEAT {
        //     unreachable!()
        // }
        // if row - current.row + (max_dc - current.col) != CHEAT + 1 {
        //     unreachable!()
        // }
        for col in min_dc..max_dc {
            // if row - current.row + (current.col as i32 - col as i32).abs() as usize > CHEAT {
            //     unreachable!();
            // }

            if grid[row][col] > min_val + delta(current, row, col) {
                // println!(
                //     "from ({}, {}) to ({}, {}) saves {} ({} to {})",
                //     current.row - BUFFER,
                //     current.col - BUFFER,
                //     row - BUFFER,
                //     col - BUFFER,
                //     grid[row][col] - grid[current.row][current.col],
                //     grid[row][col],
                //     grid[current.row][current.col]
                // );
                cheats += 1;
            }
        }
        row += 1;
        min_dc += 1;
        max_dc -= 1;
    }

    // if min_dc != max_dc + 1 {
    //     println!("min_dc = {min_dc}");
    //     println!("max_dc = {max_dc}");
    //     unreachable!()
    // }

    return cheats;
}

fn delta(current: Coord, row: usize, col: usize) -> u32 {
    return ((row as i32 - current.row as i32).abs() + (current.col as i32 - col as i32).abs())
        as u32;
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> u32 {
    return solve::<GRID_SIZE, 2, 100>(input);
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> u32 {
    return solve::<GRID_SIZE, 20, 100>(input);
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
        assert_eq!(solve::<15, 20, 75>(get_example_input()), 3)
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
        assert_eq!(part2(&get_input()), 1014683)
    }
}
