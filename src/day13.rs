const UNIT_CONVERSION_ERROR: i64 = 10_000_000_000_000;

fn get_next_cost<const CHECK_LIMIT: bool>(input: &[u8], i: &mut usize) -> i64 {
    let mut a_x: i64 = 0;
    let mut a_y: i64 = 0;
    let mut b_x: i64 = 0;
    let mut b_y: i64 = 0;
    let mut p_x: i64 = 0;
    let mut p_y: i64 = 0;

    // Parse Button A
    // Parse a_x.
    // if input[*i] != b'B'
    //     || input[*i + 1] != b'u'
    //     || input[*i + 2] != b't'
    //     || input[*i + 3] != b't'
    //     || input[*i + 4] != b'o'
    //     || input[*i + 5] != b'n'
    //     || input[*i + 6] != b' '
    //     || input[*i + 7] != b'A'
    //     || input[*i + 8] != b':'
    //     || input[*i + 9] != b' '
    //     || input[*i + 10] != b'X'
    //     || input[*i + 11] != b'+'
    // {
    //     println!("input[{}]: {:?}", *i, &input[*i..*i + 11]);
    //     unreachable!();
    // }

    *i += 12;
    a_x += (input[*i] - b'0') as i64;
    *i += 1;
    while input[*i] != b',' {
        a_x *= 10;
        a_x += (input[*i] - b'0') as i64;
        *i += 1;
    }

    // Parse a_y.
    // if input[*i] != b','
    //     || input[*i + 1] != b' '
    //     || input[*i + 2] != b'Y'
    //     || input[*i + 3] != b'+'
    // {
    //     unreachable!();
    // }
    *i += 4;

    a_y += (input[*i] - b'0') as i64;
    *i += 1;
    while input[*i] != b'\n' {
        a_y *= 10;
        a_y += (input[*i] - b'0') as i64;
        *i += 1;
    }

    // Parse Button B
    // Parse b_x
    // if input[*i] != b'\n'
    //     || input[*i + 1] != b'B'
    //     || input[*i + 2] != b'u'
    //     || input[*i + 3] != b't'
    //     || input[*i + 4] != b't'
    //     || input[*i + 5] != b'o'
    //     || input[*i + 6] != b'n'
    //     || input[*i + 7] != b' '
    //     || input[*i + 8] != b'B'
    //     || input[*i + 9] != b':'
    //     || input[*i + 10] != b' '
    //     || input[*i + 11] != b'X'
    //     || input[*i + 12] != b'+'
    // {
    //     unreachable!();
    // }

    *i += 13;
    b_x += (input[*i] - b'0') as i64;
    *i += 1;
    while input[*i] != b',' {
        b_x *= 10;
        b_x += (input[*i] - b'0') as i64;
        *i += 1;
    }

    // Parse b_y
    // if input[*i] != b','
    //     || input[*i + 1] != b' '
    //     || input[*i + 2] != b'Y'
    //     || input[*i + 3] != b'+'
    // {
    //     unreachable!();
    // }
    *i += 4;

    b_y += (input[*i] - b'0') as i64;
    *i += 1;
    while input[*i] != b'\n' {
        b_y *= 10;
        b_y += (input[*i] - b'0') as i64;
        *i += 1;
    }

    // Parse the Prize
    // Parse p_x
    // if input[*i] != b'\n'
    //     || input[*i + 1] != b'P'
    //     || input[*i + 2] != b'r'
    //     || input[*i + 3] != b'i'
    //     || input[*i + 4] != b'z'
    //     || input[*i + 5] != b'e'
    //     || input[*i + 6] != b':'
    //     || input[*i + 7] != b' '
    //     || input[*i + 8] != b'X'
    //     || input[*i + 9] != b'='
    // {
    //     unreachable!();
    // }

    *i += 10;
    p_x += (input[*i] - b'0') as i64;
    *i += 1;
    while input[*i] != b',' {
        p_x *= 10;
        p_x += (input[*i] - b'0') as i64;
        *i += 1;
    }

    // Parse p_y
    // if input[*i] != b','
    //     || input[*i + 1] != b' '
    //     || input[*i + 2] != b'Y'
    //     || input[*i + 3] != b'='
    // {
    //     unreachable!();
    // }
    *i += 4;

    p_y += (input[*i] - b'0') as i64;
    *i += 1;
    while *i < input.len() && input[*i] != b'\n' {
        p_y *= 10;
        p_y += (input[*i] - b'0') as i64;
        *i += 1;
    }

    // if *i < input.len() - 1 && (input[*i] != b'\n' || input[*i + 1] != b'\n') {
    //     unreachable!();
    // }
    *i += 2;

    if !CHECK_LIMIT {
        p_x += UNIT_CONVERSION_ERROR;
        p_y += UNIT_CONVERSION_ERROR;
    }

    // A costs 3 tokens.
    // B costs 1 token.

    // num_a * a_x + num_b * b_x                     = p_x
    // num_a * a_y + num_b * b_y                     = p_y
    //
    // 0           + num_b * (b_y * a_x - b_x * a_y) = p_y * a_x - p_x * a_y
    //
    // num_b = (a_x * p_y - a_y * p_x) / (a_x * b_y  - a_y * b_x)

    let numerator_b = a_x * p_y - a_y * p_x;
    let denominator_b = a_x * b_y - a_y * b_x;
    let num_b = numerator_b / denominator_b;
    if numerator_b % denominator_b != 0 {
        return 0;
    }
    if CHECK_LIMIT && num_b >= 100 {
        return 0;
    }

    let x_diff = p_x - (num_b * b_x);
    let num_a = x_diff / a_x;
    if x_diff % a_x != 0 {
        return 0;
    }
    if CHECK_LIMIT && num_a >= 100 {
        return 0;
    }
    if num_a * a_y + num_b * b_y != p_y {
        // triple check.
        return 0;
    }

    return num_a * 3 + num_b;
}

fn get_total_cost<const CHECK_LIMIT: bool>(input: &str) -> i64 {
    let input = input.as_bytes();

    let mut sum: i64 = 0;
    let mut i = 0;

    while i < input.len() {
        sum += get_next_cost::<CHECK_LIMIT>(input, &mut i);
    }

    return sum;
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    return get_total_cost::<true>(input);
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    return get_total_cost::<false>(input);
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_example_input() -> String {
        let input_path = "input/2024/examples/day13.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_input() -> String {
        let input_path = "input/2024/day13.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 480)
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 31761)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 90798500745591)
    }
}
