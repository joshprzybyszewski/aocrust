#[inline(always)]
fn read_input(input: &str, left: &mut Vec<i32>, right: &mut Vec<i32>) {
    let mut val: i32 = 0;

    input.bytes().into_iter().for_each(|c| match c {
        // b'0' is value 48 in ascii.
        b'0'..=b'9' => val = (val * 10) + (c - 48) as i32,
        b' ' => {
            if val != 0 {
                left.push(val);
            }
            val = 0;
        }
        b'\n' => {
            right.push(val);
            val = 0;
        }
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
pub fn part1(input_str: &str) -> i32 {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    read_input(input_str, &mut left, &mut right);

    return left
        .into_iter()
        .zip(right.into_iter())
        .fold(0, |sum, (l, r)| {
            return sum + (l - r).abs();
        });
}

#[aoc(day1, part2)]
pub fn part2(input_str: &str) -> i32 {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    read_input(input_str, &mut left, &mut right);

    let mut right_iter = right.into_iter();
    let mut current = right_iter.next().expect("must have a first value");
    let mut next_val = right_iter.next();

    let mut num_right: i32 = 1;
    while current == next_val.expect("should have a next value now") {
        next_val = right_iter.next();
        num_right += 1;
    }

    return left.into_iter().fold(0, |sum, v| {
        if v < current {
            return sum;
        }
        if v == current {
            return sum + v * num_right;
        }
        // v is more than the right value. Iterate forward.
        while v > current {
            if next_val.is_none() {
                return sum;
            }
            current = next_val.expect("checked above");
            num_right = 0;
            while !next_val.is_none() && current == next_val.expect("checked") {
                next_val = right_iter.next();
                num_right += 1;
            }
        }

        if v == current {
            return sum + v * num_right;
        }
        return sum;
    });
}
