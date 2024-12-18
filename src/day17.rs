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

const ARBITRARY_MAX_CHECK_VALUE: i64 = 0xFF_FF;

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

#[derive(Copy, Clone, Debug)]
struct ReverseCPU {
    register_a: Option<i64>,
    register_b: Option<i64>,
    register_c: Option<i64>,

    pc: usize,
    instruction_index: usize,
}

#[derive(Copy, Clone, Debug)]
struct ProgramMeta {
    valid_pc: [bool; 32],

    // the index(es) where there could be out commands
    outs: [usize; 32],
    num_outs: usize,

    // jumped_from[my_pc][pc_of_jnz]
    jumped_from: [[usize; 32]; 32],
}

impl ProgramMeta {
    #[inline(always)]
    fn new(program: &Program) -> Self {
        let mut meta = ProgramMeta {
            valid_pc: [true; 32],
            outs: [33; 32],
            num_outs: 0,
            jumped_from: [[33; 32]; 32],
        };

        for i in 0..program.num_instructions - 1 {
            if program.instructions[i + 1] == 7
                && (program.instructions[i] == 0
                    || program.instructions[i] == 2
                    || program.instructions[i] > 4)
            {
                meta.valid_pc[i] = false;
                // TODO if i >= 8, then all subsequent i+2x indexes are not valid. probably
            }

            // remember jumps: jnz
            if program.instructions[i] == 3 {
                let dest = program.instructions[i + 1] as usize;
                for j in 0..32 {
                    if meta.jumped_from[dest][j] > 32 {
                        meta.jumped_from[dest][j] = i;
                        break;
                    }
                }
            }

            // remember outs: out
            if program.instructions[i] == 5 {
                meta.outs[meta.num_outs] = i;
                meta.num_outs += 1;
            }
        }

        return meta;
    }
}

fn find_starting_condition(program: &Program, meta: &ProgramMeta) -> ReverseCPU {
    for out_index in (0..meta.num_outs - 1).rev() {
        let start = ReverseCPU {
            register_a: None,
            register_b: None,
            register_c: None,
            pc: out_index,
            instruction_index: program.num_instructions - 1,
        };

        let answer = dfs_back(program, meta, &start);
        if answer.is_none() {
            continue;
        }
        let answer = answer.unwrap();
        println!("Found: {:?}", answer);
        if sanity_check(program, &answer) {
            return answer;
        }
    }

    unreachable!();
}

fn dfs_back(program: &Program, meta: &ProgramMeta, reverse: &ReverseCPU) -> Option<ReverseCPU> {
    if !meta.valid_pc[reverse.pc] {
        return None;
    }

    let op_code = program.instructions[reverse.pc];
    let operand = program.instructions[reverse.pc + 1];

    let literal_operand: i64 = operand as i64;

    match op_code {
        0 => {
            // adv
            // self.register_a >>= self.combo_operand(operand);
        }
        1 => {
            // bxl
            if reverse.register_b.is_none() {
                for possible in 0i64..ARBITRARY_MAX_CHECK_VALUE {
                    let mut coalesced = *reverse;
                    coalesced.register_b = Some(possible ^ literal_operand);
                    let answer = check_back_a_step(program, meta, &coalesced);
                    if !answer.is_none() {
                        return answer;
                    }
                }
                println!("stopped searching. {:?}", reverse);
                return None;
            }
            let mut coalesced = *reverse;
            coalesced.register_b = Some(coalesced.register_b.unwrap() ^ literal_operand);
            return check_back_a_step(program, meta, &coalesced);
        }
        2 => {
            // bst
            // self.register_b = self.combo_operand(operand) & 0x07;
        }
        3 => {
            // jnz
            return check_back_a_step(program, meta, reverse);
        }
        4 => {
            // bxc
            // self.register_b ^= self.register_c;
            if reverse.register_b.is_none() {
                if reverse.register_c.is_none() {
                    for possible_b in 0i64..ARBITRARY_MAX_CHECK_VALUE {
                        for possible_c in 0i64..ARBITRARY_MAX_CHECK_VALUE {
                            let mut coalesced = *reverse;
                            coalesced.register_b = Some(possible_b ^ possible_c);
                            coalesced.register_c = Some(possible_b ^ possible_c);
                            let answer = check_back_a_step(program, meta, &coalesced);
                            if !answer.is_none() {
                                return answer;
                            }
                        }
                    }
                    println!("stopped searching. {:?}", reverse);
                    return None;
                }
                for possible_b in 0i64..ARBITRARY_MAX_CHECK_VALUE {
                    let mut coalesced = *reverse;
                    coalesced.register_b = Some(possible_b ^ reverse.register_c.unwrap());
                    let answer = check_back_a_step(program, meta, &coalesced);
                    if !answer.is_none() {
                        return answer;
                    }
                }
                println!("stopped searching. {:?}", reverse);
                return None;
            }
            if reverse.register_c.is_none() {
                for possible_c in 0i64..ARBITRARY_MAX_CHECK_VALUE {
                    let mut coalesced = *reverse;
                    coalesced.register_b = Some(coalesced.register_b.unwrap() ^ possible_c);
                    coalesced.register_c = Some(possible_c);
                    let answer = check_back_a_step(program, meta, &coalesced);
                    if !answer.is_none() {
                        return answer;
                    }
                }
                println!("stopped searching. {:?}", reverse);
                return None;
            }

            let mut coalesced = *reverse;
            coalesced.register_b =
                Some(coalesced.register_b.unwrap() ^ coalesced.register_c.unwrap());
            return check_back_a_step(program, meta, &coalesced);
        }
        5 => {
            // out
            let needs_to_be = program.instructions[reverse.instruction_index];
            if operand == 6 {
                if reverse.register_c.is_none() {
                    for possible in 0i64..ARBITRARY_MAX_CHECK_VALUE {
                        let value = possible << 3 | (needs_to_be as i64);
                        let mut coalesced = *reverse;
                        coalesced.register_c = Some(value);
                        let answer = check_back_a_step(program, meta, &coalesced);
                        if !answer.is_none() {
                            return answer;
                        }
                    }
                    println!("stopped searching. {:?}", reverse);
                    return None;
                }
                let combo_operand = reverse.register_c.unwrap();
                if needs_to_be != (combo_operand as u8 & 0x07) {
                    return None;
                }
                return check_back_a_step(program, meta, reverse);
            } else if operand == 5 {
                if reverse.register_b.is_none() {
                    for possible in 0i64..ARBITRARY_MAX_CHECK_VALUE {
                        let value = possible << 3 | (needs_to_be as i64);
                        let mut coalesced = *reverse;
                        coalesced.register_b = Some(value);
                        let answer = check_back_a_step(program, meta, &coalesced);
                        if !answer.is_none() {
                            return answer;
                        }
                    }
                    println!("stopped searching. {:?}", reverse);
                    return None;
                }
                let combo_operand = reverse.register_b.unwrap();
                if needs_to_be != (combo_operand as u8 & 0x07) {
                    return None;
                }
                return check_back_a_step(program, meta, reverse);
            } else if operand == 4 {
                if reverse.register_a.is_none() {
                    for possible in 0i64..ARBITRARY_MAX_CHECK_VALUE {
                        let value = possible << 3 | (needs_to_be as i64);
                        let mut coalesced = *reverse;
                        coalesced.register_a = Some(value);
                        let answer = check_back_a_step(program, meta, &coalesced);
                        if !answer.is_none() {
                            return answer;
                        }
                    }
                    println!("stopped searching. {:?}", reverse);
                    return None;
                }
                let combo_operand = reverse.register_a.unwrap();
                if needs_to_be != (combo_operand as u8 & 0x07) {
                    return None;
                }
                return check_back_a_step(program, meta, reverse);
            } else if operand < 4 {
                if needs_to_be != operand & 0x07 {
                    return None;
                }
                return check_back_a_step(program, meta, reverse);
            }
            unreachable!();
        }
        6 => {
            // bdv
            // self.register_b = self.register_a >> self.combo_operand(operand);
        }
        7 => {
            // cdv
            // self.register_c = self.register_a >> self.combo_operand(operand);
        }
        _ => unreachable!(),
    }
    unreachable!();
}

fn check_back_a_step(
    program: &Program,
    meta: &ProgramMeta,
    reverse: &ReverseCPU,
) -> Option<ReverseCPU> {
    if reverse.pc > 1 {
        let mut standard = *reverse;
        standard.pc -= 2;
        return dfs_back(program, meta, &standard);
    }

    if !reverse.register_a.is_none() && reverse.register_a.unwrap() != 0 {
        for jnz_pc in meta.jumped_from[reverse.pc] {
            if jnz_pc >= program.num_instructions {
                break;
            }

            let mut jnz_source = *reverse;
            jnz_source.pc = jnz_pc;
            return dfs_back(program, meta, &jnz_source);
        }
    }

    return None;
}

fn sanity_check(program: &Program, starting: &ReverseCPU) -> bool {
    if starting.register_a.is_none() {
        return false;
    }

    let mut cpu = CPU {
        register_a: starting.register_a.unwrap(),
        register_b: starting.register_b.unwrap_or(0),
        register_c: starting.register_c.unwrap_or(0),
        pc: 0,
    };
    let output = cpu.run(&program);
    if output.len() != program.num_instructions {
        return false;
    }
    for i in 0..output.len() {
        if output[i] != program.instructions[i] {
            return false;
        }
    }
    return true;
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
    let program = parse_input_2(input);
    let meta = ProgramMeta::new(&program);
    let starting = find_starting_condition(&program, &meta);

    return starting.register_a.unwrap();
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
