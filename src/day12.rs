use std::collections::VecDeque;

#[inline(always)]
fn convert_byte(val: u8) -> u8 {
    if val < b'A' {
        return val - b'0';
    }
    if val < b'a' {
        return val - b'A' + 10;
    }
    return val - b'a' + 36;
}

const GRID_SIZE: usize = 140;
// const GRID_SIZE: usize = 10;

#[derive(Copy, Clone, Debug)]
struct Region {
    area: u64,
    perimeter: u64,
}
impl Region {
    fn new() -> Self {
        return Region {
            area: 0,
            perimeter: 0,
        };
    }

    fn cost(&self) -> u64 {
        if self.area == 0 {
            unreachable!();
        }
        if self.perimeter == 0 {
            unreachable!();
        }
        return self.area * self.perimeter;
    }
}

#[derive(Copy, Clone, Debug)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
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

struct Garden {
    grid: [[u8; GRID_SIZE + 2]; GRID_SIZE + 2],
    seen: [[bool; GRID_SIZE + 2]; GRID_SIZE + 2],

    regions: Vec<Region>,
}

impl Garden {
    fn new(input: &str) -> Self {
        let input = input.as_bytes();
        let mut grid: [[u8; GRID_SIZE + 2]; GRID_SIZE + 2] = [[0; GRID_SIZE + 2]; GRID_SIZE + 2];
        let mut i: usize = 0;
        for c in 1..GRID_SIZE + 1 {
            grid[1][c] = convert_byte(input[i]);
            i += 1;
        }
        if input[i] != b'\n' {
            unreachable!();
        }
        i += 1; // input[i] is a newline

        for r in 2..=GRID_SIZE {
            for c in 1..=GRID_SIZE {
                grid[r][c] = convert_byte(input[i]);
                i += 1;
            }
            if i < input.len() && input[i] != b'\n' {
                unreachable!();
            }
            i += 1; // input[i] is a newline
        }

        return Garden {
            grid: grid,
            seen: [[false; GRID_SIZE + 2]; GRID_SIZE + 2],
            regions: Vec::with_capacity(GRID_SIZE * GRID_SIZE / 10),
        };
    }

    fn next_unseen(&self) -> Option<Coord> {
        for r in 1..=GRID_SIZE {
            for c in 1..=GRID_SIZE {
                if !self.is_seen(Coord { row: r, col: c }) {
                    return Some(Coord { row: r, col: c });
                }
            }
        }
        return None;
    }

    fn is_seen(&self, coord: Coord) -> bool {
        return self.seen[coord.row][coord.col];
    }
    fn see(&mut self, coord: Coord) {
        self.seen[coord.row][coord.col] = true;
    }

    fn cost(&self) -> u64 {
        return self.regions.iter().map(|region| region.cost()).sum();
    }

    fn fill_all_regions(&mut self) {
        loop {
            let region = self.fill_region();
            if region.is_none() {
                break;
            }
            self.regions.push(region.unwrap());
        }
    }

    fn fill_region(&mut self) -> Option<Region> {
        let start = self.next_unseen();
        if start.is_none() {
            return None;
        }
        // println!("processing {:?}", start.unwrap());
        let mut queue: VecDeque<Coord> = VecDeque::with_capacity(GRID_SIZE);
        queue.push_front(start.unwrap());

        let mut region: Region = Region::new();

        loop {
            let coord = queue.pop_front();
            if coord.is_none() {
                break;
            }

            let coord = coord.unwrap();
            if coord.row == 0 || coord.col == 0 || coord.row > GRID_SIZE || coord.col > GRID_SIZE {
                // out of bounds
                continue;
            }

            if self.is_seen(coord) {
                continue;
            }
            self.see(coord);
            region.area += 1;

            // Look up
            let other = coord.up();
            if self.grid[other.row][other.col] == self.grid[coord.row][coord.col] {
                queue.push_back(other);
            } else {
                region.perimeter += 1;
            }

            // look right
            let other = coord.right();
            if self.grid[other.row][other.col] == self.grid[coord.row][coord.col] {
                queue.push_back(other);
            } else {
                region.perimeter += 1;
            }

            // Look down
            let other = coord.down();
            if self.grid[other.row][other.col] == self.grid[coord.row][coord.col] {
                queue.push_back(other);
            } else {
                region.perimeter += 1;
            }

            // look left
            let other = coord.left();
            if self.grid[other.row][other.col] == self.grid[coord.row][coord.col] {
                queue.push_back(other);
            } else {
                region.perimeter += 1;
            }
        }
        if region.area == 0 || region.perimeter == 0 {
            let coord = start.unwrap();
            println!(
                "Bad region: {:?} at {:?} with {}",
                region, coord, self.grid[coord.row][coord.col]
            );
        }

        // println!(" has region {:?}", region);
        return Some(region);
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> u64 {
    let mut g = Garden::new(input);
    g.fill_all_regions();

    return g.cost();
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> u64 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day12.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                "AAAA
BBCD
BBCC
EEEC"
            ),
            140
        );
    }

    #[test]
    fn part1_example_5() {
        assert_eq!(
            part1(
                "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            ),
            772
        );
    }

    #[test]
    fn part1_example_10() {
        assert_eq!(
            part1(
                "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            1930
        );
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 1546338)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 0)
    }
}
