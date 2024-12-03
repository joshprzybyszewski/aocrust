#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut num_safe = 0;
    let mut l1: i32 = 0;
    let mut l2: i32 = 0;
    let mut i: i32 = 0;
    let mut is_ascending: bool = false;
    let mut is_not_safe: bool = false;

    for b in input.bytes() {
        if is_not_safe {
            if b == b'\n' {
                is_not_safe = false;
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
                    if is_ascending {
                        is_not_safe = l2 <= l1 || l2 > l1 + 3
                    } else {
                        is_not_safe = l2 >= l1 || l2 < l1 - 3
                    }
                }
                i += 1;
                l1 = l2;
                l2 = 0;
            }
            b'\n' => {
                if is_ascending {
                    is_not_safe = l2 <= l1 || l2 > l1 + 3
                } else {
                    is_not_safe = l2 >= l1 || l2 < l1 - 3
                }
                if !is_not_safe {
                    num_safe += 1;
                }
                is_not_safe = false;
                l1 = 0;
                l2 = 0;
                i = 0;
            }
            _ => unreachable!(),
        }
    }
    if is_ascending {
        is_not_safe = l2 <= l1 || l2 > l1 + 3
    } else {
        is_not_safe = l2 >= l1 || l2 < l1 - 3
    }
    if !is_not_safe {
        num_safe += 1;
    }

    return num_safe;
}

fn is_safe_part1(levels: &[i32], n: usize) -> bool {
    if levels.len() < 2 {
        return true;
    }
    if levels[1] > levels[0] {
        for i in 1..n {
            if levels[i] <= levels[i - 1] || levels[i] > levels[i - 1] + 3 {
                return false;
            }
        }
        return true;
    }

    for i in 1..n {
        if levels[i] >= levels[i - 1] || levels[i] < levels[i - 1] + 3 {
            return false;
        }
    }

    return true;
}

fn is_safe_part2(levels: &Vec<i32>) -> bool {
    println!("is_safe_part2 {:?}", levels);
    let n = levels.len() - 1;
    let mut check: [i32; 10] = [0; 10];
    for (i, e) in levels.iter().skip(1).enumerate() {
        check[i] = *e
    }
    for i in 0..n {
        if is_safe_part1(&check, n) {
            println!(" is safe: {:?}", &check[0..n]);
            return true;
        }
        check[i] = levels[i];
        // println!("Checking {:?}", &check[0..n]);
    }
    return false;
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let mut levels = Vec::with_capacity(16);

    let mut num_safe = 0;
    let mut cur: i32 = 0;

    for b in input.bytes() {
        match b {
            // b'0' is value 48 in ascii.
            b'0'..=b'9' => {
                cur *= 10;
                cur += (b - 48) as i32;
            }
            b' ' => {
                levels.push(cur);
                cur = 0;
            }
            b'\n' => {
                levels.push(cur);

                if is_safe_part2(&levels) {
                    num_safe += 1;
                }
                cur = 0;
                levels.clear();
            }
            _ => unreachable!(),
        }
    }
    levels.push(cur);
    if is_safe_part2(&levels) {
        num_safe += 1;
    }

    return num_safe;
}
