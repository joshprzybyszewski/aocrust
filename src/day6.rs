const GRID_SIZE: usize = 130;
// const GRID_SIZE: usize = 10;

const BLOCK: u8 = 1 << 0;
const VISITED: u8 = 1 << 1;

const UP: u8 = 1 << 2;
const RIGHT: u8 = 1 << 3;
const DOWN: u8 = 1 << 4;
const LEFT: u8 = 1 << 5;

#[derive(Copy, Clone)]
struct Guard {
    r: usize,
    c: usize,
    dir: u8,
}

fn march(grid: &mut [[u8; GRID_SIZE]; GRID_SIZE], guard: Guard) -> usize {
    let mut guard = guard;
    let mut seen: usize = 1;
    grid[guard.r][guard.c] |= VISITED;
    loop {
        match guard.dir {
            UP => {
                if guard.r == 0 {
                    grid[guard.r][guard.c] |= VISITED;
                    return seen + 1;
                }
                if grid[guard.r - 1][guard.c] == BLOCK {
                    guard.dir <<= 1;
                } else {
                    if grid[guard.r][guard.c] & VISITED != VISITED {
                        seen += 1;
                    }
                    grid[guard.r][guard.c] |= VISITED;
                    guard.r -= 1;
                }
            }
            RIGHT => {
                if guard.c == GRID_SIZE - 1 {
                    grid[guard.r][guard.c] |= VISITED;
                    return seen + 1;
                }
                if grid[guard.r][guard.c + 1] == BLOCK {
                    guard.dir <<= 1;
                } else {
                    if grid[guard.r][guard.c] & VISITED != VISITED {
                        seen += 1;
                    }
                    grid[guard.r][guard.c] |= VISITED;
                    guard.c += 1;
                }
            }
            DOWN => {
                if guard.r == GRID_SIZE - 1 {
                    grid[guard.r][guard.c] |= VISITED;
                    return seen + 1;
                }
                if grid[guard.r + 1][guard.c] == BLOCK {
                    guard.dir <<= 1;
                } else {
                    if grid[guard.r][guard.c] & VISITED != VISITED {
                        seen += 1;
                    }
                    grid[guard.r][guard.c] |= VISITED;
                    guard.r += 1;
                }
            }
            LEFT => {
                if guard.c == 0 {
                    grid[guard.r][guard.c] |= VISITED;
                    return seen + 1;
                }
                if grid[guard.r][guard.c - 1] == BLOCK {
                    guard.dir = UP;
                } else {
                    if grid[guard.r][guard.c] & VISITED != VISITED {
                        seen += 1;
                    }
                    grid[guard.r][guard.c] |= VISITED;
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

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            match input[i] {
                b'.' => grid[r][c] = 0,
                b'#' => grid[r][c] = BLOCK,
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
    loop {
        if grid[guard.r][guard.c] & guard.dir == guard.dir {
            return true;
        }
        grid[guard.r][guard.c] |= guard.dir;

        match guard.dir {
            UP => {
                if guard.r == 0 {
                    return false;
                }
                if grid[guard.r - 1][guard.c] == BLOCK {
                    guard.dir <<= 1;
                } else {
                    guard.r -= 1;
                }
            }
            RIGHT => {
                if guard.c == GRID_SIZE - 1 {
                    return false;
                }
                if grid[guard.r][guard.c + 1] == BLOCK {
                    guard.dir <<= 1;
                } else {
                    guard.c += 1;
                }
            }
            DOWN => {
                if guard.r == GRID_SIZE - 1 {
                    return false;
                }
                if grid[guard.r + 1][guard.c] == BLOCK {
                    guard.dir <<= 1;
                } else {
                    guard.r += 1;
                }
            }
            LEFT => {
                if guard.c == 0 {
                    return false;
                }
                if grid[guard.r][guard.c - 1] == BLOCK {
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

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            match input[i] {
                b'.' => grid[r][c] = 0,
                b'#' => grid[r][c] = BLOCK,
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
    let mut prev: [[u8; 130]; 130];
    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            if grid[r][c] & VISITED != VISITED {
                continue;
            }
            if guard.r == r && guard.c == c {
                // the guard's starting pos
                continue;
            }
            // Consider running march_2 in concurrently with all the others
            prev = grid;
            grid[r][c] = BLOCK;
            if march_2(&mut grid, guard) {
                total += 1;
            }
            grid = prev;
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

// const MOVEMENT_MASK: u8 = UP | RIGHT | DOWN | LEFT;

// fn printGrid(grid: [[u8; GRID_SIZE]; GRID_SIZE]) {
//     println!("----------------------");
//     for row in grid {
//         let mut s: String = "".to_string();
//         for val in row {
//             if val & BLOCK == BLOCK {
//                 s = format!("{s}#");
//                 continue;
//             }
//             let val = val & MOVEMENT_MASK;

//             let mut c = ".";
//             if val == UP {
//                 c = "^"
//             } else if val == RIGHT {
//                 c = ">"
//             } else if val == DOWN {
//                 c = "v"
//             } else if val == LEFT {
//                 c = "<"
//             } else if val == UP | DOWN {
//                 c = "|"
//             } else if val == LEFT | RIGHT {
//                 c = "-"
//             } else if val == UP | RIGHT {
//                 c = "+"
//             } else if val == UP | LEFT {
//                 c = "+"
//             } else if val == DOWN | RIGHT {
//                 c = "+"
//             } else if val == DOWN | LEFT {
//                 c = "+"
//             } else if val > 0 {
//                 c = "?"
//             }
//             s = format!("{s}{c}");
//         }
//         println!("{s}");
//     }
//     println!("----------------------");
// }
