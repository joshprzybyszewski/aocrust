#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut num_safe = 0;
    let mut l1: i32 = 0;
    let mut l2: i32 = 0;
    let mut i: i32 = 0;
    let mut is_ascending: bool = false;
    let mut is_safe: bool = true;

    for b in input.bytes() {
        if !is_safe {
            if b == b'\n' {
                is_safe = true;
                l1 = 0;
                l2 = 0;
                i = 0;
            }
            continue;
        }

        match b {
            // b'0' is value 48 in ascii.
            b'0'..=b'9' => {
                l2 *= 10;
                l2 += (b - 48) as i32;
            }
            b' ' => {
                if i > 0 {
                    if i == 1 {
                        is_ascending = l2 > l1;
                    }
                    if is_ascending && (l2 <= l1 || l2 > l1 + 3) {
                        is_safe = false
                    } else if !is_ascending && (l2 >= l1 || l2 < l1 - 3) {
                        is_safe = false
                    }
                }
                i += 1;
                l1 = l2;
                l2 = 0;
            }
            b'\n' => {
                if is_ascending && (l2 <= l1 || l2 > l1 + 3) {
                    is_safe = false
                } else if !is_ascending && (l2 >= l1 || l2 < l1 - 3) {
                    is_safe = false
                }
                if is_safe {
                    num_safe += 1;
                }
                is_safe = true;
                l1 = 0;
                l2 = 0;
                i = 0;
            }
            _ => unreachable!(),
        }
    }
    if is_ascending && (l2 <= l1 || l2 > l1 + 3) {
        is_safe = false
    } else if !is_ascending && (l2 >= l1 || l2 < l1 - 3) {
        is_safe = false
    }
    if is_safe {
        num_safe += 1;
    }

    return num_safe;
}

#[aoc(day2, part2)]
pub fn part2(_input_str: &str) -> i32 {
    return 0;
}
