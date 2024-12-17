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

        // skip past "\nRegister B: "
        i += 13;
        cpu.register_b += (input[i] - b'0') as i64;
        i += 1;
        while input[i] != b'\n' {
            cpu.register_b *= 10;
            cpu.register_b += (input[i] - b'0') as i64;
            i += 1;
        }

        // skip past "\nRegister C: "
        i += 13;
        cpu.register_c += (input[i] - b'0') as i64;
        i += 1;
        while input[i] != b'\n' {
            cpu.register_c *= 10;
            cpu.register_c += (input[i] - b'0') as i64;
            i += 1;
        }

        // skip past "\n\nProgram: "
        i += 11;

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

    fn run(&mut self) -> Vec<u8> {
        let mut output = Vec::with_capacity(32);

        loop {
            if self.pc + 1 >= self.program.len() {
                break;
            }

            let op_code = self.program[self.pc];
            let operand = self.program[self.pc + 1];
            self.pc += 2;

            let literal_operand: i64 = operand as i64;

            match op_code {
                0 => {
                    // adv
                    let numerator = self.register_a;
                    let denominator = 1 << self.combo_operand(operand);
                    self.register_a = numerator / denominator;
                }
                1 => {
                    // bxl
                    self.register_b ^= literal_operand;
                }
                2 => {
                    // bst
                    self.register_b = self.combo_operand(operand) & 0x07;
                }
                3 => {
                    // jnz
                    if self.register_a != 0 {
                        self.pc = operand as usize;
                    }
                }
                4 => {
                    // bxc
                    self.register_b ^= self.register_c;
                }
                5 => {
                    // out
                    output.push((self.combo_operand(operand) as u8) & 0x07)
                }
                6 => {
                    // bdv
                    let numerator = self.register_a;
                    let denominator = 1 << self.combo_operand(operand);
                    self.register_b = numerator / denominator;
                }
                7 => {
                    // cdv
                    let numerator = self.register_a;
                    let denominator = 1 << self.combo_operand(operand);
                    self.register_c = numerator / denominator;
                }
                _ => unreachable!(),
            }
        }

        return output;
    }

    fn combo_operand(&self, operand: u8) -> i64 {
        if operand == 7 {
            unreachable!();
        } else if operand == 6 {
            return self.register_c;
        } else if operand == 5 {
            return self.register_b;
        } else if operand == 4 {
            return self.register_a;
        } else {
            return operand as i64;
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let mut cpu = CPU_1::new(input);
    let output = cpu.run();
    return output
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",");
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> u64 {
    let mut cpu = CPU_1::new(input);
    let output = cpu.run();

    return 0;
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
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), 117440);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), "2,1,0,1,7,2,5,0,3");
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), "0");
    }
}
