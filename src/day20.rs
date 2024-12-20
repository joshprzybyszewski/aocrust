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

fn solve(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut start: Option<Coord> = None;
    let mut goal: Option<Coord> = None;
    let mut grid: [[u64; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE] =
        [[0; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE];

    let mut i: usize = 0;

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            match input[i] {
                b'#' => {}
                b'.' => {
                    grid[r + BUFFER][c + BUFFER] = u64::MAX;
                }
                b'S' => {
                    grid[r + BUFFER][c + BUFFER] = u64::MAX;
                    start = Some(Coord::new(r + BUFFER, c + BUFFER));
                }
                b'E' => {
                    grid[r + BUFFER][c + BUFFER] = u64::MAX;
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

    return dfs(&mut grid, 1, start, &goal);
}

fn dfs(
    grid: &mut [[u64; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE],
    pos: u64,
    current: Coord,
    goal: &Coord,
) -> u64 {
    // println!("dfs({pos}, {:?})", current);
    grid[current.row][current.col] = pos;

    let mut cheats = 0;
    if current != *goal {
        let next: Coord;
        if grid[current.row - 1][current.col] == u64::MAX {
            next = current.up();
        } else if grid[current.row + 1][current.col] == u64::MAX {
            next = current.down();
        } else if grid[current.row][current.col - 1] == u64::MAX {
            next = current.left();
        } else if grid[current.row][current.col + 1] == u64::MAX {
            next = current.right();
        } else {
            println!("current: {:?}", current);
            unreachable!();
        }
        cheats += dfs(grid, pos + 1, next, goal);
    }

    if grid[current.row - 2][current.col] > pos + 100 {
        cheats += 1;
    }
    if grid[current.row + 2][current.col] > pos + 100 {
        cheats += 1;
    }
    if grid[current.row][current.col - 2] > pos + 100 {
        cheats += 1;
    }
    if grid[current.row][current.col + 2] > pos + 100 {
        cheats += 1;
    }

    return cheats;
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> u64 {
    return solve(input);
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> u64 {
    return 0;
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
        assert_eq!(part2(&get_input()), 1)
    }
}