// white (w), blue (u), black (b), red (r), or green (g)
const WHITE: u8 = 1; // 4419
const BLUE: u8 = 2; //  4332
const BLACK: u8 = 3; // 4478
const RED: u8 = 4; //   4390
const GREEN: u8 = 5; // 4352

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
                _ => unreachable!(),
            }
            pattern.len += 1;
            *i += 1;
            if input[*i] != b',' && input[*i] != b'\n' {
                break;
            }
        }
        return pattern;
    }
}

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
                _ => unreachable!(),
            }
            design.len += 1;
            *i += 1;
            if *i >= input.len() || input[*i] != b'\n' {
                break;
            }
        }
        return design;
    }
}

fn parse_input(input: &str) -> (Vec<Pattern>, Vec<Design>) {
    // patterns: max len 8
    // total patterns 446.
    // total designs is 400
    // longest string is 60 for a design

    let input = input.as_bytes();
    let mut i: usize = 0;

    let mut patterns: Vec<Pattern> = Vec::with_capacity(512);
    loop {
        patterns.push(Pattern::new(input, &mut i));
        if input[i] == b'\n' {
            // skip the two newlines
            i += 2;
            break;
        }
        // skip the comma and the space
        i += 2;
    }

    let mut designs: Vec<Design> = Vec::with_capacity(512);
    while input[i] != b'\n' {
        designs.push(Design::new(input, &mut i));
    }

    return (patterns, designs);
}

fn is_possible(design: &Design, patterns: &Vec<Pattern>) -> bool {
    return false;
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

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 1)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 1)
    }
}
