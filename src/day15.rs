use std::ops::Add;

const GRID_SIZE: usize = 64;
const GRID_SIZE_2: usize = GRID_SIZE * 2;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
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
}

impl Warehouse {
    fn new(input: &str) -> Self {
        let mut walls: [u64; GRID_SIZE] = [0; GRID_SIZE];
        let mut balls: [u64; GRID_SIZE] = [0; GRID_SIZE];
        let mut robot: Coord = Coord::new(-1, -1);

        let input = input.as_bytes();
        let mut size: usize = 50;
        if input[size] != b'\n' {
            size = 64;
            for i in 0..64 {
                if input[i] == b'\n' {
                    size = i;
                    break;
                }
            }
            if size == 64 {
                unreachable!();
            }
        }

        walls[0] = 0xFF_FF_FF_FF_FF_FF_FF_FF;
        let mask: u64 = (1 << size - 1) | 1 << 0;
        for c in 1..size - 1 {
            walls[c] = mask;
        }
        walls[size - 1] = 0xFF_FF_FF_FF_FF_FF_FF_FF;

        // skip first wall line, newline, and first wall.
        let mut i: usize = size + 2;
        // if input[SIZE] != b'\n' {
        //     unreachable!();
        // }
        // if input[SIZE + 1] != b'#' {
        //     unreachable!();
        // }

        for r in 1..size - 1 {
            let b = 1u64 << r;
            for c in 1..size - 1 {
                if input[i] == b'.' {
                    // do nothing
                } else if input[i] == b'O' {
                    balls[c] |= b;
                } else if input[i] == b'#' {
                    walls[c] |= b;
                } else if input[i] == b'@' {
                    robot = Coord::new(r as i8, c as i8);
                    // } else {
                    //     println!("Input[{i}] = ({r}, {c}) {}", input[i]);
                    //     unreachable!();
                }
                i += 1;
            }

            // input[i] is a wall, then newline, then wall.
            // if input[i] != b'#' {
            //     println!("Input[{i}] = ({r}, x) {}", input[i]);
            //     unreachable!();
            // }
            // if input[i + 1] != b'\n' {
            //     println!("Input[{i}] = ({r}, x) {}", input[i + 1]);
            //     unreachable!();
            // }
            // if input[i + 2] != b'#' {
            //     println!("Input[{i}] = ({r}, x) {}", input[i + 2]);
            //     unreachable!();
            // }
            i += 3;
        }
        // gotta skip past the last row (minus the first wall), then two newlines.
        i += size + 1;

        // if input[i - 1] != b'\n' {
        //     unreachable!();
        // }
        // if input[i - 2] != b'\n' {
        //     unreachable!();
        // }

        let mut w = Warehouse {
            walls: walls,
            balls: balls,
            robot: robot,
            size,
        };

        while i < input.len() {
            if input[i] == b'v' {
                follow_instructions_1(&mut w, Coord::down());
            } else if input[i] == b'^' {
                follow_instructions_1(&mut w, Coord::up());
            } else if input[i] == b'<' {
                follow_instructions_1(&mut w, Coord::left());
            } else if input[i] == b'>' {
                follow_instructions_1(&mut w, Coord::right());
                // } else if input[i] != b'\n' {
                //     unreachable!();
            }
            i += 1;
        }

        return w;
    }

    fn ball_gps(&self) -> u64 {
        let mut sum: u64 = 0;
        let mut row: u64 = 0;

        let mut row_bit = 1u64 << 1;
        for _ in 1..self.size {
            row += 100;
            for c in 1..self.size {
                if self.balls[c] & row_bit != 0 {
                    sum += row + c as u64;
                }
            }
            row_bit <<= 1;
        }

        return sum;
    }
}

fn follow_instructions_1(w: &mut Warehouse, delta: Coord) {
    let empty: Option<Coord> = follow_instruction_1(w, w.robot, delta);
    if empty.is_none() {
        return;
    }
    let clear = w.robot + delta;
    let empty = empty.unwrap();
    if clear != empty {
        w.balls[clear.col as usize] &= !(1 << clear.row);
        w.balls[empty.col as usize] |= 1 << empty.row;
    }
    w.robot = clear;
}

fn follow_instruction_1(w: &mut Warehouse, pos: Coord, delta: Coord) -> Option<Coord> {
    let updated = pos + delta;
    let b = 1u64 << updated.row;
    if w.walls[updated.col as usize] & b == b {
        return None;
    }
    if w.balls[updated.col as usize] & b == 0 {
        return Some(updated);
    }
    return follow_instruction_1(w, updated, delta);
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> u64 {
    let warehouse = Warehouse::new(input);
    return warehouse.ball_gps();
}

struct Warehouse2 {
    // column indexed!
    walls: [u64; GRID_SIZE_2],
    balls: [u64; GRID_SIZE_2],
    robot: Coord,

    num_rows: usize,
    num_cols: usize,
}

impl Warehouse2 {
    fn new(input: &str) -> Self {
        let mut walls: [u64; GRID_SIZE_2] = [0; GRID_SIZE_2];
        let mut balls: [u64; GRID_SIZE_2] = [0; GRID_SIZE_2];
        let mut robot: Coord = Coord::new(-1, -1);

        let input = input.as_bytes();
        let mut size: usize = 50;
        if input[size] != b'\n' {
            size = 64;
            for i in 0..64 {
                if input[i] == b'\n' {
                    size = i;
                    break;
                }
            }
            if size == 64 {
                unreachable!();
            }
        }

        let num_rows = size;
        let num_cols = size * 2;

        walls[0] = 0xFF_FF_FF_FF_FF_FF_FF_FF;
        walls[1] = 0xFF_FF_FF_FF_FF_FF_FF_FF;
        let mask: u64 = (1 << num_rows - 1) | 1 << 0;
        for c in 2..num_cols - 2 {
            walls[c] = mask;
        }
        walls[num_cols - 2] = 0xFF_FF_FF_FF_FF_FF_FF_FF;
        walls[num_cols - 1] = 0xFF_FF_FF_FF_FF_FF_FF_FF;

        // skip first wall line, newline, and first wall.
        let mut i: usize = size + 2;
        // if input[SIZE] != b'\n' {
        //     unreachable!();
        // }
        // if input[SIZE + 1] != b'#' {
        //     unreachable!();
        // }

        for r in 1..size - 1 {
            let b = 1u64 << r;
            let mut c: usize = 2;
            for _ in 1..size - 1 {
                if input[i] == b'.' {
                    // do nothing
                } else if input[i] == b'O' {
                    balls[c] |= b;
                } else if input[i] == b'#' {
                    walls[c] |= b;
                    walls[c + 1] |= b;
                } else if input[i] == b'@' {
                    robot = Coord::new(r as i8, c as i8);
                    // } else {
                    //     println!("Input[{i}] = ({r}, {c}) {}", input[i]);
                    //     unreachable!();
                }
                c += 2;
                i += 1;
            }

            // input[i] is a wall, then newline, then wall.
            // if input[i] != b'#' {
            //     println!("Input[{i}] = ({r}, x) {}", input[i]);
            //     unreachable!();
            // }
            // if input[i + 1] != b'\n' {
            //     println!("Input[{i}] = ({r}, x) {}", input[i + 1]);
            //     unreachable!();
            // }
            // if input[i + 2] != b'#' {
            //     println!("Input[{i}] = ({r}, x) {}", input[i + 2]);
            //     unreachable!();
            // }
            i += 3;
        }
        // gotta skip past the last row (minus the first wall), then two newlines.
        i += size + 1;

        // if input[i - 1] != b'\n' {
        //     unreachable!();
        // }
        // if input[i - 2] != b'\n' {
        //     unreachable!();
        // }

        let mut warehouse = Warehouse2 {
            walls: walls,
            balls: balls,
            robot: robot,
            num_rows: num_rows,
            num_cols: num_cols,
        };

        let mut to_move: Vec<Coord> = Vec::with_capacity(500);
        while i < input.len() {
            if input[i] == b'v' {
                do_instruction(&mut warehouse, &mut to_move, Coord::down());
            } else if input[i] == b'^' {
                do_instruction(&mut warehouse, &mut to_move, Coord::up());
            } else if input[i] == b'<' {
                do_instruction(&mut warehouse, &mut to_move, Coord::left());
            } else if input[i] == b'>' {
                do_instruction(&mut warehouse, &mut to_move, Coord::right());
                // } else if input[i] != b'\n' {
                //     unreachable!();
            }
            i += 1;
        }

        return warehouse;
    }

    fn ball_gps(&self) -> u64 {
        let mut sum: u64 = 0;
        let mut row: u64 = 0;
        let mut row_bit = 1u64 << 1;

        for _ in 1..self.num_rows {
            row += 100;
            for c in 1..self.num_cols {
                if self.balls[c] & row_bit != 0 {
                    sum += row + c as u64;
                }
            }
            row_bit <<= 1
        }

        return sum;
    }
}

fn do_instruction(w: &mut Warehouse2, to_move: &mut Vec<Coord>, delta: Coord) {
    // w.print(i);
    to_move.clear();
    if !get_boxes_to_move(w, to_move, w.robot, delta) {
        return;
    }

    for old in to_move.iter().rev() {
        let new = *old + delta;
        w.balls[new.col as usize] |= 1 << new.row;
        w.balls[old.col as usize] &= !(1 << old.row);
    }
    w.robot = w.robot + delta;
    // w.print(w.instructions.len());
}

fn get_boxes_to_move(
    w: &mut Warehouse2,
    to_move: &mut Vec<Coord>,
    pos: Coord,
    delta: Coord,
) -> bool {
    if delta.row == 0 {
        let row = 1u64 << pos.row;
        if delta.col < 0 {
            return get_boxes_to_move_looking_left(w, row, to_move, pos);
        }
        return get_boxes_to_move_looking_right(w, row, to_move, pos);
    }
    return get_boxes_to_move_vertical(w, to_move, pos, delta);
}

fn get_boxes_to_move_looking_left(
    w: &mut Warehouse2,
    row: u64,
    to_move: &mut Vec<Coord>,
    pos: Coord,
) -> bool {
    if w.walls[(pos.col - 1) as usize] & row == row {
        // it's a wall!
        return false;
    }

    if w.balls[(pos.col - 2) as usize] & row == 0 {
        // empty space!
        return true;
    }
    // there's a box to the left. add it to the list.
    let pos = Coord::new(pos.row, pos.col - 2);
    to_move.push(pos);
    return get_boxes_to_move_looking_left(w, row, to_move, pos);
}

fn get_boxes_to_move_looking_right(
    w: &mut Warehouse2,
    row: u64,
    to_move: &mut Vec<Coord>,
    pos: Coord,
) -> bool {
    let c_i: usize = (pos.col + 1) as usize;
    if w.walls[c_i] & row == row {
        // it's a wall!
        return false;
    }

    if w.balls[c_i] & row == 0 {
        // empty space!
        return true;
    }
    // there's a box to the right. add it to the list.
    to_move.push(Coord::new(pos.row, pos.col + 1));
    // add the box's right edge as the leading edge.
    return get_boxes_to_move_looking_right(w, row, to_move, Coord::new(pos.row, pos.col + 2));
}

fn get_boxes_to_move_vertical(
    w: &mut Warehouse2,
    to_move: &mut Vec<Coord>,
    pos: Coord,
    delta: Coord,
) -> bool {
    let wall = pos + delta;
    let b = 1u64 << wall.row;
    if w.walls[wall.col as usize] & b == b {
        return false;
    }

    // check if we're pushing a ball directly (the left side of the ball)
    let direct = pos + delta;
    if w.balls[direct.col as usize] & b == b {
        // check above/below this one!
        to_move.push(direct);
        let lhs = get_boxes_to_move_vertical(w, to_move, direct, delta);
        if !lhs {
            return false;
        }
        return get_boxes_to_move_vertical(w, to_move, direct + Coord::right(), delta);
    }

    // check if we're pushing a ball directly (the left side of the ball)
    let indirect = direct + Coord::left();
    if w.balls[indirect.col as usize] & b == b {
        // check above/below this one!
        to_move.push(indirect);
        let lhs = get_boxes_to_move_vertical(w, to_move, indirect, delta);
        if !lhs {
            return false;
        }
        return get_boxes_to_move_vertical(w, to_move, indirect + Coord::right(), delta);
    }

    return true;
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> u64 {
    let warehouse = Warehouse2::new(input);
    return warehouse.ball_gps();
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
        assert_eq!(part1(example_1()), 10092);
        assert_eq!(part1(example_2()), 2028);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(example_4()), 104 + 106 + 205);
        assert_eq!(part2(example_5()), 102 + 104);
        assert_eq!(part2(example_5b()), 108 + 110);
        // assert_eq!(part2(example_6()), 0);
        // assert_eq!(part2(example_3()), 105 + 207 + 306);
        // assert_eq!(part2(example_1()), 9021);
        // assert_eq!(part2(example_2()), -1);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 1471826);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 1457703);
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

    //     fn example_3() -> &'static str {
    //         return "#######
    // #...#.#
    // #.....#
    // #..OO@#
    // #..O..#
    // #.....#
    // #######

    // <vv<<^^<<^^";
    //     }

    fn example_4() -> &'static str {
        return "#######
#.....#
#.....#
#.OO..#
#..O@.#
#.....#
#######

<vv<^^^^^";
    }

    fn example_5() -> &'static str {
        return "#######
#.OO@.#
#.....#
#.....#
#.....#
#.....#
#######

<<<<<";
    }

    fn example_5b() -> &'static str {
        return "#######
#.@OO.#
#.....#
#.....#
#.....#
#.....#
#######

>>>>>>";
    }

    //     fn example_6() -> &'static str {
    //         return "#######
    // #.....#
    // #..O@.#
    // #.OO..#
    // #.....#
    // #.....#
    // #######

    // <^^<vvvvv";
    //     }
}

/*
fn print(&self, inst: usize) {
    println!("Inst {inst}");
    for r in 0..self.num_rows {
        for c in 0..self.num_cols {
            print!("{}", self.get_string_for_spot(r, c));
        }
        println!("")
    }
    println!("")
}

fn get_string_for_spot(&self, r: usize, c: usize) -> &str {
    let is_wall = self.walls[c] & 1 << r != 0;
    let is_ball = self.balls[c] & 1 << r != 0;
    let is_prev_ball = c > 0 && self.balls[c - 1] & 1 << r != 0;
    let is_robot = self.robot.row == r as i8 && self.robot.col == c as i8;
    if is_wall {
        if is_ball {
            if is_robot {
                if is_prev_ball {
                    unreachable!();
                    return "a";
                }
                unreachable!();
                return "!";
            }
            if is_prev_ball {
                unreachable!();
                return "b";
            }
            unreachable!();
            return "?";
        }

        if is_robot {
            if is_prev_ball {
                unreachable!();
                return "c";
            }
            unreachable!();
            return "X";
        }

        if is_prev_ball {
            unreachable!();
            return "d";
        }

        return "#";
    }

    if is_ball {
        if is_robot {
            if is_prev_ball {
                unreachable!();
                return "e";
            }
            unreachable!();
            return "-";
        }
        if is_prev_ball {
            unreachable!();
            return "f";
        }
        return "[";
    }

    if is_robot {
        if is_prev_ball {
            unreachable!();
            return "g";
        }
        return "@";
    }

    if is_prev_ball {
        return "]";
    }

    return ".";
}
 */
