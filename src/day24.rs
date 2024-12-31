use std::collections::HashSet;

const NUM_GATES: usize = 26 * 26 * 26;
const X_OFFSET: usize = (b'x' - b'a') as usize * 26 * 26;
const Y_OFFSET: usize = (b'y' - b'a') as usize * 26 * 26;
const Z_OFFSET: usize = (b'z' - b'a') as usize * 26 * 26;

const OPERATION_OR: u8 = 1;
const OPERATION_XOR: u8 = 2;
const OPERATION_AND: u8 = 3;

const VALUE_SET_MASK: u8 = 0x80;

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

    fn bit_offset(&self) -> u8 {
        if self.left >= Y_OFFSET {
            return (self.left - Y_OFFSET) as u8;
        }
        if self.left >= X_OFFSET {
            return (self.left - X_OFFSET) as u8;
        }
        if self.right >= Y_OFFSET {
            return (self.right - Y_OFFSET) as u8;
        }
        if self.right >= X_OFFSET {
            return (self.right - X_OFFSET) as u8;
        }
        unreachable!();
    }
}

#[derive(Copy, Clone)]
struct Logic {
    values: [u8; NUM_GATES],
    gates: [Gate; NUM_GATES],
    outs: [[usize; 2]; NUM_GATES],

    n_bits: u8,
}

impl Logic {
    fn new(input: &str) -> Self {
        let input = input.as_bytes();
        let mut i = 0;

        let mut output = Logic {
            values: [0; NUM_GATES],
            gates: [Gate::empty(); NUM_GATES],
            outs: [[NUM_GATES; 2]; NUM_GATES],
            n_bits: 0,
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

            output.add_gate(left, right, dest, op);
        }
        return output;
    }

    fn add_gate(&mut self, left: usize, right: usize, dest: usize, op: u8) {
        self.gates[dest] = Gate::new(left, right, op);

        if dest > Z_OFFSET {
            let bits = (dest - Z_OFFSET) as u8;
            if bits > self.n_bits {
                self.n_bits = bits;
            }
        }

        self.add_out(left, dest);
        self.add_out(right, dest);
    }

    fn add_out(&mut self, input: usize, output: usize) {
        if self.outs[input][0] == NUM_GATES {
            self.outs[input][0] = output;
            return;
        }
        if self.outs[input][1] == NUM_GATES {
            self.outs[input][1] = output;
            return;
        }

        println!("Input {input}. Output {output}");
        unreachable!();
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
        let mut bad: HashSet<usize> = HashSet::with_capacity(8);

        let mut bit = 0;

        loop {
            let z = Z_OFFSET + bit as usize;
            if self.gates[z].op == 0 {
                break;
            }
            self.find_swapped(bit, &mut bad, z);
            println!("Bit: {bit}");
            println!(" - {}", self.to_ids(&bad));

            bit += 1;
            // if bad.len() == 8 {
            //     // we were told 8 is the max size.
            //     break;
            // }
        }
        if bad.len() != 8 {
            // we were told 8 is the output size.
            println!("Wrong answer: {}", self.to_ids(&bad));
            unreachable!();
        }
        return self.to_ids(&bad);
    }

    fn to_ids(&self, bad: &HashSet<usize>) -> String {
        let mut ids = bad.iter().map(|&e| e).collect::<Vec<usize>>();

        ids.sort();
        return ids
            .iter()
            .map(|&id| convert_to_string(id))
            .collect::<Vec<String>>()
            .join(",");
    }

    fn find_swapped(&self, bit: u8, bad: &mut HashSet<usize>, output: usize) {
        if output >= X_OFFSET && output < Z_OFFSET {
            // x and y cannot be "outputs".
            return;
        }

        if self.is_swapped(bit, output) {
            bad.insert(output);
        }

        let gate = &self.gates[output];
        if self.gates[output].op == 0 {
            unreachable!();
        }

        // check the left input
        self.find_swapped(bit, bad, gate.left);
        // check the right input
        self.find_swapped(bit, bad, gate.right);
    }

    fn is_swapped(&self, bit: u8, output: usize) -> bool {
        let gate = &self.gates[output];

        if gate.op == OPERATION_OR {
            return self.is_swapped_or_gate(bit, output, gate);
        }

        if gate.op == OPERATION_AND {
            return self.is_swapped_and_gate(bit, output, gate);
        }

        if gate.op != OPERATION_XOR {
            println!("output: {output} = {}", convert_to_string(output));
            unreachable!();
        }

        return self.is_swapped_xor_gate(bit, output, gate);
    }

    fn is_swapped_or_gate(&self, bit: u8, output: usize, gate: &Gate) -> bool {
        if gate.op != OPERATION_OR {
            unreachable!();
        }

        // it should go OUT to an XOR and to an AND, both for the next bit index up.

        let outs = &self.outs[output];

        if outs[1] == NUM_GATES {
            let output_index = outs[0];
            if output_index < Z_OFFSET {
                // if there's only one output, it must be to z.
                return true;
            }
            // should only output for the last bit.
            return self.n_bits != bit;
        }

        let op0 = self.gates[outs[0]].op;
        let op1 = self.gates[outs[1]].op;

        if op0 == OPERATION_XOR {
            if op1 != OPERATION_AND {
                return true;
            }
        } else if op0 == OPERATION_AND {
            if op1 != OPERATION_XOR {
                return true;
            }
        }
        // TODO get bit offset for the outs and compare them.

        return false;
    }

    fn is_swapped_and_gate(&self, bit: u8, output: usize, gate: &Gate) -> bool {
        if gate.op != OPERATION_AND {
            unreachable!();
        }

        // NOTE: bit 0 uses an AND gate as the C_OUT.
        // if the left and right are X/Y,
        //   then it should go OUT to only an OR.
        // else,
        //   then it should go OUT to only an OR.

        let outs = &self.outs[output];
        if outs[1] != NUM_GATES {
            println!(
                "is_swapped_and_gate({bit}, {output} = {})",
                convert_to_string(output)
            );
            // this goes OUT to two other gates.
            // That's only expected for bit 0, where the AND gate is the carry out.
            // if bit != 0 {
            if gate.bit_offset() != 0 {
                return true;
            }

            // verify that it goes out to the XOR and AND of the next bit offset up.
            let op0 = self.gates[outs[0]].op;
            let op1 = self.gates[outs[1]].op;

            if op0 == OPERATION_XOR {
                if op1 != OPERATION_AND {
                    return true;
                }
            } else if op0 == OPERATION_AND {
                if op1 != OPERATION_XOR {
                    return true;
                }
            }

            return false;
        }

        if output >= Z_OFFSET {
            // should not output an AND to the z_offset. (unless it's the last carry out)
            return self.n_bits != bit;
        }

        let output_index = outs[0];
        let output_gate = &self.gates[output_index];
        if output_gate.op != OPERATION_OR {
            unreachable!();
        }

        return false;
    }

    fn is_swapped_xor_gate(&self, bit: u8, output: usize, gate: &Gate) -> bool {
        if gate.op != OPERATION_XOR {
            unreachable!();
        }
        // NOTE: bit 0 uses an XOR gate as the SUM.
        //
        // if the left and right are X/Y,
        //   then it should go OUT to an XOR and an AND.
        let outs = &self.outs[output];
        if gate.left >= X_OFFSET {
            if outs[1] == NUM_GATES {
                if bit == 0 {
                    // the first bit uses a single XOR as the SUM.
                    if output != Z_OFFSET {
                        // it _must_ output to the Z_OFFSET.
                        return true;
                    }
                    return false;
                }
                // both outputs should be populated.
                return true;
            }

            if outs[0] >= Z_OFFSET && outs[1] >= Z_OFFSET {
                unreachable!();
            }

            let op0 = self.gates[outs[0]].op;
            let op1 = self.gates[outs[1]].op;

            if op0 == OPERATION_XOR {
                if op1 != OPERATION_AND {
                    return true;
                }
            } else if op0 == OPERATION_AND {
                if op1 != OPERATION_XOR {
                    return true;
                }
            }

            return false;
        }

        // else,
        //   then it should go OUT to Z.
        if outs[0] != NUM_GATES || outs[1] != NUM_GATES {
            // there should only be one output gate: at `output`.
            return true;
        }

        if output < Z_OFFSET {
            // needs to be a sum output.
            return true;
        }

        let z_out = (output - Z_OFFSET) as u8;
        // should be the same bit offset.
        return z_out != bit;
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
        // not "ffk,jsv,qjs,rrw,z00,z01,z21,z39"
        // not "ffk,jsv,rrw,z06,z21,z39"
        // not "ckb,dnc,kbs,ksv,nbd,pgc,tqq,tsm"
        // not "ckb,ksv,tqq,z39"
        // not "ckb,ksv,tqq,z06,z20,z39"
        // not "ckb,jvr,ksv,nbd,qrm,tqq,z06,z20"
        // not "ckb,jvr,ksv,qrm,tqq,z06,z20,z39"
        // not "ffk,jsv,qjs,rrw,z01,z06,z21,z39"
        // not "ckb,kbs,ksv,nbd,pgc,z06,z20,z39"
        //      ckb,kbs,ksv,nbd,tqq,z06,z20,z39
        assert_eq!(part2(&get_input()), "ckb,kbs,ksv,nbd,tqq,z06,z20,z39");
    }
}
