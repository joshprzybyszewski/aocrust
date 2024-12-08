const HEAP_SIZE: usize = 4096;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut i: usize = 0;
    let mut sum: u32 = 0;

    while i < input.len() {
        let mut elem: u32 = 0;
        // yes, I know simd can do this way faster
        while input[i] != b':' {
            // end char is the ":" for the first elem.
            // b':'  = 58
            elem *= 10;
            elem += (input[i] - b'0') as u32;
            i += 1;
        }

        let target: u32 = elem;

        // skip past ": "
        i += 2;

        // ending char is a space or a newline.
        // b'\n' = 10
        // b' '  = 32
        // b'0'  = 48
        // we only expect digits in this loop
        elem = 0;
        while input[i] >= b'0' {
            elem *= 10;
            elem += (input[i] - b'0') as u32;
            i += 1;
        }

        if input[i] == b'\n' {
            // handle the case where there's one input param.
            // the heap below is overkill (and incorrect).
            if target == elem {
                sum += target;
            }
            // skip past the newline
            i += 1;

            continue;
        }
        // skip past the space
        i += 1;

        let mut heap: [u32; HEAP_SIZE] = [0; HEAP_SIZE];
        // set the first element
        heap[0] = elem;

        let mut min_h_i: usize = 0;
        let mut max_h_i: usize = 2;
        //                   [0]
        //         [1]                 [2]
        //    [3]       [4]       [5]       [6]
        // [ 7] [ 8] [ 9] [10] [11] [12] [13] [14]

        // get the rest of the line, filling the heap.
        loop {
            // get next elem
            elem = 0;
            while input[i] >= b'0' {
                elem *= 10;
                elem += (input[i] - b'0') as u32;
                i += 1;
            }

            // min = 2 ^ e_i - 1
            // max = 2 ^ (e_i+1) - 2
            let mut h_i = min_h_i + 1;
            while h_i < max_h_i {
                let prev = heap[(h_i - 1) / 2];
                if prev == 0 || prev <= target {
                    heap[h_i] = prev * elem;
                    heap[h_i + 1] = prev + elem;
                }
                h_i += 2;
            }

            if input[i] == b'\n' {
                break;
            }
            // iterate past the space
            i += 1;

            min_h_i = max_h_i;
            // i'm sure there's some magic to make this faster too.
            max_h_i = (max_h_i + 2) * 2 - 2;
        }
        // iterate past the newline
        i += 1;

        for h_i in min_h_i..max_h_i {
            if target == heap[h_i] {
                sum += target;
                break;
            }
        }
    }

    return sum;
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_example_input() -> String {
        let input_path = "input/2024/examples/day7.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_input() -> String {
        let input_path = "input/2024/day7.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), 0);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 0)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 0)
    }
}
