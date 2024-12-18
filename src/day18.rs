use std::collections::VecDeque;

const MAX_GRID_SIZE: usize = 73;

#[derive(Copy, Clone, Debug)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(r: usize, c: usize) -> Self {
        return Coord { row: r, col: c };
    }

    #[inline(always)]
    fn up(&self) -> Coord {
        return Coord {
            row: self.row - 1,
            col: self.col,
        };
    }

    #[inline(always)]
    fn right(&self) -> Coord {
        return Coord {
            row: self.row,
            col: self.col + 1,
        };
    }

    #[inline(always)]
    fn down(&self) -> Coord {
        return Coord {
            row: self.row + 1,
            col: self.col,
        };
    }

    #[inline(always)]
    fn left(&self) -> Coord {
        return Coord {
            row: self.row,
            col: self.col - 1,
        };
    }
}

#[derive(Copy, Clone, Debug)]
struct Step {
    coord: Coord,
    cost: u64,
}

impl Step {
    fn new(coord: Coord, cost: u64) -> Self {
        return Step { coord, cost };
    }

    #[inline(always)]
    fn up(&self) -> Step {
        return Step {
            coord: self.coord.up(),
            cost: self.cost + 1,
        };
    }

    #[inline(always)]
    fn right(&self) -> Step {
        return Step {
            coord: self.coord.right(),
            cost: self.cost + 1,
        };
    }

    #[inline(always)]
    fn down(&self) -> Step {
        return Step {
            coord: self.coord.down(),
            cost: self.cost + 1,
        };
    }

    #[inline(always)]
    fn left(&self) -> Step {
        return Step {
            coord: self.coord.left(),
            cost: self.cost + 1,
        };
    }
}

#[inline(always)]
fn build_input(input: &str, cheapest: &mut [[u64; MAX_GRID_SIZE]; MAX_GRID_SIZE], n: usize) {
    let input = input.as_bytes();

    let mut i: usize = 0;
    let mut r: usize;
    let mut c: usize;
    let mut num_corruptions: usize = 0;

    while num_corruptions < n {
        r = (input[i] - b'0') as usize;
        i += 1;
        while input[i] != b',' {
            r *= 10;
            r += (input[i] - b'0') as usize;
            i += 1;
        }
        i += 1;

        c = (input[i] - b'0') as usize;
        i += 1;
        while input[i] != b'\n' {
            c *= 10;
            c += (input[i] - b'0') as usize;
            i += 1;
        }
        i += 1;
        println!("corruption[{num_corruptions}] at ({}, {})", r + 1, c + 1);
        cheapest[r + 1][c + 1] = 0;
        num_corruptions += 1;
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u64 {
    let maximum = (MAX_GRID_SIZE * MAX_GRID_SIZE * 2) as u64;
    let mut cheapest: [[u64; MAX_GRID_SIZE]; MAX_GRID_SIZE] =
        [[maximum; MAX_GRID_SIZE]; MAX_GRID_SIZE];

    for x in 0..MAX_GRID_SIZE {
        cheapest[x][0] = 0;
        cheapest[x][MAX_GRID_SIZE - 1] = 0;
        cheapest[0][x] = 0;
        cheapest[MAX_GRID_SIZE - 1][x] = 0;
    }

    build_input(input, &mut cheapest, 1024);

    let mut queue: VecDeque<Step> = VecDeque::with_capacity(4 * MAX_GRID_SIZE * MAX_GRID_SIZE);
    queue.push_front(Step::new(Coord::new(1, 1), 0));

    while !queue.is_empty() {
        let step = queue.pop_front().unwrap();
        if cheapest[step.coord.row][step.coord.col] <= step.cost {
            // println!("not cheapest {:?}", step);
            continue;
        }
        // println!("Checking {:?}", step);

        cheapest[step.coord.row][step.coord.col] = step.cost;

        // Look up
        let next = step.up();
        if cheapest[next.coord.row][next.coord.col] > next.cost {
            queue.push_back(next);
        }
        // look right
        let next = step.right();
        if cheapest[next.coord.row][next.coord.col] > next.cost {
            queue.push_back(next);
        }
        // Look down
        let next = step.down();
        if cheapest[next.coord.row][next.coord.col] > next.cost {
            queue.push_back(next);
        }
        // look left
        let next = step.left();
        if cheapest[next.coord.row][next.coord.col] > next.cost {
            queue.push_back(next);
        }
    }

    // 10368 is too high
    return cheapest[MAX_GRID_SIZE - 2][MAX_GRID_SIZE - 2];
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> u64 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day18.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 294)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 1816)
    }
}
