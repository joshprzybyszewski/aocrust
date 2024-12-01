pub struct Lists {
    left: [u32; 1000],
    right: [u32; 1000],
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Lists {
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

    // TODO sort concurrently?
    left.sort();
    right.sort();

    return Lists { left, right };
}

#[aoc(day1, part1, Lists)]
pub fn part1(input: &Lists) -> u32 {
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

#[aoc(day1, part2, Lists)]
pub fn part2(input: &Lists) -> u32 {
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
