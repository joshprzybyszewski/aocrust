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

    #[inline(always)]
    fn offset(&self, dr: i32, dc: i32) -> Self {
        return Coord {
            row: (self.row as i32 + dr) as usize,
            col: (self.col as i32 + dc) as usize,
        };
    }
}

#[inline(always)]
fn solve<const SIZE: usize, const CHEAT: i32, const SAVE: i32>(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut start: Option<Coord> = None;
    let mut goal: Option<Coord> = None;
    let mut grid: [[i32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE] =
        [[0; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE];

    let mut i: usize = 0;

    for r in 0..SIZE {
        for c in 0..SIZE {
            match input[i] {
                b'#' => {}
                b'.' => {
                    grid[r + BUFFER][c + BUFFER] = i32::MAX;
                }
                b'S' => {
                    grid[r + BUFFER][c + BUFFER] = i32::MAX;
                    start = Some(Coord::new(r + BUFFER, c + BUFFER));
                }
                b'E' => {
                    grid[r + BUFFER][c + BUFFER] = i32::MAX;
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

fn dfs<const CHEAT: i32, const SAVE: i32>(
    grid: &mut [[i32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE],
    pos: i32,
    current: Coord,
    goal: &Coord,
) -> u32 {
    // println!("dfs({pos}, {:?})", current);
    grid[current.row][current.col] = pos;

    if current == *goal {
        if CHEAT == 2 {
            return count_cheats_2::<SAVE>(grid, current);
        }
        return count_cheats::<CHEAT, SAVE>(grid, current);
    }

    let next: Coord;
    if grid[current.row - 1][current.col] == i32::MAX {
        next = current.up();
    } else if grid[current.row + 1][current.col] == i32::MAX {
        next = current.down();
    } else if grid[current.row][current.col - 1] == i32::MAX {
        next = current.left();
    } else {
        // assume (grid[current.row][current.col + 1] == i32::MAX)
        next = current.right();
    }

    // do dfs first, then count cheats.
    let prev = dfs::<CHEAT, SAVE>(grid, pos + 1, next, goal);

    // TODO there's some mad caching we could do if we know how to inspect only the
    // diff of the changed diamond.
    if CHEAT == 2 {
        return prev + count_cheats_2::<SAVE>(grid, current);
    }

    return prev + count_cheats::<CHEAT, SAVE>(grid, current);
}

#[inline(always)]
fn count_cheats_2<const SAVE: i32>(
    grid: &[[i32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE],
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

#[inline(always)]
fn count_cheats<const CHEAT: i32, const SAVE: i32>(
    grid: &[[i32; TOTAL_GRID_SIZE]; TOTAL_GRID_SIZE],
    current: Coord,
) -> u32 {
    let mut cheats = 0;

    // Shoutout to maneatingape for this pro tip.
    // https://github.com/maneatingape/advent-of-code-rust/blob/5778c24f8881392cb5d4c64bc3f010a1a1bbc8af/src/year2024/day20.rs#L71-L77

    let mut score_to_beat = SAVE + 1;
    // let score_to_beat = grid[current.row][current.col] + SAVE;

    for cheat_dist in 2..=CHEAT {
        let mut right = current.offset(0, cheat_dist);
        let mut down = current.offset(cheat_dist, 0);
        score_to_beat += 1;
        for _ in 0..cheat_dist {
            if grid[right.row][right.col] != 0
                && (grid[right.row][right.col] - grid[current.row][current.col]).abs()
                    >= score_to_beat
            {
                cheats += 1;
            }
            if grid[down.row][down.col] != 0
                && (grid[down.row][down.col] - grid[current.row][current.col]).abs()
                    >= score_to_beat
            {
                cheats += 1;
            }

            right.row += 1;
            right.col -= 1;

            down.row -= 1;
            down.col -= 1;
        }
    }

    return cheats;
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
