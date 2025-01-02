const GRID_WIDTH: usize = 101;
const GRID_WIDTH_I32: i32 = GRID_WIDTH as i32;
const GRID_HEIGHT: usize = 103;
const GRID_HEIGHT_I32: i32 = GRID_HEIGHT as i32;
const BORDER_SIZE: usize = 31;

const BITS: [u64; BORDER_SIZE + 1] = [
    0,
    0x00_00_00_01,
    0x00_00_00_03,
    0x00_00_00_07,
    0x00_00_00_0F,
    0x00_00_00_1F,
    0x00_00_00_3F,
    0x00_00_00_7F,
    0x00_00_00_FF,
    0x00_00_01_FF,
    0x00_00_03_FF,
    0x00_00_07_FF,
    0x00_00_0F_FF,
    0x00_00_1F_FF,
    0x00_00_3F_FF,
    0x00_00_7F_FF,
    0x00_00_FF_FF,
    0x00_01_FF_FF,
    0x00_03_FF_FF,
    0x00_07_FF_FF,
    0x00_0F_FF_FF,
    0x00_1F_FF_FF,
    0x00_3F_FF_FF,
    0x00_7F_FF_FF,
    0x00_FF_FF_FF,
    0x01_FF_FF_FF,
    0x03_FF_FF_FF,
    0x07_FF_FF_FF,
    0x0F_FF_FF_FF,
    0x1F_FF_FF_FF,
    0x3F_FF_FF_FF,
    0xFF_FF_FF_FF,
];

#[derive(Copy, Clone, Debug)]
struct Robot {
    x: i32,
    y: i32,

    v_x: i32,
    v_y: i32,
}

#[inline(always)]
fn new_robot(input: &[u8], i: &mut usize, robot: &mut Robot) {
    // Parse start x
    *i += 2;
    robot.x += (input[*i] - b'0') as i32;
    *i += 1;
    while input[*i] != b',' {
        robot.x *= 10;
        robot.x += (input[*i] - b'0') as i32;
        *i += 1;
    }

    // Parse start y.
    *i += 1;
    robot.y += (input[*i] - b'0') as i32;
    *i += 1;
    while input[*i] != b' ' {
        robot.y *= 10;
        robot.y += (input[*i] - b'0') as i32;
        *i += 1;
    }

    // Parse velocity
    *i += 3;
    let is_neg = input[*i] == b'-';
    if is_neg {
        *i += 1;
    }
    robot.v_x += (input[*i] - b'0') as i32;
    *i += 1;
    while input[*i] != b',' {
        robot.v_x *= 10;
        robot.v_x += (input[*i] - b'0') as i32;
        *i += 1;
    }
    if is_neg {
        robot.v_x = -robot.v_x;
    }

    // Parse v_y
    *i += 1;
    let is_neg = input[*i] == b'-';
    if is_neg {
        *i += 1;
    }

    robot.v_y += (input[*i] - b'0') as i32;
    *i += 1;
    while *i < input.len() && input[*i] != b'\n' {
        robot.v_y *= 10;
        robot.v_y += (input[*i] - b'0') as i32;
        *i += 1;
    }
    if is_neg {
        robot.v_y = -robot.v_y;
    }
    *i += 1;
}

#[inline(always)]
fn step_through_time(robot: &mut Robot) {
    robot.x = (robot.x + robot.v_x) % GRID_WIDTH_I32;
    robot.y = (robot.y + robot.v_y) % GRID_HEIGHT_I32;
    if robot.x < 0 {
        robot.x += GRID_WIDTH_I32;
    }
    if robot.y < 0 {
        robot.y += GRID_HEIGHT_I32;
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u32 {
    const NUM_STEPS: i32 = 100;

    let input = input.as_bytes();

    let mut i = 0;
    let mut ur = 0u32;
    let mut ul = 0u32;
    let mut ll = 0u32;
    let mut lr = 0u32;

    while i < input.len() {
        let mut x = 0i32;
        let mut y = 0i32;
        let mut v_x = 0i32;
        let mut v_y = 0i32;

        i += 2;
        x += (input[i] - b'0') as i32;
        i += 1;
        while input[i] != b',' {
            x *= 10;
            x += (input[i] - b'0') as i32;
            i += 1;
        }

        i += 1;

        y += (input[i] - b'0') as i32;
        i += 1;
        while input[i] != b' ' {
            y *= 10;
            y += (input[i] - b'0') as i32;
            i += 1;
        }

        i += 3;
        let is_neg = input[i] == b'-';
        if is_neg {
            i += 1;
        }
        v_x += (input[i] - b'0') as i32;
        i += 1;
        while input[i] != b',' {
            v_x *= 10;
            v_x += (input[i] - b'0') as i32;
            i += 1;
        }
        if is_neg {
            v_x *= -1;
        }

        i += 1;

        let is_neg = input[i] == b'-';
        if is_neg {
            i += 1;
        }

        v_y += (input[i] - b'0') as i32;
        i += 1;
        while i < input.len() && input[i] != b'\n' {
            v_y *= 10;
            v_y += (input[i] - b'0') as i32;
            i += 1;
        }
        if is_neg {
            v_y *= -1;
        }

        i += 1;

        // 100 steps, 101 is the width, 103 is the height.
        x = (x + (v_x * NUM_STEPS)) % GRID_WIDTH_I32;
        y = (y + (v_y * NUM_STEPS)) % GRID_HEIGHT_I32;
        if x < 0 {
            x += GRID_WIDTH_I32;
        }
        if y < 0 {
            y += GRID_HEIGHT_I32;
        }
        if y == 51 || x == 50 {
            continue;
        }
        if y < 51 {
            if x < 50 {
                ul += 1;
            } else {
                ur += 1;
            }
        } else {
            if x < 50 {
                ll += 1;
            } else {
                lr += 1;
            }
        }
    }

    return ur * ul * ll * lr;
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    // The space is 101 tiles wide and 103 tall.
    // index is based on x, since that is 101, not 103.
    // exists[0..101] represents the 0th through 63rd rows of col x
    // exists[101..202] represents the 64th through 102nd (aka last) row of col (x-101)
    let mut exists = [0u64; 202];
    let mut num_steps = 0;
    let mut good: bool = true;

    let input = input.as_bytes();

    let mut robots: [Robot; 500] = [Robot {
        x: 0,
        y: 0,
        v_x: 0,
        v_y: 0,
    }; 500];

    let mut i = 0;
    let mut r_i = 0;

    while i < input.len() {
        new_robot(input, &mut i, &mut robots[r_i]);
        r_i += 1;
    }

    // attempt to repro https://www.reddit.com/r/adventofcode/comments/1hdvhvu/comment/m1zws1g/
    let mut bx = 0;
    let mut bx_val = i32::MAX;
    let mut by = 0;
    let mut by_val = i32::MAX;
    for val in 0..GRID_HEIGHT_I32 {
        let (var_x, var_y) = get_variance(&mut robots);
        if var_x < bx_val {
            bx = val;
            bx_val = var_x;
        }
        if var_y < by_val {
            by = val;
            by_val = var_y;
        }
    }

    // pow(101, -1, 103) = 51
    return bx + ((51 * (by - bx)) % GRID_HEIGHT_I32) * GRID_WIDTH_I32;
}

#[inline(always)]
fn get_variance(robots: &mut [Robot; 500]) -> (i32, i32) {
    let mut var_x: i32 = 0i32;
    let mut var_y: i32 = 0i32;

    for robot in robots.iter_mut() {
        if robot.y < 51 {
            var_y += 51 - robot.y;
        } else {
            var_y += robot.y - 51;
        }
        if robot.x < 50 {
            var_x += 50 - robot.x;
        } else {
            var_x += robot.x - 50;
        }
        step_through_time(robot);
    }

    return (var_x, var_y);
}

#[allow(dead_code)]
fn print_robots(exists: &[u64; 202]) {
    print!(".");
    for _ in 0..GRID_WIDTH {
        print!("-");
    }
    println!(".");

    for r in 0..63 {
        print!("|");
        for c in 0..GRID_WIDTH {
            if exists[c] & 1 << r == 0 {
                print!(" ");
            } else {
                print!("X");
            }
        }
        println!("|");
    }
    for r in 64..GRID_HEIGHT {
        print!("|");
        for c in 0..GRID_WIDTH {
            if exists[GRID_WIDTH + c] & 1 << (r - 64) == 0 {
                print!(" ");
            } else {
                print!("X");
            }
        }
        println!("|");
    }

    print!("'");
    for _ in 0..GRID_WIDTH {
        print!("-");
    }
    println!("'");
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day14.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_competition_input() -> String {
        let input_path = "input/2024/day14_competition.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_input_2() -> String {
        let input_path = "input/2024/day14_2.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 224438715);
        assert_eq!(part1(&get_competition_input()), 209409792);
        assert_eq!(part1(&get_input_2()), 231221760);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 7603);
        assert_eq!(part2(&get_input_2()), 6771);
        assert_eq!(part2(&get_competition_input()), 8006);
    }
}
