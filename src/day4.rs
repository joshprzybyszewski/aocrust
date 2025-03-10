const GRID_SIZE: usize = 140;
const GRID_SIZE_LESS_1: usize = GRID_SIZE - 1;
const GRID_SIZE_LESS_2: usize = GRID_SIZE - 2;
const GRID_SIZE_LESS_3: usize = GRID_SIZE - 3;

const M_SET: u8 = 1 << 0;
const A_SET: u8 = 1 << 1;
const S_SET: u8 = 1 << 2;
const ALL_SET: u8 = M_SET | A_SET | S_SET;

const RIGHT: usize = 0;
const UR: usize = 1;
const UP: usize = 2;
const UL: usize = 3;
const LEFT: usize = 4;
const DL: usize = 5;
const DOWN: usize = 6;
const DR: usize = 7;

#[derive(Copy, Clone)]
struct CoordP1 {
    dirs: [u8; 8],
    is_x: bool,
}

fn p1set_x(grid: &mut [[CoordP1; GRID_SIZE]; GRID_SIZE], r: usize, c: usize) {
    grid[r][c].is_x = true
}

fn p1set_m(grid: &mut [[CoordP1; GRID_SIZE]; GRID_SIZE], r: usize, c: usize) {
    if c > 0 {
        grid[r][c - 1].dirs[RIGHT] |= M_SET;
        if r > 0 {
            grid[r - 1][c - 1].dirs[DR] |= M_SET;
        }
        if r < GRID_SIZE_LESS_1 {
            grid[r + 1][c - 1].dirs[UR] |= M_SET;
        }
    }
    if c < GRID_SIZE_LESS_1 {
        grid[r][c + 1].dirs[LEFT] |= M_SET;
        if r > 0 {
            grid[r - 1][c + 1].dirs[DL] |= M_SET;
        }
        if r < GRID_SIZE_LESS_1 {
            grid[r + 1][c + 1].dirs[UL] |= M_SET;
        }
    }
    if r > 0 {
        grid[r - 1][c].dirs[DOWN] |= M_SET;
    }
    if r < GRID_SIZE_LESS_1 {
        grid[r + 1][c].dirs[UP] |= M_SET;
    }
}

fn p1set_a(grid: &mut [[CoordP1; GRID_SIZE]; GRID_SIZE], r: usize, c: usize) {
    if c > 1 {
        grid[r][c - 2].dirs[RIGHT] |= A_SET;
        if r > 1 {
            grid[r - 2][c - 2].dirs[DR] |= A_SET;
        }
        if r < GRID_SIZE_LESS_2 {
            grid[r + 2][c - 2].dirs[UR] |= A_SET;
        }
    }
    if c < GRID_SIZE_LESS_2 {
        grid[r][c + 2].dirs[LEFT] |= A_SET;
        if r > 1 {
            grid[r - 2][c + 2].dirs[DL] |= A_SET;
        }
        if r < GRID_SIZE_LESS_2 {
            grid[r + 2][c + 2].dirs[UL] |= A_SET;
        }
    }
    if r > 1 {
        grid[r - 2][c].dirs[DOWN] |= A_SET;
    }
    if r < GRID_SIZE_LESS_2 {
        grid[r + 2][c].dirs[UP] |= A_SET;
    }
}

fn p1set_s(grid: &mut [[CoordP1; GRID_SIZE]; GRID_SIZE], r: usize, c: usize) {
    if c > 2 {
        grid[r][c - 3].dirs[RIGHT] |= S_SET;
        if r > 2 {
            grid[r - 3][c - 3].dirs[DR] |= S_SET;
        }
        if r < GRID_SIZE_LESS_3 {
            grid[r + 3][c - 3].dirs[UR] |= S_SET;
        }
    }
    if c < GRID_SIZE_LESS_3 {
        grid[r][c + 3].dirs[LEFT] |= S_SET;
        if r > 2 {
            grid[r - 3][c + 3].dirs[DL] |= S_SET;
        }
        if r < GRID_SIZE_LESS_3 {
            grid[r + 3][c + 3].dirs[UL] |= S_SET;
        }
    }
    if r > 2 {
        grid[r - 3][c].dirs[DOWN] |= S_SET;
    }
    if r < GRID_SIZE_LESS_3 {
        grid[r + 3][c].dirs[UP] |= S_SET;
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let input = input.as_bytes();
    let mut grid: [[CoordP1; GRID_SIZE]; GRID_SIZE] = [[CoordP1 {
        dirs: [0; 8],
        is_x: false,
    }; GRID_SIZE]; GRID_SIZE];

    let mut i: usize = 0;

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            match input[i] {
                b'M' => p1set_m(&mut grid, r, c),
                b'A' => p1set_a(&mut grid, r, c),
                b'S' => p1set_s(&mut grid, r, c),
                b'X' => p1set_x(&mut grid, r, c),
                _ => unreachable!(),
            }
            i += 1;
        }
        i += 1; // input[i] is a newline
    }

    let mut total = 0;
    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            if !grid[r][c].is_x {
                continue;
            }
            for dir in grid[r][c].dirs {
                if dir == ALL_SET {
                    total += 1;
                }
            }
        }
    }

    return total;
}

const X_S_UL_SET: u8 = 1 << 0;
const X_S_UR_SET: u8 = 1 << 1;
const X_S_DL_SET: u8 = 1 << 2;
const X_S_DR_SET: u8 = 1 << 3;
const X_M_UL_SET: u8 = 1 << 4;
const X_M_UR_SET: u8 = 1 << 5;
const X_M_DL_SET: u8 = 1 << 6;
const X_M_DR_SET: u8 = 1 << 7;

const XMAS_1: u8 = X_S_UL_SET | X_S_UR_SET | X_M_DL_SET | X_M_DR_SET;
const XMAS_2: u8 = X_S_UR_SET | X_S_DR_SET | X_M_DL_SET | X_M_UL_SET;
const XMAS_3: u8 = X_S_DR_SET | X_S_DL_SET | X_M_UL_SET | X_M_UR_SET;
const XMAS_4: u8 = X_S_DL_SET | X_S_UL_SET | X_M_UR_SET | X_M_DR_SET;

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let input = input.as_bytes();
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    let mut i: usize = 0;
    match input[i] {
        b'M' => {
            grid[1][1] |= X_M_UL_SET;
        }
        b'S' => {
            grid[1][1] |= X_S_UL_SET;
        }
        _ => {}
    }
    i += 1;

    for c in 1..GRID_SIZE_LESS_1 {
        // don't check up, only down
        match input[i] {
            b'M' => {
                grid[1][c + 1] |= X_M_UL_SET;
                grid[1][c - 1] |= X_M_UR_SET;
            }
            b'S' => {
                grid[1][c + 1] |= X_S_UL_SET;
                grid[1][c - 1] |= X_S_UR_SET;
            }
            _ => {}
        }
        i += 1;
    }

    match input[i] {
        b'M' => {
            grid[1][GRID_SIZE_LESS_2] |= X_M_UR_SET;
        }
        b'S' => {
            grid[1][GRID_SIZE_LESS_2] |= X_S_UR_SET;
        }
        _ => {}
    }
    i += 1;

    //input[i] is now a newline
    i += 1;

    for r in 1..GRID_SIZE_LESS_1 {
        match input[i] {
            b'M' => {
                grid[r - 1][1] |= X_M_DL_SET;
                grid[r + 1][1] |= X_M_UL_SET;
            }
            b'S' => {
                grid[r - 1][1] |= X_S_DL_SET;
                grid[r + 1][1] |= X_S_UL_SET;
            }
            _ => {}
        }
        i += 1;

        for c in 1..GRID_SIZE_LESS_1 {
            match input[i] {
                b'M' => {
                    grid[r][c] = 0;
                    // (r +/- 1) and (c +/- 1) are always in bounds
                    grid[r - 1][c - 1] |= X_M_DR_SET;
                    grid[r + 1][c - 1] |= X_M_UR_SET;
                    grid[r - 1][c + 1] |= X_M_DL_SET;
                    grid[r + 1][c + 1] |= X_M_UL_SET;
                }
                b'S' => {
                    grid[r][c] = 0;
                    // (r +/- 1) and (c +/- 1) are always in bounds
                    grid[r - 1][c - 1] |= X_S_DR_SET;
                    grid[r + 1][c - 1] |= X_S_UR_SET;
                    grid[r - 1][c + 1] |= X_S_DL_SET;
                    grid[r + 1][c + 1] |= X_S_UL_SET;
                }
                b'X' => grid[r][c] = 0,
                _ => {}
            }
            i += 1;
        }

        match input[i] {
            b'M' => {
                grid[r - 1][GRID_SIZE_LESS_2] |= X_M_DR_SET;
                grid[r + 1][GRID_SIZE_LESS_2] |= X_M_UR_SET;
            }
            b'S' => {
                grid[r - 1][GRID_SIZE_LESS_2] |= X_S_DR_SET;
                grid[r + 1][GRID_SIZE_LESS_2] |= X_S_UR_SET;
            }
            _ => {}
        }
        i += 1;

        i += 1; // input[i] is a newline
    }

    match input[i] {
        b'M' => {
            grid[GRID_SIZE_LESS_2][1] |= X_M_DL_SET;
        }
        b'S' => {
            grid[GRID_SIZE_LESS_2][1] |= X_S_DL_SET;
        }
        _ => {}
    }
    i += 1;

    for c in 1..GRID_SIZE_LESS_1 {
        // don't check up, only down
        match input[i] {
            b'M' => {
                grid[GRID_SIZE_LESS_2][c + 1] |= X_M_DL_SET;
                grid[GRID_SIZE_LESS_2][c - 1] |= X_M_DR_SET;
            }
            b'S' => {
                grid[GRID_SIZE_LESS_2][c + 1] |= X_S_DL_SET;
                grid[GRID_SIZE_LESS_2][c - 1] |= X_S_DR_SET;
            }
            _ => {}
        }
        i += 1;
    }

    match input[i] {
        b'M' => {
            grid[GRID_SIZE_LESS_2][GRID_SIZE_LESS_2] |= X_M_DR_SET;
        }
        b'S' => {
            grid[GRID_SIZE_LESS_2][GRID_SIZE_LESS_2] |= X_S_DR_SET;
        }
        _ => {}
    }
    // i += 1; // don't need to increment cuz we don't use it again.

    let mut total = 0;
    for r in 1..GRID_SIZE_LESS_1 {
        for c in 1..GRID_SIZE_LESS_1 {
            if grid[r][c] == XMAS_1
                || grid[r][c] == XMAS_2
                || grid[r][c] == XMAS_3
                || grid[r][c] == XMAS_4
            {
                total += 1;
            }
        }
    }
    return total;
}
