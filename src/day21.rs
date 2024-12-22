/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

*/

const SHORTEST_ARROW_PATHS: [[[Path; MAX_SHORTEST_ARROW_PATHS]; NUM_ARROWS]; NUM_ARROWS] =
    generate_shortest_keyboard_paths();

const SHORTEST_NUMERIC_PATHS: [[[Path; MAX_SHORTEST_NUMERIC_PATHS]; NUM_NUMERICS]; NUM_NUMERICS] =
    generate_shortest_numeric_paths();

const MAX_PATH_LENGTH: usize = 10;
const MAX_SHORTEST_ARROW_PATHS: usize = 2;
const MAX_SHORTEST_NUMERIC_PATHS: usize = 10;

const ARROW_INVALID: usize = 0;
const ARROW_UP: usize = 1;
const ARROW_LEFT: usize = 2;
const ARROW_DOWN: usize = 3;
const ARROW_RIGHT: usize = 4;
const ARROW_A: usize = 5;
const NUM_ARROWS: usize = 6;

const NUMERIC_INVALID: usize = 0;
const NUMERIC_A: usize = 1;
const NUMERIC_0: usize = 2;
const NUMERIC_3: usize = 3;
const NUMERIC_2: usize = 4;
const NUMERIC_1: usize = 5;
const NUMERIC_6: usize = 6;
const NUMERIC_4: usize = 7;
const NUMERIC_5: usize = 8;
const NUMERIC_7: usize = 9;
const NUMERIC_8: usize = 10;
const NUMERIC_9: usize = 11;
const NUM_NUMERICS: usize = 12;

struct NumericKeypad {
    states: [NumericState; 12],
}

impl NumericKeypad {
    const fn new() -> Self {
        return NumericKeypad {
            states: [
                // NUMERIC_INVALID,
                NumericState {
                    next: [NUMERIC_INVALID; 5],
                },
                // A -> 0, 3
                NumericState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        NUMERIC_INVALID,
                        NUMERIC_3,
                        NUMERIC_0,
                        NUMERIC_INVALID,
                        NUMERIC_INVALID,
                    ],
                },
                // 0 -> 2, A
                NumericState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        NUMERIC_INVALID,
                        NUMERIC_2,
                        NUMERIC_INVALID,
                        NUMERIC_INVALID,
                        NUMERIC_A,
                    ],
                },
                // 3 -> 6, 2,A
                NumericState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        NUMERIC_INVALID,
                        NUMERIC_6,
                        NUMERIC_2,
                        NUMERIC_A,
                        NUMERIC_INVALID,
                    ],
                },
                // 2 -> 5, 1, 0, 3
                NumericState {
                    // ARROW_INVALID, up, left, down, right
                    next: [NUMERIC_INVALID, NUMERIC_5, NUMERIC_1, NUMERIC_0, NUMERIC_3],
                },
                // 1 -> 4, 2
                NumericState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        NUMERIC_INVALID,
                        NUMERIC_4,
                        NUMERIC_INVALID,
                        NUMERIC_INVALID,
                        NUMERIC_2,
                    ],
                },
                // 6 -> 9, 5, 3
                NumericState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        NUMERIC_INVALID,
                        NUMERIC_9,
                        NUMERIC_5,
                        NUMERIC_3,
                        NUMERIC_INVALID,
                    ],
                },
                // 4 -> 7, 1, 5
                NumericState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        NUMERIC_INVALID,
                        NUMERIC_7,
                        NUMERIC_INVALID,
                        NUMERIC_1,
                        NUMERIC_5,
                    ],
                },
                // 5 -> 8, 4, 2, 6
                NumericState {
                    // ARROW_INVALID, up, left, down, right
                    next: [NUMERIC_INVALID, NUMERIC_8, NUMERIC_4, NUMERIC_2, NUMERIC_6],
                },
                // 7 -> 4, 8
                NumericState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        NUMERIC_INVALID,
                        NUMERIC_INVALID,
                        NUMERIC_INVALID,
                        NUMERIC_4,
                        NUMERIC_8,
                    ],
                },
                // 8 -> 7, 5, 9
                NumericState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        NUMERIC_INVALID,
                        NUMERIC_INVALID,
                        NUMERIC_7,
                        NUMERIC_5,
                        NUMERIC_9,
                    ],
                },
                // 9 -> 8, 6
                NumericState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        NUMERIC_INVALID,
                        NUMERIC_INVALID,
                        NUMERIC_8,
                        NUMERIC_6,
                        NUMERIC_INVALID,
                    ],
                },
            ],
        };
    }
}

struct NumericState {
    next: [usize; 5],
}

struct ArrowKeypad {
    states: [ArrowState; 6],
}

impl ArrowKeypad {
    const fn new() -> Self {
        return ArrowKeypad {
            states: [
                // Arrow_INVALID,
                ArrowState {
                    next: [ARROW_INVALID; 5],
                },
                // up -> down, a
                ArrowState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        ARROW_INVALID,
                        ARROW_INVALID,
                        ARROW_INVALID,
                        ARROW_DOWN,
                        ARROW_A,
                    ],
                },
                // left -> down
                ArrowState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        ARROW_INVALID,
                        ARROW_INVALID,
                        ARROW_INVALID,
                        ARROW_INVALID,
                        ARROW_DOWN,
                    ],
                },
                // down ->
                ArrowState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        ARROW_INVALID,
                        ARROW_UP,
                        ARROW_LEFT,
                        ARROW_INVALID,
                        ARROW_RIGHT,
                    ],
                },
                // right ->
                ArrowState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        ARROW_INVALID,
                        ARROW_A,
                        ARROW_DOWN,
                        ARROW_INVALID,
                        ARROW_INVALID,
                    ],
                },
                // a ->
                ArrowState {
                    // ARROW_INVALID, up, left, down, right
                    next: [
                        ARROW_INVALID,
                        ARROW_INVALID,
                        ARROW_UP,
                        ARROW_RIGHT,
                        ARROW_INVALID,
                    ],
                },
            ],
        };
    }
}

struct ArrowState {
    next: [usize; 5],
}

#[derive(Copy, Clone)]
struct Path {
    directions: [usize; MAX_PATH_LENGTH - 1],
    positions: [usize; MAX_PATH_LENGTH],
    steps: usize,
}

impl Path {
    const fn new() -> Self {
        return Path {
            directions: [0; MAX_PATH_LENGTH - 1],
            positions: [0; MAX_PATH_LENGTH],
            steps: 0,
        };
    }

    const fn add(&self, direction: usize, dest: usize) -> Self {
        let mut copy = *self;
        copy.directions[copy.steps] = direction;
        copy.steps += 1;
        copy.positions[copy.steps] = dest;
        return copy;
    }

    const fn has_been_to(&self, pos: usize) -> bool {
        let mut i = self.steps;
        loop {
            if self.positions[i] == pos {
                return true;
            }
            if i == 0 {
                return false;
            }
            i -= 1;
        }
    }

    const fn last(&self) -> usize {
        self.positions[self.steps]
    }
}

const fn generate_shortest_keyboard_paths(
) -> [[[Path; MAX_SHORTEST_ARROW_PATHS]; NUM_ARROWS]; NUM_ARROWS] {
    let mut answer: [[[Path; MAX_SHORTEST_ARROW_PATHS]; NUM_ARROWS]; NUM_ARROWS] =
        [[[Path::new(); MAX_SHORTEST_ARROW_PATHS]; NUM_ARROWS]; NUM_ARROWS];

    let mut start = 0;
    loop {
        let mut end = 0;
        loop {
            answer[start][end] = get_all_shortest_path_between_arrows(start, end);

            end += 1;
            if end == NUM_ARROWS {
                break;
            }
        }
        start += 1;
        if start == NUM_ARROWS {
            break;
        }
    }
    return answer;
}

const fn get_all_shortest_path_between_arrows(
    start: usize,
    end: usize,
) -> [Path; MAX_SHORTEST_ARROW_PATHS] {
    let mut output: [Path; MAX_SHORTEST_ARROW_PATHS] = [Path::new(); MAX_SHORTEST_ARROW_PATHS];
    if start == end || start == ARROW_INVALID || end == ARROW_INVALID {
        return output;
    }

    let keyboard = ArrowKeypad::new();
    let mut pending: [Path; NUM_ARROWS * NUM_ARROWS] = [Path::new(); NUM_ARROWS * NUM_ARROWS];
    let mut index = 0;
    let mut pending_index = 1;
    let mut output_index = 0;

    pending[index].positions[0] = start;

    loop {
        if index == pending_index {
            break;
        }
        let path = pending[index];
        if output[0].steps > 0 && path.steps > output[0].steps {
            index += 1;
            continue;
        }
        let position = path.last();
        if position == end {
            // TODO
            // add to output
            output[output_index] = path;
            index += 1;
            output_index += 1;
            continue;
        }

        let mut direction = ARROW_UP;
        loop {
            let pos = keyboard.states[position].next[direction];
            if pos != ARROW_INVALID && !path.has_been_to(pos) {
                pending[pending_index] = pending[index].add(direction, pos);
                pending_index += 1;
            }
            direction += 1;
            if direction == ARROW_A {
                break;
            }
        }

        index += 1;
        if index > pending_index || index >= pending.len() {
            unreachable!();
        }
    }
    return output;
}

const fn generate_shortest_numeric_paths(
) -> [[[Path; MAX_SHORTEST_NUMERIC_PATHS]; NUM_NUMERICS]; NUM_NUMERICS] {
    let mut answer: [[[Path; MAX_SHORTEST_NUMERIC_PATHS]; NUM_NUMERICS]; NUM_NUMERICS] =
        [[[Path::new(); MAX_SHORTEST_NUMERIC_PATHS]; NUM_NUMERICS]; NUM_NUMERICS];

    let mut start = 0;
    loop {
        let mut end = 0;
        loop {
            answer[start][end] = get_shortest_path_between_numbers(start, end);

            end += 1;
            if end == NUM_NUMERICS {
                break;
            }
        }
        start += 1;
        if start == NUM_NUMERICS {
            break;
        }
    }
    return answer;
}

const fn get_shortest_path_between_numbers(
    start: usize,
    end: usize,
) -> [Path; MAX_SHORTEST_NUMERIC_PATHS] {
    let mut output = [Path::new(); MAX_SHORTEST_NUMERIC_PATHS];
    if start == end || start == NUMERIC_INVALID || end == NUMERIC_INVALID {
        return output;
    }

    let keyboard = NumericKeypad::new();
    let mut pending: [Path; MAX_SHORTEST_NUMERIC_PATHS * MAX_SHORTEST_NUMERIC_PATHS] =
        [Path::new(); MAX_SHORTEST_NUMERIC_PATHS * MAX_SHORTEST_NUMERIC_PATHS];
    let mut index = 0;
    let mut pending_index = 1;
    let mut output_index = 0;

    pending[index].positions[0] = start;

    loop {
        if index == pending_index {
            break;
        }
        let path = pending[index];
        if output[0].steps > 0 && path.steps > output[0].steps {
            break;
        }

        let position = path.last();
        if position == end {
            output[output_index] = path;
            output_index += 1;
            index += 1;
            continue;
        }

        let mut direction = ARROW_UP;
        loop {
            let pos = keyboard.states[position].next[direction];
            if pos != NUMERIC_INVALID && !path.has_been_to(pos) {
                pending[pending_index] = pending[index].add(direction, pos);
                pending_index += 1;
            }
            direction += 1;
            if direction == ARROW_A {
                break;
            }
        }

        index += 1;
        if index > pending_index || index >= pending.len() {
            unreachable!();
        }
    }
    return output;
}

const fn get_min_cost_of_numeric_path(path: Path) -> u64 {
    let mut i = 0;
    let mut best = 0;
    let mut prev = ARROW_A;
    loop {
        let next_pos;
        if i > path.steps {
            break;
        } else if i == path.steps {
            next_pos = ARROW_A;
        } else {
            next_pos = path.directions[i];
        }
        let options = SHORTEST_ARROW_PATHS[prev][next_pos];

        let mut best_step = u64::MAX;
        if options[0].steps != 0 {
            let mut option_index = 0;

            loop {
                if option_index >= options.len() || options[option_index].steps == 0 {
                    break;
                }

                let cost = get_min_cost_of_key_pad_1_path(options[option_index]);
                if cost < best_step {
                    best_step = cost;
                }
                option_index += 1;
            }
            if best_step == u64::MAX {
                unreachable!();
            }
        } else {
            // just push A!
            best_step = 1;
        }

        best += best_step;
        prev = next_pos;
        i += 1;
    }
    //
    return best;
}

const fn get_min_cost_of_key_pad_1_path(path: Path) -> u64 {
    if path.steps == 0 {
        // just push A on my keypad!
        return 1;
    }

    let mut i = 0;
    let mut total = 0;
    let mut prev = ARROW_A;
    loop {
        let next_pos;
        if i > path.steps {
            break;
        } else if i == path.steps {
            next_pos = ARROW_A;
        } else {
            next_pos = path.directions[i];
        }
        let options = SHORTEST_ARROW_PATHS[prev][next_pos];

        let mut option_index = 0;
        loop {
            if option_index >= options.len() || options[option_index].steps == 0 {
                break;
            }
            total += get_min_cost_of_key_pad_2_path(options[option_index]);
            option_index += 1;
        }
        prev = next_pos;
        i += 1;
    }

    return total;
}

const fn get_min_cost_of_key_pad_2_path(path: Path) -> u64 {
    // all of the third keypad steps to get there + 1 for the A.
    return path.steps as u64;
}

fn arrow_to_byte(arrow: usize) -> &'static str {
    match arrow {
        ARROW_UP => return "^",
        ARROW_DOWN => return "v",
        ARROW_LEFT => return "<",
        ARROW_RIGHT => return ">",
        ARROW_A => return "A",
        _ => unreachable!(),
    }
}

fn numeric_to_byte(numeric: usize) -> &'static str {
    match numeric {
        NUMERIC_0 => return "0",
        NUMERIC_1 => return "1",
        NUMERIC_2 => return "2",
        NUMERIC_3 => return "3",
        NUMERIC_4 => return "4",
        NUMERIC_5 => return "5",
        NUMERIC_6 => return "6",
        NUMERIC_7 => return "7",
        NUMERIC_8 => return "8",
        NUMERIC_9 => return "9",
        NUMERIC_A => return "A",
        _ => unreachable!(),
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut i: usize = 0;

    let mut total = 0;
    let mut total_sequence_length = 0;
    let mut current_value = 0;

    let mut prev = NUMERIC_A;

    loop {
        if input[i] < b'A' {
            current_value *= 10;
            current_value += (input[i] - b'0') as u64;
        }

        let next_step = convert_to_number(input[i]);
        let mut best = u64::MAX;
        let mut numeric_path_index = 0;
        let all_numeric_paths = SHORTEST_NUMERIC_PATHS[prev][next_step];
        loop {
            if all_numeric_paths[numeric_path_index].steps == 0 {
                break;
            }
            let path = all_numeric_paths[numeric_path_index];
            let cost = get_min_cost_of_numeric_path(path);
            if cost < best {
                best = cost;
            }
            numeric_path_index += 1;
        }
        if best != u64::MAX {
            total_sequence_length += best;
        }
        prev = next_step;
        i += 1;
        if i >= input.len() || input[i] == b'\n' {
            println!("total += {total_sequence_length} * {current_value}");
            total += current_value * total_sequence_length;

            if i >= input.len() {
                break;
            }
            // add to sum
            current_value = 0;
            total_sequence_length = 0;
            i += 1;
            if i >= input.len() {
                break;
            }
        }
    }

    return total;
}

fn convert_to_number(byte: u8) -> usize {
    match byte {
        b'A' => return NUMERIC_A,
        b'0' => return NUMERIC_0,
        b'1' => return NUMERIC_1,
        b'2' => return NUMERIC_2,
        b'3' => return NUMERIC_3,
        b'4' => return NUMERIC_4,
        b'5' => return NUMERIC_5,
        b'6' => return NUMERIC_6,
        b'7' => return NUMERIC_7,
        b'8' => return NUMERIC_8,
        b'9' => return NUMERIC_9,
        _ => {
            unreachable!();
        }
    }
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> u64 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day21.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_example_input() -> &'static str {
        return "029A
980A
179A
456A
379A";
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 126384)
    }

    #[test]
    fn part1_real_input() {
        // 171095 is too low
        // 181357 is too high.
        // 189206
        assert_eq!(part1(&get_input()), 1)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), 1)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 1)
    }
}
