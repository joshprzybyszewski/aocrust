#[aoc(day3, part1)]
pub fn part1(input_string: &str) -> i32 {
    let mut sum = 0;

    let mut l1: i32;
    let mut l2: i32;
    let mut ok: bool;
    let mut i: usize = 0;

    let input = input_string.as_bytes();
    let max_i = input.len() - 8; // need space at the end for mul(x,y)
    while i < max_i {
        if input[i] != b'm' {
            i += 1;
            continue;
        }
        if input[i + 1] != b'u' {
            i += 1;
            continue;
        }
        if input[i + 2] != b'l' {
            i += 2;
            continue;
        }
        if input[i + 3] != b'(' {
            i += 3;
            continue;
        }
        i += 4;
        l1 = 0;
        ok = true;
        while i < max_i {
            if input[i] == b',' {
                i += 1;
                break;
            }
            if input[i] < b'0' || input[i] > b'9' {
                ok = false;
                break;
            }
            l1 *= 10;
            l1 += (input[i] - b'0') as i32;
            i += 1;
        }
        if !ok {
            continue;
        }

        l2 = 0;
        while i < max_i {
            if input[i] == b')' {
                i += 1;
                break;
            }
            if input[i] < b'0' || input[i] > b'9' {
                ok = false;
                break;
            }
            l2 *= 10;
            l2 += (input[i] - b'0') as i32;
            i += 1;
        }
        if !ok {
            continue;
        }

        sum += l1 * l2
    }

    return sum;
}

#[aoc(day3, part2)]
pub fn part2(input_string: &str) -> i32 {
    let mut sum = 0;

    let mut l1: i32;
    let mut l2: i32;
    let mut ok: bool;
    let mut is_enabled: bool = true;
    let mut i: usize = 0;

    let input = input_string.as_bytes();
    let max_i = input.len() - 8; // need space at the end for mul(x,y)
    while i < max_i {
        if input[i] == b'd' {
            i += 1;
            if input[i] != b'o' {
                continue;
            }
            i += 1;
            if is_enabled {
                if input[i] != b'n' {
                    continue;
                }
                i += 1;
                if input[i] != b'\'' {
                    continue;
                }
                i += 1;
                if input[i] != b't' {
                    continue;
                }
                i += 1;
                if input[i] != b'(' {
                    continue;
                }
                i += 1;
                if input[i] != b')' {
                    continue;
                }
                is_enabled = false
            } else {
                if input[i] != b'(' {
                    continue;
                }
                i += 1;
                if input[i] != b')' {
                    continue;
                }
                is_enabled = true
            }

            continue;
        }
        if input[i] != b'm' {
            i += 1;
            continue;
        }
        if !is_enabled {
            // doesn't matter
            i += 1;
            continue;
        }
        if input[i + 1] != b'u' {
            i += 1;
            continue;
        }
        if input[i + 2] != b'l' {
            i += 2;
            continue;
        }
        if input[i + 3] != b'(' {
            i += 3;
            continue;
        }
        i += 4;
        l1 = 0;
        ok = true;
        while i < max_i {
            if input[i] == b',' {
                i += 1;
                break;
            }
            if input[i] < b'0' || input[i] > b'9' {
                ok = false;
                break;
            }
            l1 *= 10;
            l1 += (input[i] - b'0') as i32;
            i += 1;
        }
        if !ok {
            continue;
        }

        l2 = 0;
        while i < max_i {
            if input[i] == b')' {
                i += 1;
                break;
            }
            if input[i] < b'0' || input[i] > b'9' {
                ok = false;
                break;
            }
            l2 *= 10;
            l2 += (input[i] - b'0') as i32;
            i += 1;
        }
        if !ok {
            continue;
        }

        sum += l1 * l2
    }

    return sum;
}
