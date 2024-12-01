#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut left: [u32; 1000] = [0; 1000];
    let mut right: [u32; 1000] = [0; 1000];

    let mut i = 0;
    let mut val: u32 = 0;

    for (_, c) in input.as_bytes().iter().enumerate() {
        match c {
            b'\n' => {
                right[i] = val;
                val = 0;
                i += 1;
            }
            b' ' => {
                if val != 0 {
                    left[i] = val;
                }
                val = 0;
            }
            // b'0' is value 48 in ascii.
            b'0'..=b'9' => val = (val * 10) + ((*c as u32) - 48),
            _ => unreachable!(),
        }
    }
    if val != 0 {
        right[i] = val;
    }

    // TODO concurrently?
    left.sort();
    right.sort();

    let mut sum: u32 = 0;

    for (i, l) in left.iter().enumerate() {
        if *l > right[i] {
            sum += *l - right[i]
        } else {
            sum += right[i] - *l
        }
    }

    return sum;
}

#[aoc(day1, part2)]
pub fn part2(_input: &str) -> usize {
    return 0;
}
