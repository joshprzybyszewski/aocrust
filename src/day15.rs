use std::{collections::HashSet, ops::Add};

const GRID_SIZE: usize = 50;
const GRID_SIZE_2: usize = 100;

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

struct Warehouse2 {
    // column indexed!
    walls: [u64; GRID_SIZE_2],
    balls: [u64; GRID_SIZE_2],
    robot: Coord,

    num_rows: usize,
    num_cols: usize,

    instructions: Vec<Coord>,
}

impl Warehouse2 {
    fn new<const SIZE: usize>(input: &str) -> Self {
        let mut walls: [u64; GRID_SIZE_2] = [0; GRID_SIZE_2];
        let mut balls: [u64; GRID_SIZE_2] = [0; GRID_SIZE_2];
        let mut robot: Coord = Coord::new(-1, -1);

        let num_rows = SIZE;
        let num_cols = SIZE * 2;

        walls[0] = 0xFF_FF_FF_FF_FF_FF_FF_FF;
        walls[1] = 0xFF_FF_FF_FF_FF_FF_FF_FF;
        let mask: u64 = (1 << num_rows - 1) | 1 << 0;
        for c in 2..num_cols - 2 {
            // should be 1 << 50 | 1 << 0
            walls[c] = mask;
        }
        walls[num_cols - 2] = 0xFF_FF_FF_FF_FF_FF_FF_FF;
        walls[num_cols - 1] = 0xFF_FF_FF_FF_FF_FF_FF_FF;

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
            let mut c: usize = 2;
            for _ in 1..SIZE - 1 {
                if input[i] == b'.' {
                    // do nothing
                } else if input[i] == b'O' {
                    balls[c] |= b;
                } else if input[i] == b'#' {
                    walls[c] |= b;
                    walls[c + 1] |= b;
                } else if input[i] == b'@' {
                    robot = Coord::new(r as i8, c as i8);
                } else {
                    println!("Input[{i}] = ({r}, {c}) {}", input[i]);
                    unreachable!();
                }
                c += 2;
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

        return Warehouse2 {
            walls: walls,
            balls: balls,
            robot: robot,
            num_rows: num_rows,
            num_cols: num_cols,
            instructions: instructions,
        };
    }

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

    fn follow_instructions(&mut self) {
        for i in 0..self.instructions.len() {
            // self.print(i);
            let delta = self.instructions[i];
            let mut to_move: Vec<Coord> = Vec::new();
            if !self.get_boxes_to_move(&mut to_move, self.robot, delta) {
                continue;
            }

            let mut all_new: HashSet<Coord> = HashSet::new();
            for i in 0..to_move.len() {
                let old = to_move[i];
                let new = old + delta;
                all_new.insert(new);
            }
            for old in to_move {
                let new: Coord = old + delta;
                self.balls[new.col as usize] |= 1 << new.row;
                if !all_new.contains(&old) {
                    // only clear out the old if nobody else is moving here.
                    self.balls[old.col as usize] &= !(1 << old.row);
                }
            }
            self.robot = self.robot + self.instructions[i];
        }
        // self.print(self.instructions.len());
    }

    fn get_boxes_to_move(&mut self, to_move: &mut Vec<Coord>, pos: Coord, delta: Coord) -> bool {
        if delta.row == 0 {
            let row = 1u64 << pos.row;
            if delta.col < 0 {
                return self.get_boxes_to_move_looking_left(row, to_move, pos);
            }
            return self.get_boxes_to_move_looking_right(row, to_move, pos);
        }
        return self.get_boxes_to_move_vertical(to_move, pos, delta);
    }

    fn get_boxes_to_move_looking_left(
        &mut self,
        row: u64,
        to_move: &mut Vec<Coord>,
        pos: Coord,
    ) -> bool {
        if self.walls[(pos.col - 1) as usize] & row == row {
            // it's a wall!
            return false;
        }

        if self.balls[(pos.col - 2) as usize] & row == 0 {
            // empty space!
            return true;
        }
        // there's a box to the left. add it to the list.
        let pos = Coord::new(pos.row, pos.col - 2);
        to_move.push(pos);
        return self.get_boxes_to_move_looking_left(row, to_move, pos);
    }

    fn get_boxes_to_move_looking_right(
        &mut self,
        row: u64,
        to_move: &mut Vec<Coord>,
        pos: Coord,
    ) -> bool {
        let c_i: usize = (pos.col + 1) as usize;
        if self.walls[c_i] & row == row {
            // it's a wall!
            return false;
        }

        if self.balls[c_i] & row == 0 {
            // empty space!
            return true;
        }
        // there's a box to the right. add it to the list.
        to_move.push(Coord::new(pos.row, pos.col + 1));
        // add the box's right edge as the leading edge.
        return self.get_boxes_to_move_looking_right(
            row,
            to_move,
            Coord::new(pos.row, pos.col + 2),
        );
    }

    fn get_boxes_to_move_vertical(
        &mut self,
        to_move: &mut Vec<Coord>,
        pos: Coord,
        delta: Coord,
    ) -> bool {
        let wall = pos + delta;
        let b = 1u64 << wall.row;
        if self.walls[wall.col as usize] & b == b {
            return false;
        }

        // check if we're pushing a ball directly (the left side of the ball)
        let direct = pos + delta;
        if self.balls[direct.col as usize] & b == b {
            // check above/below this one!
            to_move.push(direct);
            let lhs = self.get_boxes_to_move_vertical(to_move, direct, delta);
            if !lhs {
                return false;
            }
            return self.get_boxes_to_move_vertical(to_move, direct + Coord::right(), delta);
        }

        // check if we're pushing a ball directly (the left side of the ball)
        let indirect = direct + Coord::left();
        if self.balls[indirect.col as usize] & b == b {
            // check above/below this one!
            to_move.push(indirect);
            let lhs = self.get_boxes_to_move_vertical(to_move, indirect, delta);
            if !lhs {
                return false;
            }
            return self.get_boxes_to_move_vertical(to_move, indirect + Coord::right(), delta);
        }

        return true;
    }

    fn move_ball(&mut self, old: Coord, new: Coord) {
        if self.balls[old.col as usize] & (1 << old.row) == 0 {
            self.print(0);
            println!("old col, row = ({}, {})", new.col, new.row);
            unreachable!();
        }
        self.balls[old.col as usize] &= !(1 << old.row);
        if self.walls[new.col as usize] & (1 << new.row) != 0
            || self.walls[new.col as usize + 1] & (1 << new.row) != 0
        {
            self.print(0);
            println!("new col, row = ({}, {})", new.col, new.row);
            unreachable!();
        }
        self.balls[new.col as usize] |= 1 << new.row;
    }

    fn ball_gps(&self) -> u64 {
        let mut sum: u64 = 0;
        let mut row: u64 = 0;

        let mut num_boxes = 0;
        for r in 1..self.num_rows {
            row += 100;
            let b = 1u64 << r;
            for c in 1..self.num_cols {
                if self.balls[c] & b != 0 {
                    sum += row + c as u64;
                    num_boxes += 1;
                }
            }
        }
        if num_boxes != 598 {
            unreachable!();
        }

        return sum;
    }
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> u64 {
    return part2_inner::<GRID_SIZE>(input);
}

pub fn part2_inner<const SIZE: usize>(input: &str) -> u64 {
    let mut warehouse = Warehouse2::new::<SIZE>(input);
    warehouse.follow_instructions();
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
        assert_eq!(part1_inner::<10>(example_1()), 10092);
        assert_eq!(part1_inner::<8>(example_2()), 2028);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2_inner::<7>(example_4()), 104 + 106 + 205);
        assert_eq!(part2_inner::<7>(example_5()), 102 + 104);
        assert_eq!(part2_inner::<7>(example_5b()), 108 + 110);
        assert_eq!(part2_inner::<7>(example_6()), 0);
        // assert_eq!(part2_inner::<7>(example_3()), 105 + 207 + 306);
        // assert_eq!(part2_inner::<10>(example_1()), 9021);
        // assert_eq!(part2_inner::<8>(example_2()), -1);
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

    fn example_3() -> &'static str {
        return "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
    }

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

    fn example_6() -> &'static str {
        return "#######
#.....#
#..O@.#
#.OO..#
#.....#
#.....#
#######

<^^<vvvvv";
    }
}
