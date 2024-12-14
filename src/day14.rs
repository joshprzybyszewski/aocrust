use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
struct Robot {
    x: i32,
    y: i32,

    v_x: i32,
    v_y: i32,

    speed: i32,

    time: i32,
}

#[inline(always)]
fn new_robot(input: &[u8], i: &mut usize, robot: &mut Robot) {
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

    robot.speed = robot.v_x * robot.v_x + robot.v_y * robot.v_y;
}

impl Robot {
    fn step_through_time<const WIDTH: i32, const HEIGHT: i32>(&mut self, cur_time: i32) {
        // if cur_time == self.time {
        //     unreachable!();
        // }
        let steps = cur_time - self.time;
        self.x = (self.x + (self.v_x * steps)).rem_euclid(WIDTH);
        self.y = (self.y + (self.v_y * steps)).rem_euclid(HEIGHT);
        if self.x < 0 {
            self.x += WIDTH;
        }
        if self.y < 0 {
            self.y += HEIGHT;
        }
        self.time = cur_time;
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u32 {
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
        x = (x + (v_x * 100)) % 101;
        y = (y + (v_y * 100)) % 103;
        if x < 0 {
            x += 101;
        }
        if y < 0 {
            y += 103;
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
    let mut exists = [0u64; 202]; // index is x, since that is 101, not 103.
    let mut num_steps = 0;
    let mut good: bool = true;

    let input = input.as_bytes();

    let mut robots = [Robot {
        x: 0,
        y: 0,
        v_x: 0,
        v_y: 0,
        time: 0,
        speed: 0,
    }; 500];

    let mut i = 0;
    let mut r_i = 0;

    while i < input.len() {
        new_robot(input, &mut i, &mut robots[r_i]);
        r_i += 1;
    }

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

    robots.sort_by(|a: &Robot, b: &Robot| {
        if a.speed < b.speed {
            return Ordering::Less;
        }
        if a.speed == b.speed {
            return Ordering::Equal;
        }
        return Ordering::Greater;
    });

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
