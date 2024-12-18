#[derive(Copy, Clone)]
struct Program {
    instructions: [u8; 16],

    num_instructions: usize,
}

impl Program {
    fn blank() -> Self {
        return Program {
            instructions: [0; 16],
            num_instructions: 0,
        };
    }

    fn to_string(&self) -> String {
        if self.num_instructions == 0 {
            return String::new();
        }
        let mut array: [u8; 32] = [b','; 32];
        for i in 0..self.num_instructions {
            array[i * 2] = b'0' + self.instructions[i];
        }
        return String::from_utf8_lossy(&array[0..(self.num_instructions * 2) - 1]).to_string();
    }
}

#[derive(Copy, Clone)]
struct CPU {
    register_a: i64,
    register_b: i64,
    register_c: i64,

    pc: usize,
}

impl CPU {
    fn with_a(a: i64) -> Self {
        return CPU {
            register_a: a,
            register_b: 0,
            register_c: 0,
            pc: 0,
        };
    }

    #[inline(always)]
    fn run(&mut self, program: &Program) -> Program {
        let mut output = Program::blank();

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
                    output.instructions[output.num_instructions] = next;
                    output.num_instructions += 1;
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

    let mut program = Program::blank();

    loop {
        program.instructions[program.num_instructions] = input[i] - b'0';
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
    return output.to_string();
    // .into_iter()
    // .map(|v| v.to_string())
    // .collect::<Vec<String>>()
    // .join(",");
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

    let mut program = Program::blank();

    loop {
        program.instructions[program.num_instructions] = input[i] - b'0';
        program.num_instructions += 1;
        i += 1;
        if i >= input.len() || input[i] != b',' {
            break;
        }
        i += 1;
    }

    return program;
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
    let program = parse_input_2(input);

    let mut a: i64 = 0;
    for _ in 0..(program.num_instructions - 1) {
        a <<= 3;
        a += 0x07;
    }
    let a = check(&program, a, 0);
    if a.is_none() {
        unreachable!();
    }
    return a.unwrap();
}

#[inline(always)]
fn check(program: &Program, a: i64, n: usize) -> Option<i64> {
    if n > program.num_instructions {
        return Some(a);
    }

    let val: i64 = 1i64 << 3 * (program.num_instructions - n);
    let mask: i64 = 0x07i64 << 3 * (program.num_instructions - n);
    let mut a = a & !mask;

    for _ in 0..8 {
        let mut cpu = CPU::with_a(a);
        let output = cpu.run(&program);
        if matches_last_n(&output, &program, n) {
            let answer = check(program, a, n + 1);
            if !answer.is_none() {
                return answer;
            }
        }
        a += val;
    }
    return None;
}

#[inline(always)]
fn matches_last_n(output: &Program, gold: &Program, n: usize) -> bool {
    if output.num_instructions < n {
        return false;
    }
    for i in 1..=n {
        if output.instructions[output.num_instructions - i]
            != gold.instructions[gold.num_instructions - i]
        {
            return false;
        }
    }

    return true;
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

    fn get_example_input_2() -> &'static str {
        return "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input_2()), 117440);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), "2,1,0,1,7,2,5,0,3");
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 267265166222235);
    }
}
