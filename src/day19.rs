use std::cmp::Ordering;
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

#[inline(always)]
fn get_num_to_end(design: &Design, patterns: &AllPatterns) -> u64 {
    let mut farthest: [usize; 60] = [0; 60];
    let mut possibilities: [u64; 61] = [0; 61];

    possibilities[design.len] = 1;
    for design_index in (0..design.len).rev() {
        let mut max_distance = 0;
        let mut num_possible = 0;

        patterns.patterns_by_start_color[design.colors[design_index] as usize]
            .iter()
            .for_each(|pattern| {
                // TODO once we match a pattern, if we stop matching patterns, we don't need to check any after that.
                let jump = pattern.len;
                for i in 0..pattern.len {
                    if design_index + i >= design.len
                        || design.colors[design_index + i] != pattern.colors[i]
                    {
                        return;
                    }
                }

                let my_max = jump + farthest[design_index + jump];
                if max_distance > my_max {
                    return;
                }
                if my_max > max_distance {
                    num_possible = 0;
                    max_distance = my_max;
                }
                num_possible += possibilities[design_index + jump];
            });

        farthest[design_index] = max_distance;
        possibilities[design_index] = num_possible;
    }
    if farthest[0] != design.len {
        return 0;
    }
    return possibilities[0];
}

fn solve<const PART1: bool>(input: &str) -> u64 {
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

    let mut total: u64 = 0;
    while i < input.len() {
        let design = Design::new(input, &mut i);
        let num_possible = num_possible(&design, &patterns);
        if num_possible > 0 {
            total += if PART1 { 1 } else { num_possible }
        }
        // skip the newline
        i += 1;
    }

    return total;
}

#[inline(always)]
fn is_possible(design: &Design, patterns: &AllPatterns) -> bool {
    return get_num_to_end(design, patterns) != 0;
}

#[inline(always)]
fn num_possible(design: &Design, patterns: &AllPatterns) -> u64 {
    return get_num_to_end(design, patterns);
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
