const UPPER_RIGHT: usize = 0;
const UPPER_LEFT: usize = 1;
const LOWER_LEFT: usize = 2;
const LOWER_RIGHT: usize = 3;
const X_AXIS: usize = 4;
const Y_AXIS: usize = 5;
const CENTER: usize = 6;

#[derive(Copy, Clone, Debug)]
struct Robot {
    x: i32,
    y: i32,

    v_x: i32,
    v_y: i32,

    time: i32,
}

impl Robot {
    fn new(input: &[u8], i: &mut usize) -> Self {
        let mut robot = Robot {
            x: 0,
            y: 0,
            v_x: 0,
            v_y: 0,
            time: 0,
        };

        // Parse start x
        // if input[*i] != b'p' || input[*i + 1] != b'=' {
        //     println!("input[{}]: {:?}", *i, &input[*i..*i + 11]);
        //     unreachable!();
        // }

        *i += 2;
        robot.x += (input[*i] - b'0') as i32;
        *i += 1;
        while input[*i] != b',' {
            robot.x *= 10;
            robot.x += (input[*i] - b'0') as i32;
            *i += 1;
        }

        // Parse start y.
        // if input[*i] != b',' {
        //     unreachable!();
        // }
        *i += 1;

        robot.y += (input[*i] - b'0') as i32;
        *i += 1;
        while input[*i] != b' ' {
            robot.y *= 10;
            robot.y += (input[*i] - b'0') as i32;
            *i += 1;
        }

        // Parse velocity
        // Parse v_x
        // if input[*i] != b' ' || input[*i + 1] != b'v' || input[*i + 2] != b'=' {
        //     unreachable!();
        // }

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
            robot.v_x *= -1;
        }

        // Parse v_y
        // if input[*i] != b',' {
        //     unreachable!();
        // }
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
            robot.v_y *= -1;
        }

        // if *i < input.len() && (input[*i] != b'\n') {
        //     unreachable!();
        // }
        *i += 1;

        return robot;
    }

    fn quadrant<const STEPS: i32, const WIDTH: i32, const HEIGHT: i32>(&self) -> usize {
        let mut x = (self.x + (self.v_x * STEPS)) % WIDTH;
        let mut y = (self.y + (self.v_y * STEPS)) % HEIGHT;
        if x < 0 {
            x += WIDTH;
        }
        if y < 0 {
            y += HEIGHT;
        }
        if x == WIDTH / 2 {
            if y == HEIGHT / 2 {
                return CENTER;
            }
            return X_AXIS;
        }
        if y == HEIGHT / 2 {
            return Y_AXIS;
        }
        let is_left = x < WIDTH / 2;
        if y < HEIGHT / 2 {
            if is_left {
                return UPPER_LEFT;
            }
            return UPPER_RIGHT;
        }
        if is_left {
            return LOWER_LEFT;
        }
        return LOWER_RIGHT;
    }

    fn step_through_time<const WIDTH: i32, const HEIGHT: i32>(&mut self, cur_time: i32) {
        if cur_time == self.time {
            unreachable!();
        }
        let steps = cur_time - self.time;
        self.x = (self.x + (self.v_x * steps)) % WIDTH;
        self.y = (self.y + (self.v_y * steps)) % HEIGHT;
        if self.x < 0 {
            self.x += WIDTH;
        }
        if self.y < 0 {
            self.y += HEIGHT;
        }
        self.time = cur_time;
    }
}

fn get_robots(input: &str) -> Vec<Robot> {
    let input = input.as_bytes();

    let mut robots: Vec<Robot> = Vec::with_capacity(500);

    let mut i = 0;

    while i < input.len() {
        robots.push(Robot::new(input, &mut i));
    }

    return robots;
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u32 {
    let robots = get_robots(input);
    let quadrants = robots
        .iter()
        .map(|robot| robot.quadrant::<100, 101, 103>())
        .fold([0u32; 7], |mut acc, q| {
            acc[q] += 1;
            return acc;
        });
    return quadrants[UPPER_RIGHT]
        * quadrants[UPPER_LEFT]
        * quadrants[LOWER_LEFT]
        * quadrants[LOWER_RIGHT];
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    let mut exists = [0u64; 202]; // index is x, since that is 101, not 103.
    let mut num_steps = 0;
    let mut good: bool = true;
    let mut robots = get_robots(input);

    // check zero steps
    for i in 0..robots.len() {
        let robot = robots[i];
        let index: usize;
        let b: u64;
        if robot.y < 64 {
            index = robot.x as usize;
            b = 1 << robot.y;
        } else {
            index = 101 + robot.x as usize;
            b = 1 << (robot.y - 64);
        }
        if exists[index] & b != 0 {
            good = false;
            break;
        }
        exists[index] |= b;
    }

    if good {
        // print_robots(&exists);
        return num_steps;
    }

    loop {
        num_steps += 1;
        for i in 0..exists.len() {
            exists[i] = 0;
        }

        // it seems like the tree is a picture in the space and none of the robots are on the
        // same square. Hopefully, that's actually true, because that works for my input.
        good = true;
        for i in 0..robots.len() {
            robots[i].step_through_time::<101, 103>(num_steps);
            let robot = robots[i];
            let index: usize;
            let b: u64;
            if robot.y < 64 {
                index = robot.x as usize;
                b = 1 << robot.y;
            } else {
                index = 101 + robot.x as usize;
                b = 1 << (robot.y - 64);
            }
            if exists[index] & b != 0 {
                good = false;
                break;
            }
            exists[index] |= b;
        }

        if good {
            // print_robots(&exists);
            return num_steps;
        }
    }
}

// fn print_robots(exists: &[u64; 202]) {
//     for r in 0..63 {
//         print!("|");
//         for c in 0..101 {
//             if exists[c] & 1 << r == 0 {
//                 print!(" ");
//             } else {
//                 print!("X");
//             }
//         }
//         println!("|");
//     }
//     for r in 64..103 {
//         print!("|");
//         for c in 0..101 {
//             if exists[101 + c] & 1 << (r - 64) == 0 {
//                 print!(" ");
//             } else {
//                 print!("X");
//             }
//         }
//         println!("|");
//     }
// }

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day14.txt";
        fs::read_to_string(input_path).unwrap()
    }

    // fn get_input_2() -> String {
    //     let input_path = "input/2024/day14_2.txt";
    //     fs::read_to_string(input_path).unwrap()
    // }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 224438715);
        // assert_eq!(part1(&get_input_2()), 231221760);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 7603);
        // assert_eq!(part2(&get_input_2()), 6771);
    }
}
