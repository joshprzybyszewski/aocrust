#[inline(always)]
fn read_input(input: &str, left: &mut Vec<usize>, right: &mut Vec<usize>) {
    let mut val: usize = 0;

    input.as_bytes().iter().for_each(|c| match c {
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
        b'0'..=b'9' => val = (val * 10) + ((*c as usize) - 48),
        _ => unreachable!(),
    });

    if val != 0 {
        right.push(val);
    }

    // TODO sort concurrently?
    left.sort();
    right.sort();
}

#[aoc(day1, part1)]
pub fn part1(input_str: &str) -> usize {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    read_input(input_str, &mut left, &mut right);

    return left.iter().zip(right.iter()).fold(0, |sum, (&l, &r)| {
        return if l > r { sum + l - r } else { sum + r - l };
    });
}

#[aoc(day1, part2)]
pub fn part2(input_str: &str) -> usize {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    read_input(input_str, &mut left, &mut right);
    let mut sum: usize = 0;

    let mut ri = 0;
    let mut num_right: usize = 1;

    for v in left {
        if v < right[ri] {
            continue;
        }
        if v == right[ri] {
            sum += v * num_right;
            continue;
        }
        // v is more than the right value. Iterate forward.

        ri += num_right as usize;
        while ri < right.len() && right[ri] < v {
            ri += 1;
        }
        num_right = 1;
        while ri + (num_right as usize) < right.len()
            && right[ri] == right[ri + (num_right as usize)]
        {
            num_right += 1;
        }

        if v == right[ri] {
            sum += v * num_right;
        }
    }

    return sum;
}
