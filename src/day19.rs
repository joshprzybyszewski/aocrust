// experientially, this is my longest pattern. Hopefully that's true.
const MAX_PATTERN_LEN: usize = 8;
// This is bound by the number of colors to the power of the max pattern len.
// 5 ^ 8 = 390_625
// However, more practically, I can't have more nodes than the length of the first line
// in my input file.
const MAX_NODES: usize = 4096;
// In my input, this is actually 60. Let's assume it can go up to 64.
const MAX_DESIGN_LEN: usize = 64;

// white (w), blue (u), black (b), red (r), or green (g)
const WHITE: u8 = 1; // 4419
const BLUE: u8 = 2; //  4332
const BLACK: u8 = 3; // 4478
const RED: u8 = 4; //   4390
const GREEN: u8 = 5; // 4352
const NUM_COLORS: usize = 6; // none is a color

#[derive(Copy, Clone, Debug)]
struct AllPatterns {
    nodes: [Node; MAX_NODES],
    num_nodes: usize,
}

#[derive(Copy, Clone, Debug)]
struct Node {
    next: [usize; NUM_COLORS],
}

impl AllPatterns {
    fn new() -> Self {
        return AllPatterns {
            nodes: [Node {
                next: [0; NUM_COLORS],
            }; MAX_NODES],
            num_nodes: NUM_COLORS,
        };
    }

    #[inline(always)]
    fn is_available(&self, node_id: usize) -> bool {
        return self.nodes[node_id].next[0] == MAX_NODES;
    }

    #[inline(always)]
    fn next_node_id(&self, node_id: usize, color: u8) -> usize {
        return self.nodes[node_id].next[color as usize];
    }

    fn add(&mut self, pattern: Pattern) {
        let mut node_id = pattern.colors[0] as usize;
        for i in 1..pattern.len {
            let next_id = self.nodes[node_id].next[pattern.colors[i] as usize];
            if next_id != 0 {
                node_id = next_id;
                continue;
            }
            let next_id = self.num_nodes;
            self.num_nodes += 1;
            self.nodes[node_id].next[pattern.colors[i] as usize] = next_id;
            node_id = next_id;
        }
        // mark this as is_available
        self.nodes[node_id].next[0] = MAX_NODES;
    }
}

#[derive(Copy, Clone, Debug)]
struct Pattern {
    colors: [u8; MAX_PATTERN_LEN],

    len: usize,
}

impl Pattern {
    fn new(input: &[u8], i: &mut usize) -> Self {
        let mut pattern = Pattern {
            colors: [0; MAX_PATTERN_LEN],
            len: 0,
        };
        loop {
            match input[*i] {
                b'b' => {
                    pattern.colors[pattern.len] = BLACK;
                }
                b'g' => {
                    pattern.colors[pattern.len] = GREEN;
                }
                b'r' => {
                    pattern.colors[pattern.len] = RED;
                }
                b'u' => {
                    pattern.colors[pattern.len] = BLUE;
                }
                b'w' => {
                    pattern.colors[pattern.len] = WHITE;
                }
                _ => {
                    // println!("input[{}] = {}", *i, input[*i]);
                    // unreachable!();
                }
            }
            pattern.len += 1;
            *i += 1;
            if input[*i] == b',' || input[*i] == b'\n' {
                break;
            }
        }
        return pattern;
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut array: [u8; MAX_PATTERN_LEN] = [b' '; MAX_PATTERN_LEN];
        for i in 0..self.len {
            match self.colors[i] {
                WHITE => array[i] = b'w',
                BLUE => array[i] = b'u',
                BLACK => array[i] = b'b',
                RED => array[i] = b'r',
                GREEN => array[i] = b'g',
                _ => {} // unreachable!(),
            }
        }
        return String::from_utf8_lossy(&array[0..self.len]).to_string();
    }
}

#[derive(Copy, Clone, Debug)]
struct Design {
    colors: [u8; MAX_DESIGN_LEN],
    len: usize,
}

impl Design {
    fn new(input: &[u8], i: &mut usize) -> Self {
        let mut design = Design {
            colors: [0; MAX_DESIGN_LEN],
            len: 0,
        };
        loop {
            match input[*i] {
                b'b' => {
                    design.colors[design.len] = BLACK;
                }
                b'g' => {
                    design.colors[design.len] = GREEN;
                }
                b'r' => {
                    design.colors[design.len] = RED;
                }
                b'u' => {
                    design.colors[design.len] = BLUE;
                }
                b'w' => {
                    design.colors[design.len] = WHITE;
                }
                _ => {
                    // println!("input[{}] = {}", *i, input[*i]);
                    // unreachable!();
                }
            }
            design.len += 1;
            *i += 1;
            if *i >= input.len() || input[*i] == b'\n' {
                break;
            }
        }
        return design;
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut array: [u8; MAX_DESIGN_LEN] = [b' '; MAX_DESIGN_LEN];
        for i in 0..self.len {
            match self.colors[i] {
                WHITE => array[i] = b'w',
                BLUE => array[i] = b'u',
                BLACK => array[i] = b'b',
                RED => array[i] = b'r',
                GREEN => array[i] = b'g',
                _ => {} // unreachable!(),
            }
        }
        return String::from_utf8_lossy(&array[0..self.len]).to_string();
    }
}

fn solve<const PART1: bool>(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut i: usize = 0;

    let mut patterns = AllPatterns::new();
    loop {
        patterns.add(Pattern::new(input, &mut i));
        if input[i] == b'\n' {
            // skip the two newlines
            i += 2;
            break;
        }
        // skip the comma and the space
        i += 2;
    }

    let mut total: u64 = 0;
    while i < input.len() {
        let design = Design::new(input, &mut i);
        let num_possible = get_num_to_end(&design, &patterns);
        if num_possible > 0 {
            total += if PART1 { 1 } else { num_possible }
        }
        // skip the newline
        i += 1;
    }

    return total;
}

#[inline(always)]
fn get_num_to_end(design: &Design, patterns: &AllPatterns) -> u64 {
    let mut farthest: [usize; MAX_DESIGN_LEN] = [0; MAX_DESIGN_LEN];
    let mut possibilities: [u64; MAX_DESIGN_LEN + 1] = [0; MAX_DESIGN_LEN + 1];

    possibilities[design.len] = 1;
    for design_index in (0..design.len).rev() {
        let mut max_distance = 0;
        let mut num_possible = 0;

        let mut node_id = design.colors[design_index] as usize;
        for offset in 1..=MAX_PATTERN_LEN {
            if patterns.is_available(node_id) {
                let jump = offset;
                let my_max = jump + farthest[design_index + jump];
                if my_max >= max_distance {
                    if my_max > max_distance {
                        num_possible = 0;
                        max_distance = my_max;
                    }
                    num_possible += possibilities[design_index + jump];
                }
            }
            if design_index + offset >= design.len {
                break;
            }

            node_id = patterns.next_node_id(node_id, design.colors[design_index + offset]);
            if node_id == 0 {
                // no more matches.
                break;
            }
        }

        farthest[design_index] = max_distance;
        possibilities[design_index] = num_possible;
    }
    if farthest[0] != design.len {
        return 0;
    }
    return possibilities[0];
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> u64 {
    return solve::<true>(input);
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> u64 {
    return solve::<false>(input);
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day19.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_example_input() -> &'static str {
        return "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(get_example_input()), 6)
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 353)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(get_example_input()), 16)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 880877787214477)
    }
}
