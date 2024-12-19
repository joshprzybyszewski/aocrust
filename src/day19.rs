use std::cmp::{max, Ordering};
use string_builder::Builder;

// white (w), blue (u), black (b), red (r), or green (g)
const WHITE: u8 = 1; // 4419
const BLUE: u8 = 2; //  4332
const BLACK: u8 = 3; // 4478
const RED: u8 = 4; //   4390
const GREEN: u8 = 5; // 4352

#[derive(Clone, Debug)]
struct AllPatterns {
    patterns_by_start_color: [Vec<Pattern>; 6],
}

impl AllPatterns {
    fn new() -> Self {
        return AllPatterns {
            patterns_by_start_color: [
                Vec::with_capacity(100),
                Vec::with_capacity(100),
                Vec::with_capacity(100),
                Vec::with_capacity(100),
                Vec::with_capacity(100),
                Vec::with_capacity(100),
            ],
        };
    }

    fn add(&mut self, pattern: Pattern) {
        self.patterns_by_start_color[pattern.colors[0] as usize].push(pattern);
    }

    fn sort(&mut self) {
        self.patterns_by_start_color[WHITE as usize].sort();
        self.patterns_by_start_color[BLUE as usize].sort();
        self.patterns_by_start_color[BLACK as usize].sort();
        self.patterns_by_start_color[RED as usize].sort();
        self.patterns_by_start_color[GREEN as usize].sort();
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut builder = Builder::default();
        for i in 1..=(GREEN as usize) {
            builder.append("");
            builder.append(
                self.patterns_by_start_color[i]
                    .iter()
                    .map(|pattern| pattern.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
            builder.append(" ");
        }
        return builder.string().unwrap();
    }
}

#[derive(Copy, Clone, Debug)]
struct Pattern {
    colors: [u8; 8],

    len: usize,
}

impl Pattern {
    fn new(input: &[u8], i: &mut usize) -> Self {
        let mut pattern = Pattern {
            colors: [0; 8],
            len: 0,
        };
        loop {
            match input[*i] {
                b'w' => {
                    pattern.colors[pattern.len] = WHITE;
                }
                b'u' => {
                    pattern.colors[pattern.len] = BLUE;
                }
                b'b' => {
                    pattern.colors[pattern.len] = BLACK;
                }
                b'r' => {
                    pattern.colors[pattern.len] = RED;
                }
                b'g' => {
                    pattern.colors[pattern.len] = GREEN;
                }
                _ => {
                    println!("input[{}] = {}", *i, input[*i]);
                    unreachable!();
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
        let mut array: [u8; 8] = [b' '; 8];
        for i in 0..self.len {
            match self.colors[i] {
                WHITE => array[i] = b'w',
                BLUE => array[i] = b'u',
                BLACK => array[i] = b'b',
                RED => array[i] = b'r',
                GREEN => array[i] = b'g',
                _ => unreachable!(),
            }
        }
        return String::from_utf8_lossy(&array[0..self.len]).to_string();
    }
}

impl Ord for Pattern {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.colors.cmp(&other.colors);
    }
}

impl PartialOrd for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        (self.len, self.colors) == (other.len, other.colors)
    }
}

impl Eq for Pattern {}

#[derive(Copy, Clone, Debug)]
struct Design {
    colors: [u8; 60],
    len: usize,
}

impl Design {
    fn new(input: &[u8], i: &mut usize) -> Self {
        let mut design = Design {
            colors: [0; 60],
            len: 0,
        };
        loop {
            match input[*i] {
                b'w' => {
                    design.colors[design.len] = WHITE;
                }
                b'u' => {
                    design.colors[design.len] = BLUE;
                }
                b'b' => {
                    design.colors[design.len] = BLACK;
                }
                b'r' => {
                    design.colors[design.len] = RED;
                }
                b'g' => {
                    design.colors[design.len] = GREEN;
                }
                _ => {
                    println!("input[{}] = {}", *i, input[*i]);
                    unreachable!();
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
        let mut array: [u8; 60] = [b' '; 60];
        for i in 0..self.len {
            match self.colors[i] {
                WHITE => array[i] = b'w',
                BLUE => array[i] = b'u',
                BLACK => array[i] = b'b',
                RED => array[i] = b'r',
                GREEN => array[i] = b'g',
                _ => unreachable!(),
            }
        }
        return String::from_utf8_lossy(&array[0..self.len]).to_string();
    }
}

struct DesignMatches {
    jumps: [Vec<usize>; 60],
    farthest: [usize; 60],
}

impl DesignMatches {
    fn new(design: &Design, patterns: &AllPatterns) -> Self {
        let mut matches = DesignMatches::empty();

        for design_index in 0..design.len {
            let start = design.colors[design_index];
            patterns.patterns_by_start_color[start as usize]
                .iter()
                .for_each(|pattern| {
                    for i in 0..pattern.len {
                        if design_index + i >= design.len
                            || design.colors[design_index + i] != pattern.colors[i]
                        {
                            return;
                        }
                    }
                    matches.jumps[design_index].push(pattern.len);
                });
            matches.jumps[design_index].reverse();
        }
        for i in (0..design.len).rev() {
            let mut max_distance = 0;
            matches.jumps[i].iter().for_each(|jump| {
                //
                max_distance = max(max_distance, *jump + matches.farthest[i + *jump])
            });
            matches.farthest[i] = max_distance;
        }
        return matches;
    }

    fn empty() -> Self {
        DesignMatches {
            farthest: [0; 60],
            jumps: [
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
                Vec::with_capacity(2),
            ],
        }
    }

    fn can_reach(&self, start: usize, target: usize) -> bool {
        return self.farthest[start] == target;
        // if start == target {
        //     return true;
        // }
        // return self.jumps[start]
        //     .iter()
        //     .any(|jump| self.can_reach(start + *jump, target));
    }
}

fn parse_input(input: &str) -> (AllPatterns, Vec<Design>) {
    // patterns: max len 8
    // total patterns 446.
    // total designs is 400
    // longest string is 60 for a design

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

    // println!("All patterns: {:?}", patterns.to_string());
    patterns.sort();
    // println!("All patterns: {:?}", patterns.to_string());

    let mut designs: Vec<Design> = Vec::with_capacity(512);
    while i < input.len() {
        designs.push(Design::new(input, &mut i));
        // skip the newline
        i += 1;
    }

    return (patterns, designs);
}

fn is_possible(design: &Design, patterns: &AllPatterns) -> bool {
    // println!("Checking {}", design.to_string());
    let matches = DesignMatches::new(design, patterns);
    return matches.can_reach(0, design.len);
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> u32 {
    let (patterns, designs) = parse_input(input);

    return designs
        .iter()
        .map(|design| if is_possible(design, &patterns) { 1 } else { 0 })
        .sum();
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> u32 {
    return 0;
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
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 1)
    }
}
