use std::ops::Add;

const GRID_SIZE: usize = 50;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coord {
    row: i8,
    col: i8,
}

impl Coord {
    fn new(r: i8, c: i8) -> Self {
        return Coord { row: r, col: c };
    }

    fn down() -> Self {
        return Coord { row: 1, col: 0 };
    }

    fn up() -> Self {
        return Coord { row: -1, col: 0 };
    }

    fn right() -> Self {
        return Coord { row: 0, col: 1 };
    }

    fn left() -> Self {
        return Coord { row: 0, col: -1 };
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

struct Warehouse {
    // column indexed!
    walls: [u64; GRID_SIZE],
    balls: [u64; GRID_SIZE],
    robot: Coord,

    size: usize,

    instructions: Vec<Coord>,
}

impl Warehouse {
    fn new<const SIZE: usize>(input: &str) -> Self {
        let mut walls: [u64; GRID_SIZE] = [0; GRID_SIZE];
        let mut balls: [u64; GRID_SIZE] = [0; GRID_SIZE];
        let mut robot: Coord = Coord::new(-1, -1);

        walls[0] = 0xFF_FF_FF_FF_FF_FF_FF_FF;
        let mask: u64 = (1 << SIZE - 1) | 1 << 0;
        for c in 1..SIZE - 1 {
            // should be 1 << 50 | 1 << 0
            walls[c] = mask;
        }
        walls[SIZE - 1] = 0xFF_FF_FF_FF_FF_FF_FF_FF;

        // skip first wall line, newline, and first wall.
        let mut i: usize = SIZE + 2;
        let input = input.as_bytes();
        if input[SIZE] != b'\n' {
            unreachable!();
        }
        if input[SIZE + 1] != b'#' {
            unreachable!();
        }

        for r in 1..SIZE - 1 {
            let b = 1u64 << r;
            for c in 1..SIZE - 1 {
                if input[i] == b'.' {
                    // do nothing
                } else if input[i] == b'O' {
                    balls[c] |= b;
                } else if input[i] == b'#' {
                    walls[c] |= b;
                } else if input[i] == b'@' {
                    robot = Coord::new(r as i8, c as i8);
                } else {
                    println!("Input[{i}] = ({r}, {c}) {}", input[i]);
                    unreachable!();
                }
                i += 1;
            }

            // input[i] is a wall, then newline, then wall.
            if input[i] != b'#' {
                println!("Input[{i}] = ({r}, x) {}", input[i]);
                unreachable!();
            }
            if input[i + 1] != b'\n' {
                println!("Input[{i}] = ({r}, x) {}", input[i + 1]);
                unreachable!();
            }
            if input[i + 2] != b'#' {
                println!("Input[{i}] = ({r}, x) {}", input[i + 2]);
                unreachable!();
            }
            i += 3;
        }
        // gotta skip past the last row (minus the first wall), then two newlines.
        i += SIZE + 1;

        if input[i - 1] != b'\n' {
            unreachable!();
        }
        if input[i - 2] != b'\n' {
            unreachable!();
        }

        let mut instructions = Vec::with_capacity(20_000);
        while i < input.len() {
            if input[i] == b'v' {
                instructions.push(Coord::down());
            } else if input[i] == b'^' {
                instructions.push(Coord::up());
            } else if input[i] == b'<' {
                instructions.push(Coord::left());
            } else if input[i] == b'>' {
                instructions.push(Coord::right());
            } else if input[i] != b'\n' {
                unreachable!();
            }
            i += 1;
        }

        return Warehouse {
            walls: walls,
            balls: balls,
            robot: robot,
            size: SIZE,
            instructions: instructions,
        };
    }

    fn print(&self, inst: usize) {
        println!("Inst {inst}");
        for r in 0..self.size {
            for c in 0..self.size {
                let is_wall = self.walls[c] & 1 << r != 0;
                let is_ball = self.balls[c] & 1 << r != 0;
                let is_robot = self.robot.row == r as i8 && self.robot.col == c as i8;
                if is_wall {
                    if is_ball {
                        if is_robot {
                            print!("!")
                        } else {
                            print!("?")
                        }
                    } else {
                        if is_robot {
                            print!("X")
                        } else {
                            print!("#")
                        }
                    }
                } else if is_ball {
                    if is_robot {
                        print!("-")
                    } else {
                        print!("0")
                    }
                } else if is_robot {
                    print!("@")
                } else {
                    print!(" ")
                }
            }
            println!("")
        }
        println!("")
    }

    fn follow_instructions(&mut self) {
        for i in 0..self.instructions.len() {
            // self.print(i);
            let empty: Option<Coord> = self.follow_instruction(self.robot, self.instructions[i]);
            if empty.is_none() {
                continue;
            }
            let clear = self.robot + self.instructions[i];
            let empty = empty.unwrap();
            if clear != empty {
                self.balls[clear.col as usize] &= !(1 << clear.row);
                self.balls[empty.col as usize] |= 1 << empty.row;
            }
            self.robot = self.robot + self.instructions[i];
        }
        // self.print(self.instructions.len());
    }

    fn follow_instruction(&mut self, pos: Coord, delta: Coord) -> Option<Coord> {
        let updated = pos + delta;
        let b = 1u64 << updated.row;
        if self.walls[updated.col as usize] & b == b {
            return None;
        }
        if self.balls[updated.col as usize] & b == 0 {
            return Some(updated);
        }
        return self.follow_instruction(updated, delta);
    }

    fn ball_gps(&self) -> u64 {
        let mut sum: u64 = 0;
        let mut row: u64 = 0;

        for r in 1..self.size {
            row += 100;
            let b = 1u64 << r;
            for c in 1..self.size {
                if self.balls[c] & b != 0 {
                    sum += row + c as u64;
                }
            }
        }

        return sum;
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> u64 {
    return part1_inner::<GRID_SIZE>(input);
}

pub fn part1_inner<const SIZE: usize>(input: &str) -> u64 {
    let mut warehouse = Warehouse::new::<SIZE>(input);
    warehouse.follow_instructions();
    return warehouse.ball_gps();
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> u64 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day15.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_examples() {
        assert_eq!(part1_inner::<10>(example_1()), 10092);
        assert_eq!(part1_inner::<8>(example_2()), 2028);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 1471826);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 0);
    }

    fn example_1() -> &'static str {
        return "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    }

    fn example_2() -> &'static str {
        return "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    }
}
