struct CPU_1 {
    register_a: i64,
    register_b: i64,
    register_c: i64,

    pc: usize,

    program: Vec<u8>,
}

impl CPU_1 {
    fn new(input: &str) -> Self {
        let input = input.as_bytes();
        let mut cpu = CPU_1 {
            register_a: 0,
            register_b: 0,
            register_c: 0,
            pc: 0,
            program: Vec::with_capacity(32),
        };

        // skip past "Register A: "
        let mut i: usize = 12;
        cpu.register_a += (input[i] - b'0') as i64;
        i += 1;
        while input[i] != b'\n' {
            cpu.register_a *= 10;
            cpu.register_a += (input[i] - b'0') as i64;
            i += 1;
        }

        // skip past "Register B: "
        i += 12;
        cpu.register_b += (input[i] - b'0') as i64;
        i += 1;
        while input[i] != b'\n' {
            cpu.register_b *= 10;
            cpu.register_b += (input[i] - b'0') as i64;
            i += 1;
        }

        // skip past "Register C: "
        i += 12;
        cpu.register_c += (input[i] - b'0') as i64;
        i += 1;
        while input[i] != b'\n' {
            cpu.register_c *= 10;
            cpu.register_c += (input[i] - b'0') as i64;
            i += 1;
        }

        // skip past "\nProgram: "
        i += 10;

        loop {
            cpu.program.push(input[i] - b'0');
            i += 1;
            if i >= input.len() || input[i] != b',' {
                break;
            }
            i += 1;
        }

        return cpu;
    }

    fn run(&mut self) -> Vec<u64> {
        let output = Vec::with_capacity(32);

        loop {
            let op_code = self.program[self.pc];
            let operand = self.program[self.pc + 1];
            let combo_operand: i64;
            if operand == 7 {
                unreachable!();
            } else if operand == 6 {
                combo_operand = self.register_c
            } else if operand == 5 {
                combo_operand = self.register_b
            } else if operand == 4 {
                combo_operand = self.register_a
            } else {
                combo_operand = operand as i64
            }
            match op_code {
                0 => {
                    // adv
                    let numerator = self.register_a;
                    let denominator = 2 << combo_operand;
                    self.register_a = numerator / denominator;
                }
                1 => {
                    // bxl
                }
                2 => {
                    // bst
                }
                3 => {
                    // jnz
                }
                4 => {
                    // bxc
                }
                5 => {
                    // out
                }
                6 => {
                    // bdv
                    let numerator = self.register_a;
                    let denominator = 2 << combo_operand;
                    self.register_b = numerator / denominator;
                }
                7 => {
                    // cdv
                    let numerator = self.register_a;
                    let denominator = 2 << combo_operand;
                    self.register_c = numerator / denominator;
                }
                _ => unreachable!(),
            }
        }

        return output;
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let mut cpu = CPU_1::new(input);
    let output = cpu.run();
    return String::new(); // output.iter().join(",");
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> String {
    return String::new();
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day17.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_example_input() -> &'static str {
        return "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), "0");
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), "0");
    }
}
