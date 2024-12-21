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
    }

    // do dfs first, then count cheats.
    let prev = dfs::<CHEAT, SAVE>(grid, pos + 1, next, goal);
    if grid[current.row][current.col] + SAVE > grid[goal.row][goal.col] {
        // no reason to look at the steps closest to the goal. There will be
        // no cheats that beat the distance.
        return 0;
    }

    // TODO there's some mad caching we could do if we know how to inspect only the
    // diff of the changed diamond.
    if CHEAT == 2 {
        return prev + count_cheats_2::<SAVE>(grid, current);
    }

    return prev + count_cheats::<CHEAT, SAVE>(grid, current);
}

fn count_cheats_2<const SAVE: u32>(
    grid: &[[u32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE],
    current: Coord,
) -> u32 {
    let mut cheats = 0;
    let min_val = grid[current.row][current.col] + SAVE;
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

fn count_cheats<const CHEAT: usize, const SAVE: u32>(
    grid: &[[u32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE],
    current: Coord,
) -> u32 {
    let mut cheats = 0;

    let mut row = current.row - CHEAT;
    let mut min_dc = current.col;
    let mut max_dc = current.col + 1;

    let min_val = grid[current.row][current.col] + SAVE - 1;

    for _ in 0..CHEAT {
        for col in min_dc..0 {
            if grid[row][col] > min_val + delta(current, row, col) {
                cheats += 1;
            }
        }

        for col in 0..max_dc {
            if grid[row][col] > min_val + delta(current, row, col) {
                cheats += 1;
            }
        }
        row += 1;
        min_dc -= 1;
        max_dc += 1;
    }

    for col in min_dc..max_dc {
        if grid[row][col] > min_val + delta(current, row, col) {
            cheats += 1;
        }
    }
    row += 1;
    min_dc += 1;
    max_dc -= 1;

    for _ in 0..CHEAT {
        for col in min_dc..max_dc {
            if grid[row][col] > min_val + delta(current, row, col) {
                cheats += 1;
            }
        }
        row += 1;
        min_dc += 1;
        max_dc -= 1;
    }

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
        assert_eq!(solve::<15, 2, 64>(get_example_input()), 1);
        assert_eq!(solve::<15, 2, 40>(get_example_input()), 1 + 1);
        assert_eq!(solve::<15, 2, 38>(get_example_input()), 1 + 1 + 1);
        assert_eq!(solve::<15, 2, 36>(get_example_input()), 1 + 1 + 1 + 1);
        assert_eq!(solve::<15, 2, 20>(get_example_input()), 1 + 1 + 1 + 1 + 1);
        // 5 singles
        assert_eq!(solve::<15, 2, 12>(get_example_input()), 5 + 3);
        assert_eq!(solve::<15, 2, 10>(get_example_input()), 5 + 3 + 2);
        assert_eq!(solve::<15, 2, 8>(get_example_input()), 5 + 3 + 2 + 4);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 1417)
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve::<15, 20, 76>(get_example_input()), 3);
        assert_eq!(solve::<15, 20, 74>(get_example_input()), 3 + 4);
        assert_eq!(solve::<15, 20, 72>(get_example_input()), 3 + 4 + 22);
        assert_eq!(solve::<15, 20, 70>(get_example_input()), 3 + 4 + 22 + 12);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 1014683)
    }
}
