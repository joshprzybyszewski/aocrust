pub struct Lists {
    left: Vec<u32>,
    right: Vec<u32>,
}

// #[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Lists {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    let mut val: u32 = 0;

    for (_, c) in input.as_bytes().iter().enumerate() {
        match c {
            b'\n' => {
                right.push(val);
                val = 0;
            }
            b' ' => {
                if val != 0 {
                    left.push(val);
                }
                val = 0;
            }
            // b'0' is value 48 in ascii.
            b'0'..=b'9' => val = (val * 10) + ((*c as u32) - 48),
            _ => unreachable!(),
        }
    }
    if val != 0 {
        right.push(val);
    }

    // TODO sort concurrently?
    left.sort();
    right.sort();

    return Lists { left, right };
}

#[aoc(day1, part1)]
// #[aoc(day1, part1, Lists)]
pub fn part1(input_str: &str) -> u32 {
    // pub fn part1(input: &Lists) -> u32 {
    let input = input_generator(input_str);
    let mut sum: u32 = 0;

    for (i, l) in input.left.iter().enumerate() {
        if *l > input.right[i] {
            sum += *l - input.right[i]
        } else {
            sum += input.right[i] - *l
        }
    }

    return sum;
}

// #[aoc(day1, part2, Lists)]
#[aoc(day1, part2)]
pub fn part2(input_str: &str) -> u32 {
    // pub fn part2(input: &Lists) -> u32 {
    let input = input_generator(input_str);
    let mut sum: u32 = 0;

    let mut ri = 0;
    let mut num_right: u32 = 1;

    for v in input.left {
        if v < input.right[ri] {
            continue;
        }
        if v == input.right[ri] {
            sum += v * num_right;
            continue;
        }
        // v is more than the right value. Iterate forward.

        ri += num_right as usize;
        while ri < input.right.len() && input.right[ri] < v {
            ri += 1;
        }
        num_right = 1;
        while ri + (num_right as usize) < input.right.len()
            && input.right[ri] == input.right[ri + (num_right as usize)]
        {
            num_right += 1;
        }

        if v == input.right[ri] {
            sum += v * num_right;
        }
    }

    return sum;
}
