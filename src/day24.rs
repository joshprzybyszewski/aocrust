use std::collections::HashSet;

const NUM_GATES: usize = 26 * 26 * 26;
const X_OFFSET: usize = (b'x' - b'a') as usize * 26 * 26;
const Y_OFFSET: usize = (b'y' - b'a') as usize * 26 * 26;
const Z_OFFSET: usize = (b'z' - b'a') as usize * 26 * 26;

const OPERATION_OR: u8 = 1;
const OPERATION_XOR: u8 = 2;
const OPERATION_AND: u8 = 3;

#[derive(Copy, Clone)]
struct Gate {
    id: usize,
    left: usize,
    right: usize,
    op: u8,
    _pad: [u8; 3],
    outs: [usize; 2],
}

impl Gate {
    fn new(dest: usize, left: usize, right: usize, op: u8) -> Self {
        Gate {
            id: dest,
            left,
            right,
            op,
            _pad: [0; 3],
            outs: [NUM_GATES; 2],
        }
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

struct Logic {
    gates: Vec<Gate>,
    xs: [u8; 64],
    ys: [u8; 64],

    n_bits: u8,
}

impl Logic {
    fn new<const PART1: bool>(input: &str) -> Self {
        let input = input.as_bytes();
        let mut i = 0;

        let mut output = Logic {
            gates: Vec::with_capacity(5 * 64), // 5 gates for up to a 64 bit adder
            xs: [0; 64],
            ys: [0; 64],
            n_bits: 0,
        };

        // iterate through starting
        loop {
            if input[i] == b'\n' {
                break;
            }
            let index = output.parse_index(input, i);
            i += 5;
            if index >= Y_OFFSET {
                output.ys[index - Y_OFFSET] = input[i] - b'0';
            } else {
                output.xs[index - X_OFFSET] = input[i] - b'0';
            }
            i += 2;
        }
        i += 1;

        // iterate through gates
        let mut all_outs: Vec<(usize, usize)> = Vec::with_capacity(5 * 64 * 2);
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

            all_outs.extend(output.add_gate(left, right, dest, op));
        }

        output.gates.sort_by(|a, b| a.id.cmp(&b.id));

        if PART1 {
            return output;
        }

        for (input_gate, output_gate) in all_outs {
            let mut_gate_index = output.gates.iter().position(|g| g.id == input_gate);
            if mut_gate_index.is_none() {
                if input_gate >= X_OFFSET && input_gate < Z_OFFSET {
                    continue;
                }
                println!(
                    "Couldn't find {input_gate} = {}",
                    convert_to_string(input_gate)
                );
                unreachable!();
            }
            let mut_gate_index = mut_gate_index.unwrap();
            if output.gates[mut_gate_index].outs[0] == NUM_GATES {
                output.gates[mut_gate_index].outs[0] = output_gate;
            } else {
                if output.gates[mut_gate_index].outs[1] != NUM_GATES {
                    unreachable!();
                }
                output.gates[mut_gate_index].outs[1] = output_gate;
            }
        }

        return output;
    }

    #[inline(always)]
    fn add_gate(&mut self, left: usize, right: usize, dest: usize, op: u8) -> Vec<(usize, usize)> {
        self.gates.push(Gate::new(dest, left, right, op));

        if dest > Z_OFFSET {
            let bits = (dest - Z_OFFSET) as u8;
            if bits > self.n_bits {
                self.n_bits = bits;
            }
        }

        return Vec::from([(left, dest), (right, dest)]);
    }

    #[inline(always)]
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

    #[inline(always)]
    fn solve_part1(&self) -> u64 {
        let mut output: u64 = 0;

        let mut offset: usize = 0;
        for _ in 0..=self.n_bits {
            output |= self.get_value(Z_OFFSET + offset) << offset;
            offset += 1;
        }

        return output;
    }

    fn get_gate(&self, index: usize) -> &Gate {
        let gate_index = self.gates.binary_search_by(|g| g.id.cmp(&index));
        if !gate_index.is_ok() {
            println!("Couldn't find {index} = {}", convert_to_string(index));
            unreachable!();
        }
        return &self.gates[gate_index.unwrap()];
    }

    fn get_value(&self, index: usize) -> u64 {
        if index >= X_OFFSET && index < Z_OFFSET {
            if index >= Y_OFFSET {
                return self.ys[index - Y_OFFSET] as u64;
            }
            return self.xs[index - X_OFFSET] as u64;
        }

        let gate = self.get_gate(index);
        if gate.op == 0 {
            unreachable!();
        }

        let left_val = self.get_value(gate.left);
        let right_val = self.get_value(gate.right);
        match gate.op {
            OPERATION_AND => return left_val & right_val,
            OPERATION_XOR => return left_val ^ right_val,
            OPERATION_OR => return left_val | right_val,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn solve_part2(&self) -> String {
        let mut bad: HashSet<usize> = HashSet::with_capacity(8);

        let mut bit = 0;

        loop {
            if bit > self.n_bits {
                break;
            }
            let z = Z_OFFSET + bit as usize;
            self.find_swapped(bit, &mut bad, z);

            bit += 1;
            if bad.len() == 8 {
                // we were told 8 is the max size.
                break;
            }
        }
        // if bad.len() != 8 {
        //     // we were told 8 is the output size.
        //     println!("Wrong answer: {}", self.to_ids(&bad));
        //     unreachable!();
        // }
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

        let gate = self.get_gate(output);
        if self.is_swapped(bit, output, gate) {
            bad.insert(output);
        }

        if gate.op == 0 {
            unreachable!();
        }

        // check the left input
        self.find_swapped(bit, bad, gate.left);
        // check the right input
        self.find_swapped(bit, bad, gate.right);
    }

    #[inline(always)]
    fn is_swapped(&self, bit: u8, output: usize, gate: &Gate) -> bool {
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

    #[inline(always)]
    fn is_swapped_or_gate(&self, bit: u8, output: usize, gate: &Gate) -> bool {
        if gate.op != OPERATION_OR {
            unreachable!();
        }

        // it should go OUT to an XOR and to an AND, both for the next bit index up.

        let outs = &gate.outs;

        if outs[1] == NUM_GATES {
            let output_index = outs[0];
            if output_index < Z_OFFSET {
                // if there's only one output, it must be to z.
                return true;
            }
            // should only output for the last bit.
            return self.n_bits != bit;
        }

        let op0 = self.get_gate(outs[0]).op;
        let op1 = self.get_gate(outs[1]).op;

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

    #[inline(always)]
    fn is_swapped_and_gate(&self, bit: u8, output: usize, gate: &Gate) -> bool {
        if gate.op != OPERATION_AND {
            unreachable!();
        }

        // NOTE: bit 0 uses an AND gate as the C_OUT.
        // if the left and right are X/Y,
        //   then it should go OUT to only an OR.
        // else,
        //   then it should go OUT to only an OR.

        let outs = &gate.outs;
        if outs[1] != NUM_GATES {
            // this goes OUT to two other gates.
            // That's only expected for bit 0, where the AND gate is the carry out.
            // if bit != 0 {
            if gate.bit_offset() != 0 {
                return true;
            }

            // verify that it goes out to the XOR and AND of the next bit offset up.
            let op0 = self.get_gate(outs[0]).op;
            let op1 = self.get_gate(outs[1]).op;

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
        if self.get_gate(output_index).op != OPERATION_OR {
            unreachable!();
        }

        return false;
    }

    #[inline(always)]
    fn is_swapped_xor_gate(&self, bit: u8, output: usize, gate: &Gate) -> bool {
        if gate.op != OPERATION_XOR {
            unreachable!();
        }
        // NOTE: bit 0 uses an XOR gate as the SUM.
        //
        // if the left and right are X/Y,
        //   then it should go OUT to an XOR and an AND.
        let outs = &gate.outs;
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
                // return true;
                unreachable!();
            }

            let op0 = self.get_gate(outs[0]).op;
            let op1 = self.get_gate(outs[1]).op;

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

#[inline(always)]
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
    let logic = Logic::new::<true>(input);

    return logic.solve_part1();
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> String {
    let logic = Logic::new::<false>(input);

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

    fn get_competition_input() -> String {
        let input_path = "input/2024/day24_competition.txt";
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

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 2024);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 49574189473968);
        assert_eq!(part1(&get_competition_input()), 50411513338638);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), "ckb,kbs,ksv,nbd,tqq,z06,z20,z39");
        assert_eq!(
            part2(&get_competition_input()),
            "gfv,hcm,kfs,tqm,vwr,z06,z11,z16"
        );
    }
}
