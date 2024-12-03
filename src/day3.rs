#[aoc(day3, part1)]
pub fn part1(input_string: &str) -> i32 {
    let mut sum = 0;

    let mut l1: i32;
    let mut l2: i32;
    let mut ok: bool;
    let mut i: usize = 0;

    let input = input_string.as_bytes();
    let max_i = input.len() - 7; // need space at the end for mul(x,y)
    let max_len = input.len();
    while i < max_i {
        if input[i] != b'm' {
            i += 1;
            continue;
        }
        i += 1;

        if input[i] != b'u' {
            continue;
        }
        i += 1;

        if input[i] != b'l' {
            continue;
        }
        i += 1;

        if input[i] != b'(' {
            continue;
        }
        i += 1;

        l1 = 0;
        ok = false;
        while i < max_len {
            if input[i] == b',' {
                i += 1;
                ok = true;
                break;
            }
            if input[i] > b'9' || input[i] < b'0' {
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
        ok = false;
        while i < max_len {
            if input[i] == b')' {
                i += 1;
                ok = true;
                break;
            }
            if input[i] > b'9' || input[i] < b'0' {
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
    let max_i = input.len() - 7; // need space at the end for mul(x,y)
    let max_len = input.len();
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
                i += 1;

                is_enabled = false
            } else {
                if input[i] != b'(' {
                    continue;
                }
                i += 1;

                if input[i] != b')' {
                    continue;
                }
                i += 1;

                is_enabled = true
            }

            continue;
        }

        if !is_enabled {
            // doesn't matter what input[i] is. Just skip it.
            i += 1;
            continue;
        }

        if input[i] != b'm' {
            i += 1;
            continue;
        }
        i += 1;

        if input[i] != b'u' {
            continue;
        }
        i += 1;

        if input[i] != b'l' {
            continue;
        }
        i += 1;

        if input[i] != b'(' {
            continue;
        }
        i += 1;

        l1 = 0;
        ok = false;
        while i < max_len {
            if input[i] == b',' {
                i += 1;
                ok = true;
                break;
            }
            if input[i] > b'9' || input[i] < b'0' {
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
        ok = false;
        while i < max_len {
            if input[i] == b')' {
                i += 1;
                ok = true;
                break;
            }
            if input[i] > b'9' || input[i] < b'0' {
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

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day3.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_minimal() {
        assert_eq!(part1("mul(2,3)"), 6);
        assert_eq!(part1("mul(2,3mul(4,5)"), 20);
        assert_eq!(part1("mul(2,3)mul(4,5)mul(2,3"), 26);
        assert_eq!(part1("mul(2,3)add(4,5)mul( 2,3)"), 6);
    }

    #[test]
    fn part2_minimal() {
        assert_eq!(part2("mul(2,3)"), 6);
        assert_eq!(part1("mul(2,3mul(4,5)"), 20);
        assert_eq!(part2("mul(2,3)mul(4,5)mul(2,3"), 26);
        assert_eq!(part1("mul(2,3)add(4,5)mul( 2,3)"), 6);
        assert_eq!(part2("mul(2,3)don't()mul(4,5)do()mul(6,7)do"), 48);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 153469856)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 77055967)
    }
}
