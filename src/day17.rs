#[derive(Copy, Clone)]
struct Program {
    // might only need 16, but 32 is fine.
    instructions: [u8; 32],

    num_instructions: usize,
}

#[derive(Copy, Clone)]
struct CPU {
    register_a: i64,
    register_b: i64,
    register_c: i64,

    pc: usize,
}

impl CPU {
    #[inline(always)]
    fn run(&mut self, program: &Program) -> Vec<u8> {
        let mut output = Vec::with_capacity(32);

        loop {
            if self.pc + 1 >= program.num_instructions {
                break;
            }

            let op_code = program.instructions[self.pc];
            let operand = program.instructions[self.pc + 1];
            self.pc += 2;

            let literal_operand: i64 = operand as i64;

            match op_code {
                0 => {
                    // adv
                    self.register_a >>= self.combo_operand(operand);
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
                    let next = (self.combo_operand(operand) as u8) & 0x07;
                    output.push(next);
                }
                6 => {
                    // bdv
                    self.register_b = self.register_a >> self.combo_operand(operand);
                }
                7 => {
                    // cdv
                    self.register_c = self.register_a >> self.combo_operand(operand);
                }
                _ => unreachable!(),
            }
        }

        return output;
    }

    #[inline(always)]
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

#[inline(always)]
fn parse_input(input: &str) -> (CPU, Program) {
    let input = input.as_bytes();
    let mut cpu = CPU {
        register_a: 0,
        register_b: 0,
        register_c: 0,
        pc: 0,
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

    let mut program = Program {
        instructions: [0; 32],
        num_instructions: 0,
    };

    loop {
        program.instructions[program.num_instructions] = (input[i] - b'0');
        program.num_instructions += 1;
        i += 1;
        if i >= input.len() || input[i] != b',' {
            break;
        }
        i += 1;
    }

    return (cpu, program);
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let (mut cpu, program) = parse_input(input);
    let output = cpu.run(&program);
    return output
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",");
}

#[inline(always)]
fn parse_input_2(input: &str) -> Program {
    let input = input.as_bytes();

    // skip past "Register A: "
    let mut i: usize = 13;
    while input[i] != b'\n' {
        i += 1;
    }

    // skip past "\nRegister B: "
    i += 14;
    while input[i] != b'\n' {
        i += 1;
    }

    // skip past "\nRegister C: "
    i += 14;
    while input[i] != b'\n' {
        i += 1;
    }

    // skip past "\n\nProgram: "
    i += 11;

    let mut program = Program {
        instructions: [0; 32],
        num_instructions: 0,
    };

    loop {
        program.instructions[program.num_instructions] = (input[i] - b'0');
        program.num_instructions += 1;
        i += 1;
        if i >= input.len() || input[i] != b',' {
            break;
        }
        i += 1;
    }

    return program;
}

#[derive(Copy, Clone)]
struct ReverseCPU {
    register_a: i64,
    register_b: Option<i64>,
    register_c: Option<i64>,

    pc: usize,
}

fn find_starting_condition(program: &Program) -> ReverseCPU {
    return ReverseCPU {
        register_a: 0,
        register_b: None,
        register_c: None,
        pc: 0,
    };
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
    let program = parse_input_2(input);
    let starting = find_starting_condition(&program);

    // sanity check
    let mut cpu = CPU {
        register_a: starting.register_a,
        register_b: starting.register_b.unwrap_or(0),
        register_c: starting.register_c.unwrap_or(0),
        pc: 0,
    };
    let output = cpu.run(&program);
    if output.len() != program.num_instructions {
        unreachable!();
    }
    for i in 0..output.len() {
        if output[i] != program.instructions[i] {
            unreachable!();
        }
    }
    // end sanity check

    return starting.register_a;
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
        assert_eq!(part2(&get_input()), 5);
    }
}
