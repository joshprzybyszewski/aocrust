const GRID_SIZE: usize = 130;
// const GRID_SIZE: usize = 10;

const BLOCK_UP: u8 = 1 << 0;
const BLOCK_RIGHT: u8 = 1 << 1;
const BLOCK_DOWN: u8 = 1 << 2;
const BLOCK_LEFT: u8 = 1 << 3;

const SEEN_UP: u8 = UP; // 1 << 4;
const SEEN_RIGHT: u8 = RIGHT; // 1 << 5;
const SEEN_DOWN: u8 = DOWN; // 1 << 6;
const SEEN_LEFT: u8 = LEFT; // 1 << 7;
const SEEN_MASK: u8 = SEEN_UP | SEEN_RIGHT | SEEN_DOWN | SEEN_LEFT;

const UP: u8 = 1 << 4;
const RIGHT: u8 = 1 << 5;
const DOWN: u8 = 1 << 6;
const LEFT: u8 = 1 << 7;

#[derive(Copy, Clone)]
struct Guard {
    r: usize,
    c: usize,
    dir: u8,
}

fn march(grid: &mut [[u8; GRID_SIZE]; GRID_SIZE], guard: Guard) -> usize {
    let mut guard = guard;
    let mut seen: usize = 1;
    grid[guard.r][guard.c] |= SEEN_UP;
    loop {
        match guard.dir {
            UP => {
                if grid[guard.r][guard.c] & BLOCK_UP == BLOCK_UP {
                    if guard.r == 0 {
                        return seen + 1;
                    }
                    guard.dir = RIGHT;
                } else {
                    if grid[guard.r][guard.c] & SEEN_MASK == 0 {
                        seen += 1;
                    }
                    grid[guard.r][guard.c] |= SEEN_UP;
                    guard.r -= 1;
                }
            }
            RIGHT => {
                if grid[guard.r][guard.c] & BLOCK_RIGHT == BLOCK_RIGHT {
                    if guard.c == GRID_SIZE - 1 {
                        return seen + 1;
                    }
                    guard.dir = DOWN;
                } else {
                    if grid[guard.r][guard.c] & SEEN_MASK == 0 {
                        seen += 1;
                    }
                    grid[guard.r][guard.c] |= SEEN_RIGHT;
                    guard.c += 1;
                }
            }
            DOWN => {
                if grid[guard.r][guard.c] & BLOCK_DOWN == BLOCK_DOWN {
                    if guard.r == GRID_SIZE - 1 {
                        return seen + 1;
                    }
                    guard.dir = LEFT;
                } else {
                    if grid[guard.r][guard.c] & SEEN_MASK == 0 {
                        seen += 1;
                    }
                    grid[guard.r][guard.c] |= SEEN_DOWN;
                    guard.r += 1;
                }
            }
            LEFT => {
                if grid[guard.r][guard.c] & BLOCK_LEFT == BLOCK_LEFT {
                    if guard.c == 0 {
                        return seen + 1;
                    }
                    guard.dir = UP;
                } else {
                    if grid[guard.r][guard.c] & SEEN_MASK == 0 {
                        seen += 1;
                    }
                    grid[guard.r][guard.c] |= SEEN_LEFT;
                    guard.c -= 1;
                }
            }
            _ => unreachable!(),
        }
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut i: usize = 0;

    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
    let mut guard: Guard = Guard {
        r: 0,
        c: 0,
        dir: UP,
    };
    for i in 0..GRID_SIZE {
        grid[0][i] |= BLOCK_UP;
        grid[GRID_SIZE - 1][i] |= BLOCK_DOWN;
        grid[i][0] |= BLOCK_LEFT;
        grid[i][GRID_SIZE - 1] |= BLOCK_RIGHT;
    }

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            match input[i] {
                b'.' => grid[r][c] = 0,
                b'#' => {
                    if r > 0 {
                        grid[r - 1][c] |= BLOCK_DOWN;
                    }
                    if r < GRID_SIZE - 1 {
                        grid[r + 1][c] |= BLOCK_UP;
                    }
                    if c > 0 {
                        grid[r][c - 1] |= BLOCK_RIGHT;
                    }
                    if c < GRID_SIZE - 1 {
                        grid[r][c + 1] |= BLOCK_LEFT;
                    }
                }
                b'^' => {
                    guard.r = r;
                    guard.c = c;
                }
                _ => unreachable!(),
            }
            i += 1;
        }
        i += 1; // input[i] is a newline
    }

    return march(&mut grid, guard);
}

fn march_2(grid: &mut [[u8; GRID_SIZE]; GRID_SIZE], guard: Guard) -> bool {
    let mut guard = guard;
    grid[guard.r][guard.c] |= SEEN_UP;
    loop {
        if grid[guard.r][guard.c] & guard.dir == guard.dir {
            return true;
        }
        grid[guard.r][guard.c] |= guard.dir;

        match guard.dir {
            UP => {
                if grid[guard.r][guard.c] & BLOCK_UP == BLOCK_UP {
                    if guard.r == 0 {
                        return false;
                    }
                    guard.dir = RIGHT;
                } else {
                    guard.r -= 1;
                }
            }
            RIGHT => {
                if grid[guard.r][guard.c] & BLOCK_RIGHT == BLOCK_RIGHT {
                    if guard.c == GRID_SIZE - 1 {
                        return false;
                    }
                    guard.dir = DOWN;
                } else {
                    guard.c += 1;
                }
            }
            DOWN => {
                if grid[guard.r][guard.c] & BLOCK_DOWN == BLOCK_DOWN {
                    if guard.r == GRID_SIZE - 1 {
                        return false;
                    }
                    guard.dir = LEFT;
                } else {
                    guard.r += 1;
                }
            }
            LEFT => {
                if grid[guard.r][guard.c] & BLOCK_LEFT == BLOCK_LEFT {
                    if guard.c == 0 {
                        return false;
                    }
                    guard.dir = UP;
                } else {
                    guard.c -= 1;
                }
            }
            _ => unreachable!(),
        }
    }
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let mut i: usize = 0;

    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
    let mut guard: Guard = Guard {
        r: 0,
        c: 0,
        dir: UP,
    };
    for i in 0..GRID_SIZE {
        grid[0][i] |= BLOCK_UP;
        grid[GRID_SIZE - 1][i] |= BLOCK_DOWN;
        grid[i][0] |= BLOCK_LEFT;
        grid[i][GRID_SIZE - 1] |= BLOCK_RIGHT;
    }

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            match input[i] {
                b'.' => grid[r][c] = 0,
                b'#' => {
                    if r > 0 {
                        grid[r - 1][c] |= BLOCK_UP;
                    }
                    if r < GRID_SIZE - 1 {
                        grid[r + 1][c] |= BLOCK_DOWN;
                    }
                    if c > 0 {
                        grid[r][c - 1] |= BLOCK_RIGHT;
                    }
                    if c < GRID_SIZE - 1 {
                        grid[r][c + 1] |= BLOCK_LEFT;
                    }
                }
                b'^' => {
                    guard.r = r;
                    guard.c = c;
                }
                _ => unreachable!(),
            }
            i += 1;
        }
        i += 1; // input[i] is a newline
    }
    let guard = guard;

    let _ = march(&mut grid, guard);

    let mut total: usize = 0;
    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            if grid[r][c] & SEEN_MASK == 0 {
                // was not seen
                continue;
            }
            if guard.r == r && guard.c == c {
                // the guard's starting pos
                continue;
            }
            // Consider running march_2 in concurrently with all the others
            let mut grid = grid;
            if r > 0 {
                grid[r - 1][c] = BLOCK_DOWN;
            }
            if r < GRID_SIZE - 1 {
                grid[r + 1][c] = BLOCK_UP;
            }
            if c > 0 {
                grid[r][c - 1] = BLOCK_RIGHT;
            }
            if c < GRID_SIZE - 1 {
                grid[r][c + 1] = BLOCK_LEFT;
            }
            if march_2(&mut grid, guard) {
                total += 1;
            }
        }
    }
    return total;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_example_input() -> String {
        let input_path = "input/2024/examples/day6.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_input() -> String {
        let input_path = "input/2024/day6.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), 6);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 4890)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 1995)
    }
}
