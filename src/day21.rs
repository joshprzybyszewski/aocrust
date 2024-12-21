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

*/

const SHORTEST_KEYBOARD_PATHS: [[[usize; MAX_PATH_LENGTH]; NUM_ARROWS]; NUM_ARROWS] =
    generate_shortest_keyboard_paths();

const MAX_PATH_LENGTH: usize = 6;

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
    current: usize,
    states: [NumericState; 12],
}

impl NumericKeypad {
    const fn new() -> Self {
        return NumericKeypad {
            current: NUMERIC_A,
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
    current: usize,
    states: [ArrowState; 6],
}

impl ArrowKeypad {
    const fn new() -> Self {
        return ArrowKeypad {
            current: ARROW_A,
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
    directions: [usize; MAX_PATH_LENGTH],
    steps: usize,
    position: usize,
}

impl Path {
    const fn new() -> Self {
        return Path {
            directions: [0; MAX_PATH_LENGTH],
            steps: 0,
            position: ARROW_INVALID,
        };
    }

    const fn add(&self, direction: usize, dest: usize) -> Self {
        let mut copy = *self;
        copy.directions[copy.steps] = direction;
        copy.steps += 1;
        copy.position = dest;
        return copy;
    }

    const fn last(&self) -> usize {
        self.position
    }
}

const fn generate_shortest_keyboard_paths() -> [[[usize; MAX_PATH_LENGTH]; NUM_ARROWS]; NUM_ARROWS]
{
    let mut answer: [[[usize; MAX_PATH_LENGTH]; NUM_ARROWS]; NUM_ARROWS] =
        [[[0; MAX_PATH_LENGTH]; NUM_ARROWS]; NUM_ARROWS];

    let mut start = 0;
    loop {
        let mut end = 0;
        loop {
            answer[start][end] = get_shortest_path_between_arrows(start, end);

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

const fn get_shortest_path_between_arrows(start: usize, end: usize) -> [usize; MAX_PATH_LENGTH] {
    if start == end || start == ARROW_INVALID || end == ARROW_INVALID {
        return [ARROW_INVALID; MAX_PATH_LENGTH];
    }

    let keyboard = ArrowKeypad::new();
    let mut seen: [bool; NUM_ARROWS] = [false; NUM_ARROWS];
    let mut pending: [Path; NUM_ARROWS * NUM_ARROWS] = [Path::new(); NUM_ARROWS * NUM_ARROWS];
    let mut index = 0;
    let mut pending_index = 1;

    pending[index].position = start;

    loop {
        let position = pending[index].last();
        if position == end {
            return pending[pending_index].directions;
        }

        if !seen[position] {
            seen[position] = true;

            let mut direction = 1;
            loop {
                if keyboard.states[position].next[direction] != ARROW_INVALID {
                    pending[pending_index] =
                        pending[index].add(direction, keyboard.states[position].next[direction]);
                    pending_index += 1;
                }
                direction += 1;
                if direction == ARROW_A {
                    break;
                }
            }
        }

        index += 1;
        if index >= pending_index || index >= pending.len() {
            unreachable!();
        }
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> u64 {
    let numeric = NumericKeypad::new();
    let robot1 = ArrowKeypad::new();
    let robot2 = ArrowKeypad::new();
    // let robot2 = ArrowKeypad::new();
    return 0;
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
        return "";
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 1)
    }

    #[test]
    fn part1_real_input() {
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
