use std::cmp::max;

#[derive(Copy, Clone, Debug)]
struct Machine {
    a_x: u32,
    a_y: u32,

    b_x: u32,
    b_y: u32,

    p_x: u32,
    p_y: u32,
}

impl Machine {
    fn new(input: &[u8], i: &mut usize) -> Self {
        let mut machine = Machine {
            a_x: 0,
            a_y: 0,
            b_x: 0,
            b_y: 0,
            p_x: 0,
            p_y: 0,
        };

        // Parse Button A
        // Parse a_x.
        if input[*i] != b'B'
            || input[*i + 1] != b'u'
            || input[*i + 2] != b't'
            || input[*i + 3] != b't'
            || input[*i + 4] != b'o'
            || input[*i + 5] != b'n'
            || input[*i + 6] != b' '
            || input[*i + 7] != b'A'
            || input[*i + 8] != b':'
            || input[*i + 9] != b' '
            || input[*i + 10] != b'X'
            || input[*i + 11] != b'+'
        {
            println!("input[{}]: {:?}", *i, &input[*i..*i + 11]);
            unreachable!();
        }

        *i += 12;
        machine.a_x += (input[*i] - b'0') as u32;
        *i += 1;
        while input[*i] != b',' {
            machine.a_x *= 10;
            machine.a_x += (input[*i] - b'0') as u32;
            *i += 1;
        }

        // Parse a_y.
        if input[*i] != b','
            || input[*i + 1] != b' '
            || input[*i + 2] != b'Y'
            || input[*i + 3] != b'+'
        {
            unreachable!();
        }
        *i += 4;

        machine.a_y += (input[*i] - b'0') as u32;
        *i += 1;
        while input[*i] != b'\n' {
            machine.a_y *= 10;
            machine.a_y += (input[*i] - b'0') as u32;
            *i += 1;
        }

        // Parse Button B
        // Parse b_x
        if input[*i] != b'\n'
            || input[*i + 1] != b'B'
            || input[*i + 2] != b'u'
            || input[*i + 3] != b't'
            || input[*i + 4] != b't'
            || input[*i + 5] != b'o'
            || input[*i + 6] != b'n'
            || input[*i + 7] != b' '
            || input[*i + 8] != b'B'
            || input[*i + 9] != b':'
            || input[*i + 10] != b' '
            || input[*i + 11] != b'X'
            || input[*i + 12] != b'+'
        {
            unreachable!();
        }

        *i += 13;
        machine.b_x += (input[*i] - b'0') as u32;
        *i += 1;
        while input[*i] != b',' {
            machine.b_x *= 10;
            machine.b_x += (input[*i] - b'0') as u32;
            *i += 1;
        }

        // Parse b_y
        if input[*i] != b','
            || input[*i + 1] != b' '
            || input[*i + 2] != b'Y'
            || input[*i + 3] != b'+'
        {
            unreachable!();
        }
        *i += 4;

        machine.b_y += (input[*i] - b'0') as u32;
        *i += 1;
        while input[*i] != b'\n' {
            machine.b_y *= 10;
            machine.b_y += (input[*i] - b'0') as u32;
            *i += 1;
        }

        // Parse the Prize
        // Parse p_x
        if input[*i] != b'\n'
            || input[*i + 1] != b'P'
            || input[*i + 2] != b'r'
            || input[*i + 3] != b'i'
            || input[*i + 4] != b'z'
            || input[*i + 5] != b'e'
            || input[*i + 6] != b':'
            || input[*i + 7] != b' '
            || input[*i + 8] != b'X'
            || input[*i + 9] != b'='
        {
            unreachable!();
        }

        *i += 10;
        machine.p_x += (input[*i] - b'0') as u32;
        *i += 1;
        while input[*i] != b',' {
            machine.p_x *= 10;
            machine.p_x += (input[*i] - b'0') as u32;
            *i += 1;
        }

        // Parse p_y
        if input[*i] != b','
            || input[*i + 1] != b' '
            || input[*i + 2] != b'Y'
            || input[*i + 3] != b'='
        {
            unreachable!();
        }
        *i += 4;

        machine.p_y += (input[*i] - b'0') as u32;
        *i += 1;
        while *i < input.len() && input[*i] != b'\n' {
            machine.p_y *= 10;
            machine.p_y += (input[*i] - b'0') as u32;
            *i += 1;
        }

        if *i < input.len() - 1 && (input[*i] != b'\n' || input[*i + 1] != b'\n') {
            unreachable!();
        }
        *i += 2;

        return machine;
    }

    fn cost_p1(&self) -> u32 {
        // A costs 3 tokens.
        // B costs 1 token.

        let tokens = 0;

        let max_b = max(100, max(self.p_x / self.b_x, self.p_y / self.b_y));
        // let maxA = max(100, max(self.p_x / self.a_x, self.p_y / self.a_y));

        let mut x = self.b_x * max_b;
        let mut y = self.b_y * max_b;

        for b in (0..max_b).rev() {
            if self.p_x >= x && self.p_y >= y {
                let x_diff = self.p_x - x;
                let y_diff = self.p_y - y;
                if x_diff % self.a_x == 0 && y_diff % self.a_y == 0 {
                    let num_b = b;
                    let num_a = x_diff / self.a_x;
                    if num_a == y_diff / self.a_y {
                        return num_b + 1 + num_a * 3;
                        // println!("machine    = {:?}", self);
                        // println!("num_b    = {num_b}");
                        // println!("x_diff   = {x_diff}");
                        // println!("self.a_x = {}", self.a_x);
                        // println!("y_diff   = {y_diff}");
                        // println!("self.a_y = {}", self.a_y);
                        // println!("num_a    = {num_a}");
                        // unreachable!();
                    }
                }
            }
            x -= self.b_x;
            y -= self.b_y;
        }

        return 0;
    }
}

fn get_machines(input: &str) -> Vec<Machine> {
    let input = input.as_bytes();

    let mut machines: Vec<Machine> = Vec::with_capacity(320);

    let mut i = 0;

    while i < input.len() {
        machines.push(Machine::new(input, &mut i));
    }

    return machines;
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> u32 {
    let machines = get_machines(input);
    return machines.iter().map(|machine| machine.cost_p1()).sum();
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> u64 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_example_input() -> String {
        let input_path = "input/2024/examples/day13.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_input() -> String {
        let input_path = "input/2024/day13.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 480)
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 31761)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 0)
    }
}
