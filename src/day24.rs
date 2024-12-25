const NUM_GATES: usize = 26 * 26 * 26;
const X_OFFSET: usize = (b'x' - b'a') as usize * 26 * 26;
const Y_OFFSET: usize = (b'y' - b'a') as usize * 26 * 26;
const Z_OFFSET: usize = (b'z' - b'a') as usize * 26 * 26;

const OPERATION_OR: u8 = 1;
const OPERATION_XOR: u8 = 2;
const OPERATION_AND: u8 = 3;

const VALUE_SET_MASK: u8 = 0x80;

const X: u8 = 1;
const Y: u8 = 2;
const C_OUT: u8 = 3;
const XOR_1: u8 = 4;
const XOR_2: u8 = 5;
const AND_1: u8 = 6;
const AND_2: u8 = 7;
const BAD: u8 = 8;

#[derive(Copy, Clone)]
struct Gate {
    left: usize,
    right: usize,
    op: u8,
}

impl Gate {
    fn empty() -> Self {
        Gate {
            left: NUM_GATES + 1,
            right: NUM_GATES + 1,
            op: 0,
        }
    }

    fn new(left: usize, right: usize, op: u8) -> Self {
        Gate { left, right, op }
    }
}

#[derive(Copy, Clone)]
struct Logic {
    values: [u8; NUM_GATES],
    gates: [Gate; NUM_GATES],
}

impl Logic {
    fn new(input: &str) -> Self {
        let input = input.as_bytes();
        let mut i = 0;

        let mut output = Logic {
            values: [0; NUM_GATES],
            gates: [Gate::empty(); NUM_GATES],
        };

        // iterate through starting
        loop {
            if input[i] == b'\n' {
                break;
            }
            let index = output.parse_index(input, i);
            i += 5;
            output.values[index] = VALUE_SET_MASK | (input[i] - b'0');
            i += 2;
        }
        i += 1;

        // iterate through gates
        loop {
            if i >= input.len() || input[i] == b'\n' {
                break;
            }

            let left = output.parse_index(input, i);
            i += 4;
            let op: u8;
            if input[i] == b'X' {
                i += 4;
                op = OPERATION_XOR;
            } else if input[i] == b'A' {
                i += 4;
                op = OPERATION_AND;
            } else if input[i] == b'O' {
                i += 3;
                op = OPERATION_OR;
            } else {
                unreachable!();
            }
            let right = output.parse_index(input, i);
            i += 7;

            let dest = output.parse_index(input, i);
            i += 4;
            output.gates[dest] = Gate::new(left, right, op);
        }
        return output;
    }

    fn parse_index(&self, input: &[u8], i: usize) -> usize {
        if input[i] == b'x' {
            let index = (input[i + 1] - b'0') as usize * 10 + (input[i + 2] - b'0') as usize;
            return X_OFFSET + index;
        }
        if input[i] == b'y' {
            let index = (input[i + 1] - b'0') as usize * 10 + (input[i + 2] - b'0') as usize;
            return index + Y_OFFSET;
        }
        if input[i] == b'z' {
            let index = (input[i + 1] - b'0') as usize * 10 + (input[i + 2] - b'0') as usize;
            return index + Z_OFFSET;
        }

        return (input[i] - b'a') as usize * 26 * 26
            + (input[i + 1] - b'a') as usize * 26
            + (input[i + 2] - b'a') as usize;
    }

    fn solve_part1(&self) -> u64 {
        let mut output: u64 = 0;

        let mut offset: usize = 0;
        for _ in 0..64 {
            output |= (self.get_value(Z_OFFSET + offset) as u64) << offset;
            offset += 1;
        }

        return output;
    }

    fn get_value(&self, index: usize) -> u8 {
        if self.values[index] & VALUE_SET_MASK == VALUE_SET_MASK {
            return self.values[index] & 1;
        }

        if self.gates[index].op == 0 {
            if index >= Z_OFFSET {
                return 0;
            }
            if index >= Y_OFFSET {
                println!("not set y: {}", index - Y_OFFSET);
                return 0;
            }
            if index >= X_OFFSET {
                println!("not set x: {}", index - X_OFFSET);
                return 0;
            }
            unreachable!();
        }

        let left_val = self.get_value(self.gates[index].left);
        let right_val = self.get_value(self.gates[index].right);
        match self.gates[index].op {
            OPERATION_AND => return left_val & right_val,
            OPERATION_XOR => return left_val ^ right_val,
            OPERATION_OR => return left_val | right_val,
            _ => unreachable!(),
        }
    }

    fn solve_part2(&self) -> String {
        let mut ids: Vec<usize> = Vec::with_capacity(8);

        let mut z = Z_OFFSET + 63;
        loop {
            if self.gates[z].op == 0 {
                z -= 1;
                continue;
            }

            let mine = self.check_gate(&mut ids, z);
            if mine != C_OUT {
                println!("mine = {mine}");
                // unreachable!();
                ids.push(z);
            }
            z -= 1;
            break;
        }
        loop {
            let mine = self.check_gate(&mut ids, z);
            if mine != XOR_2 {
                println!("mine = {mine}");
                // unreachable!();
                ids.push(z);
            }

            if z == Z_OFFSET || ids.len() == 8 {
                break;
            }
            z -= 1;
        }

        ids.sort();
        return ids
            .iter()
            .map(|&id| convert_to_string(id))
            .collect::<Vec<String>>()
            .join(",");
    }

    fn check_gate(&self, bad: &mut Vec<usize>, index: usize) -> u8 {
        if self.values[index] & VALUE_SET_MASK == VALUE_SET_MASK {
            if index >= Z_OFFSET {
                unreachable!();
            }
            if index >= Y_OFFSET {
                return Y;
            }
            if index >= X_OFFSET {
                return X;
            }
            unreachable!();
        }

        let left_is = self.check_gate(bad, self.gates[index].left);
        let right_is = self.check_gate(bad, self.gates[index].right);

        let my_op = self.gates[index].op;

        if left_is == X || left_is == Y || right_is == X || right_is == Y {
            if right_is == BAD || left_is == BAD {
                match my_op {
                    OPERATION_XOR => return XOR_1,
                    OPERATION_AND => return AND_1,
                    _ => unreachable!(),
                }
            }
            if left_is == X && right_is != Y {
                bad.push(index);
                return BAD;
            }
            if left_is == Y && right_is != X {
                bad.push(index);
                return BAD;
            }
            match my_op {
                OPERATION_XOR => return XOR_1,
                OPERATION_AND => return AND_1,
                _ => {}
            }
            bad.push(index);
            return BAD;
        }

        if left_is == XOR_1 || left_is == C_OUT || right_is == XOR_1 || right_is == C_OUT {
            if right_is == BAD || left_is == BAD {
                match my_op {
                    OPERATION_XOR => return XOR_2,
                    OPERATION_AND => return AND_2,
                    _ => unreachable!(),
                }
            }
            if left_is == XOR_1 && right_is != C_OUT {
                bad.push(index);
                return BAD;
            }
            if right_is == XOR_1 && left_is != C_OUT {
                bad.push(index);
                return BAD;
            }
            match my_op {
                OPERATION_XOR => return XOR_2,
                OPERATION_AND => return AND_2,
                _ => {}
            }
            bad.push(index);
            return BAD;
        }

        if left_is == AND_2 || left_is == AND_1 || right_is == AND_2 || right_is == AND_1 {
            if right_is == BAD || left_is == BAD {
                match my_op {
                    OPERATION_OR => return C_OUT,

                    _ => unreachable!(),
                }
            }
            if left_is == AND_2 && right_is != AND_1 {
                bad.push(index);
                return BAD;
            }
            if right_is == AND_2 && left_is != AND_1 {
                bad.push(index);
                return BAD;
            }
            match my_op {
                OPERATION_OR => return C_OUT,
                _ => {}
                // _ => unreachable!(),
            }
            bad.push(index);
            return BAD;
        }

        println!("left_is = {left_is}");
        println!("right_is = {right_is}");
        unreachable!();
    }
}

fn convert_to_string(node_id: usize) -> String {
    let mut array: [u8; 3] = [0; 3];
    if node_id >= Z_OFFSET {
        array[0] = b'z';
        array[1] = b'0' + ((node_id - Z_OFFSET) / 10) as u8;
        array[2] = b'0' + ((node_id - Z_OFFSET) % 10) as u8;
    } else if node_id >= Y_OFFSET {
        array[0] = b'y';
        array[1] = b'0' + ((node_id - Y_OFFSET) / 10) as u8;
        array[2] = b'0' + ((node_id - Y_OFFSET) % 10) as u8;
    } else if node_id >= X_OFFSET {
        array[0] = b'x';
        array[1] = b'0' + ((node_id - X_OFFSET) / 10) as u8;
        array[2] = b'0' + ((node_id - X_OFFSET) % 10) as u8;
    } else {
        array[0] = b'a' + (node_id / 676) as u8;
        array[1] = b'a' + ((node_id / 26) % 26) as u8;
        array[2] = b'a' + (node_id % 26) as u8;
    }

    return String::from_utf8_lossy(&array).to_string();
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> u64 {
    let logic = Logic::new(input);

    return logic.solve_part1();
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> String {
    let logic = Logic::new(input);

    return logic.solve_part2();
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day24.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_example_input() -> &'static str {
        return "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    }

    fn get_example_input_2() -> &'static str {
        return "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 2024);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 49574189473968)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input_2()), "z00,z01,z02,z05");
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), "");
    }
}
