use rayon::prelude::*;

const UNIT_CONVERSION_ERROR: i64 = 10_000_000_000_000;

#[derive(Copy, Clone, Debug)]
struct Machine {
    a_x: i64,
    a_y: i64,

    b_x: i64,
    b_y: i64,

    p_x: i64,
    p_y: i64,
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
        machine.a_x += (input[*i] - b'0') as i64;
        *i += 1;
        while input[*i] != b',' {
            machine.a_x *= 10;
            machine.a_x += (input[*i] - b'0') as i64;
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

        machine.a_y += (input[*i] - b'0') as i64;
        *i += 1;
        while input[*i] != b'\n' {
            machine.a_y *= 10;
            machine.a_y += (input[*i] - b'0') as i64;
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
        machine.b_x += (input[*i] - b'0') as i64;
        *i += 1;
        while input[*i] != b',' {
            machine.b_x *= 10;
            machine.b_x += (input[*i] - b'0') as i64;
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

        machine.b_y += (input[*i] - b'0') as i64;
        *i += 1;
        while input[*i] != b'\n' {
            machine.b_y *= 10;
            machine.b_y += (input[*i] - b'0') as i64;
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
        machine.p_x += (input[*i] - b'0') as i64;
        *i += 1;
        while input[*i] != b',' {
            machine.p_x *= 10;
            machine.p_x += (input[*i] - b'0') as i64;
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

        machine.p_y += (input[*i] - b'0') as i64;
        *i += 1;
        while *i < input.len() && input[*i] != b'\n' {
            machine.p_y *= 10;
            machine.p_y += (input[*i] - b'0') as i64;
            *i += 1;
        }

        if *i < input.len() - 1 && (input[*i] != b'\n' || input[*i + 1] != b'\n') {
            unreachable!();
        }
        *i += 2;

        return machine;
    }

    fn cost_p1(&self) -> i64 {
        return self.cost::<100>(self.p_x, self.p_y);
    }

    fn cost_p2(&self) -> i64 {
        return self.cost::<{ 1 << 62 }>(
            self.p_x + UNIT_CONVERSION_ERROR,
            self.p_y + UNIT_CONVERSION_ERROR,
        );
    }

    // where's my linear algebra textbook?
    fn cost<const PRESS_LIMIT: i64>(&self, p_x: i64, p_y: i64) -> i64 {
        // A costs 3 tokens.
        // B costs 1 token.

        // num_a * a_x + num_b * b_x                     = p_x
        // num_a * a_y + num_b * b_y                     = p_y
        //
        // 0           + num_b * (b_y * a_x - b_x * a_y) = p_y * a_x - p_x * a_y
        // num_b = (a_x * p_y - a_y * p_x) / (a_x * b_y  - a_y * b_x)

        let numerator_b = self.a_x * p_y - self.a_y * p_x;
        let denominator_b = self.a_x * self.b_y - self.a_y * self.b_x;
        if numerator_b % denominator_b != 0 {
            return 0;
        }

        let num_b = numerator_b / denominator_b;
        if num_b > PRESS_LIMIT {
            return 0;
        }

        let x_diff = p_x - (num_b * self.b_x);
        if x_diff % self.a_x != 0 {
            return 0;
        }
        let num_a: i64 = x_diff / self.a_x;
        if num_a > PRESS_LIMIT {
            return 0;
        }
        if num_a * self.a_y + num_b * self.b_y != p_y {
            // triple check.
            return 0;
        }

        return num_a * 3 + num_b;
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
pub fn part1(input: &str) -> i64 {
    let machines = get_machines(input);
    return machines.par_iter().map(|machine| machine.cost_p1()).sum();
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let machines = get_machines(input);
    return machines.par_iter().map(|machine| machine.cost_p2()).sum();
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
        assert_eq!(part2(&get_input()), 90798500745591)
    }
}
