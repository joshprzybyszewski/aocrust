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

use std::u64;

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

const ARROW_INVALID: usize = 0;
const ARROW_UP: usize = 1;
const ARROW_LEFT: usize = 2;
const ARROW_DOWN: usize = 3;
const ARROW_RIGHT: usize = 4;
const ARROW_A: usize = 5;
const NUM_ARROWS: usize = 6;

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

const SHORTEST_NUMERIC_PATHS: [[u64; NUM_NUMERICS]; NUM_NUMERICS] =
    generate_shortest_keyboard_costs::<2>();

// #[allow(long_running_const_eval)]
// const SHORTEST_NUMERIC_PATHS_PART_2: [[u64; NUM_NUMERICS]; NUM_NUMERICS] =
//     generate_shortest_keyboard_costs::<25>();

const MAX_PATH_LENGTH: usize = 10;
const MAX_SHORTEST_ARROW_PATHS: usize = 2;
const MAX_SHORTEST_NUMERIC_PATHS: usize = 10;

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

    fn cache_index(&self) -> usize {
        if self.steps > 3 {
            return 625; // 5 * 5 * 5 * 5;
        }
        return self.directions[0] + (self.directions[1] * 5) + (self.directions[2] * 25);
    }

    fn to_string(&self) -> String {
        return self.directions.map(arrow_to_byte).join("");
    }
}

const fn generate_shortest_keyboard_costs<const DEPTH: usize>(
) -> [[u64; NUM_NUMERICS]; NUM_NUMERICS] {
    let mut answer: [[u64; NUM_NUMERICS]; NUM_NUMERICS] = [[u64::MAX; NUM_NUMERICS]; NUM_NUMERICS];

    let mut start = NUMERIC_A;
    loop {
        let mut end = NUMERIC_A;
        loop {
            answer[start][end] = get_shortest_path_between_numerics::<DEPTH>(start, end);

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

const fn get_shortest_path_between_numerics<const DEPTH: usize>(start: usize, end: usize) -> u64 {
    if start == end {
        // press A -> press A -> press A -> press end
        return 1;
    }

    if start == NUMERIC_INVALID || end == NUMERIC_INVALID {
        unreachable!();
    }

    let mut shortest: [Path; MAX_SHORTEST_NUMERIC_PATHS] =
        [Path::new(); MAX_SHORTEST_NUMERIC_PATHS];
    let mut shortest_index = 0;

    let mut pending: [Path; MAX_SHORTEST_NUMERIC_PATHS * MAX_SHORTEST_NUMERIC_PATHS] =
        [Path::new(); MAX_SHORTEST_NUMERIC_PATHS * MAX_SHORTEST_NUMERIC_PATHS];
    let mut pending_index = 1;

    let mut index = 0;

    pending[index].positions[0] = start;

    let keyboard = NumericKeypad::new();

    loop {
        if index == pending_index {
            // completed FIFO queue.
            break;
        }

        let path = pending[index];
        if shortest[0].steps > 0 && path.steps > shortest[0].steps {
            // skip this one, since it's longer than the shortest.
            index += 1;
            // we probably could break here since it's BFS, but let's play it safe.
            continue;
        }

        let position = path.last();
        if position == end {
            // We found a path to the end. let's remember it
            shortest[shortest_index] = path;
            shortest_index += 1;
            index += 1;
            continue;
        }

        // Check every cardinal direction.
        let mut direction = ARROW_UP;
        loop {
            let next_pos = keyboard.states[position].next[direction];
            if next_pos != NUMERIC_INVALID && !path.has_been_to(next_pos) {
                // if we are a valid move, and the current path hasn't cross the number
                // before, let's add this to the FIFO queue.
                pending[pending_index] = path.add(direction, next_pos);
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

    if shortest_index == 0 {
        // there has to be at least one shortest path to check.
        unreachable!();
    }

    let mut best = u64::MAX;

    loop {
        shortest_index -= 1;

        let mine = get_numeric_path_min_cost::<DEPTH>(shortest[shortest_index]);
        if mine < best {
            best = mine;
        }

        if shortest_index == 0 {
            break;
        }
    }

    return best;
}

const fn get_numeric_path_min_cost<const DEPTH: usize>(numeric_path: Path) -> u64 {
    return get_arrow_path_min_cost(DEPTH, numeric_path);
}

const fn get_arrow_path_min_cost(depth: usize, path: Path) -> u64 {
    if depth == 0 {
        // plus one to push A.
        return path.steps as u64 + 1;
    }

    let mut total = 0;
    let mut my_state = ARROW_A;
    let mut i = 0;
    loop {
        let next: usize;
        if i > path.steps {
            break;
        } else if i == path.steps {
            next = ARROW_A;
        } else {
            next = path.directions[i];
        }
        i += 1;

        let (paths, num_paths) = get_shortest_paths_between_arrows(my_state, next);
        let mut path_index = 0;
        if num_paths == 0 {
            unreachable!();
        }

        let mut best_to_next = u64::MAX;
        loop {
            let path_cost = get_arrow_path_min_cost(depth - 1, paths[path_index]);
            path_index += 1;
            if path_cost < best_to_next {
                best_to_next = path_cost;
            }
            if path_index == num_paths {
                break;
            }
        }
        if best_to_next == u64::MAX {
            unreachable!();
        }
        total += best_to_next;

        my_state = next;
    }

    return total;
}

const fn get_shortest_paths_between_arrows(
    start: usize,
    end: usize,
) -> ([Path; MAX_SHORTEST_ARROW_PATHS], usize) {
    if start == end {
        return ([Path::new(); MAX_SHORTEST_ARROW_PATHS], 1);
    }

    if start == ARROW_INVALID || end == ARROW_INVALID {
        unreachable!();
    }

    let mut shortest: [Path; MAX_SHORTEST_ARROW_PATHS] = [Path::new(); MAX_SHORTEST_ARROW_PATHS];
    let mut shortest_index = 0;

    let mut pending: [Path; 16] = [Path::new(); 16];
    let mut pending_index = 1;

    let mut index = 0;

    pending[index].positions[0] = start;

    let keyboard = ArrowKeypad::new();

    loop {
        if index == pending_index {
            // completed FIFO queue.
            break;
        }

        let path = pending[index];
        if shortest[0].steps > 0 && path.steps > shortest[0].steps {
            // skip this one, since it's longer than the shortest.
            index += 1;
            // we probably could break here since it's BFS, but let's play it safe.
            continue;
        }

        let position = path.last();
        if position == end {
            // We found a path to the end. let's remember it
            shortest[shortest_index] = path;
            shortest_index += 1;
            index += 1;
            continue;
        }

        // Check every cardinal direction.
        let mut direction = ARROW_UP;
        loop {
            let next_pos = keyboard.states[position].next[direction];
            if next_pos != NUMERIC_INVALID && !path.has_been_to(next_pos) {
                // if we are a valid move, and the current path hasn't cross the number
                // before, let's add this to the FIFO queue.
                pending[pending_index] = path.add(direction, next_pos);
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

    if shortest_index == 0 {
        // there has to be at least one shortest path to check.
        unreachable!();
    }

    return (shortest, shortest_index);
}

fn arrow_to_byte(arrow: usize) -> &'static str {
    match arrow {
        ARROW_UP => return "^",
        ARROW_DOWN => return "v",
        ARROW_LEFT => return "<",
        ARROW_RIGHT => return ">",
        ARROW_A => return "A",
        ARROW_INVALID => return "",
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
        total_sequence_length += SHORTEST_NUMERIC_PATHS[prev][next_step];

        prev = next_step;
        i += 1;
        if i >= input.len() || input[i] == b'\n' {
            // println!("total += {total_sequence_length} * {current_value}");
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
    // println!("const SHORTEST_NUMERIC_PATHS_PART_2: [[u64; NUM_NUMERICS]; NUM_NUMERICS] = [");
    // for start in 0..NUM_NUMERICS {
    //     print!("[");
    //     for end in 0..NUM_NUMERICS {
    //         let shortest: u64;
    //         if start < NUMERIC_A || end < NUMERIC_A {
    //             shortest = u64::MAX;
    //         } else {
    //             shortest = get_shortest_path_between_numerics::<25>(start, end);
    //         }
    //         print!("{}", shortest); // SHORTEST_NUMERIC_PATHS_PART_2[start][end]);
    //         if end < NUM_NUMERICS -1 {
    //             print!(", ");
    //         }
    //     }
    //     println!("], ");
    // }
    // println!("];");

    let input = input.as_bytes();
    let mut i: usize = 0;

    let mut total = 0;
    let mut total_sequence_length = 0;
    let mut current_value = 0;

    let mut prev = NUMERIC_A;

    let mut cacher = ArrowCacher {
        answers: [[0; 125]; 26],
    };

    loop {
        if input[i] < b'A' {
            current_value *= 10;
            current_value += (input[i] - b'0') as u64;
        }

        let next_step = convert_to_number(input[i]);
        total_sequence_length += cacher.get_shortest_path_between_numerics2::<25>(prev, next_step);
        // total_sequence_length += SHORTEST_NUMERIC_PATHS_PART_2[prev][next_step];

        prev = next_step;
        i += 1;
        if i >= input.len() || input[i] == b'\n' {
            // println!("total += {total_sequence_length} * {current_value}");
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

struct ArrowCacher {
    answers: [[u64; 125]; 26],
}

impl ArrowCacher {
    fn get_shortest_path_between_numerics2<const DEPTH: usize>(
        &mut self,
        start: usize,
        end: usize,
    ) -> u64 {
        if start == end {
            // press A -> press A -> press A -> press end
            return 1;
        }

        if start == NUMERIC_INVALID || end == NUMERIC_INVALID {
            unreachable!();
        }

        let mut shortest: [Path; MAX_SHORTEST_NUMERIC_PATHS] =
            [Path::new(); MAX_SHORTEST_NUMERIC_PATHS];
        let mut shortest_index = 0;

        let mut pending: [Path; MAX_SHORTEST_NUMERIC_PATHS * MAX_SHORTEST_NUMERIC_PATHS] =
            [Path::new(); MAX_SHORTEST_NUMERIC_PATHS * MAX_SHORTEST_NUMERIC_PATHS];
        let mut pending_index = 1;

        let mut index = 0;

        pending[index].positions[0] = start;

        let keyboard = NumericKeypad::new();

        loop {
            if index == pending_index {
                // completed FIFO queue.
                break;
            }

            let path = pending[index];
            if shortest[0].steps > 0 && path.steps > shortest[0].steps {
                // skip this one, since it's longer than the shortest.
                index += 1;
                // we probably could break here since it's BFS, but let's play it safe.
                continue;
            }

            let position = path.last();
            if position == end {
                // We found a path to the end. let's remember it
                shortest[shortest_index] = path;
                shortest_index += 1;
                index += 1;
                continue;
            }

            // Check every cardinal direction.
            let mut direction = ARROW_UP;
            loop {
                let next_pos = keyboard.states[position].next[direction];
                if next_pos != NUMERIC_INVALID && !path.has_been_to(next_pos) {
                    // if we are a valid move, and the current path hasn't cross the number
                    // before, let's add this to the FIFO queue.
                    pending[pending_index] = path.add(direction, next_pos);
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

        if shortest_index == 0 {
            // there has to be at least one shortest path to check.
            unreachable!();
        }

        let mut best = u64::MAX;

        loop {
            shortest_index -= 1;

            let mine = self.get_numeric_path_min_cost2::<DEPTH>(shortest[shortest_index]);
            if mine < best {
                best = mine;
            }

            if shortest_index == 0 {
                break;
            }
        }

        return best;
    }

    fn get_numeric_path_min_cost2<const DEPTH: usize>(&mut self, numeric_path: Path) -> u64 {
        return self.get_arrow_path_min_cost2(DEPTH, numeric_path);
    }

    fn get_arrow_path_min_cost2(&mut self, depth: usize, path: Path) -> u64 {
        let answer_index = path.cache_index();
        if answer_index < 125 && self.answers[depth][answer_index] != 0 {
            return self.answers[depth][answer_index];
        }

        if depth == 0 {
            // plus one to push A.
            if answer_index >= 125 {
                return path.steps as u64 + 1;
            }
            self.answers[depth][answer_index] = path.steps as u64 + 1;
            return self.answers[depth][answer_index];
        }

        let mut total = 0;
        let mut my_state = ARROW_A;
        let mut i = 0;
        loop {
            let next: usize;
            if i > path.steps {
                break;
            } else if i == path.steps {
                next = ARROW_A;
            } else {
                next = path.directions[i];
            }
            i += 1;

            let (paths, num_paths) = get_shortest_paths_between_arrows(my_state, next);
            let mut path_index = 0;
            if num_paths == 0 {
                unreachable!();
            }

            let mut best_to_next = u64::MAX;
            loop {
                let path_cost = self.get_arrow_path_min_cost2(depth - 1, paths[path_index]);
                path_index += 1;
                if path_cost < best_to_next {
                    best_to_next = path_cost;
                }
                if path_index == num_paths {
                    break;
                }
            }
            if best_to_next == u64::MAX {
                unreachable!();
            }
            total += best_to_next;

            my_state = next;
        }

        if answer_index >= 125 {
            return total;
        }
        self.answers[depth][answer_index] = total;
        return self.answers[depth][answer_index];
        // return total;
    }
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
        assert_eq!(part1(&get_input()), 176650)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 217698355426872)
    }
}
