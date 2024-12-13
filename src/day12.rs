use std::collections::VecDeque;

const GRID_SIZE: usize = 140;
// const GRID_SIZE: usize = 4;
// checkerboard. oof
const MAX_REGIONS: usize = GRID_SIZE * GRID_SIZE;
const UNSEEN: usize = MAX_REGIONS + 2;

#[derive(Copy, Clone, Debug)]
struct Region {
    top_left: Coord,

    area: u64,
    perimeter: u64,
    num_corners: u64,
}

impl Region {
    fn new(top_left: Coord) -> Self {
        return Region {
            top_left,
            area: 0,
            perimeter: 0,
            num_corners: 0,
        };
    }

    fn cost_p1(&self) -> u64 {
        // if self.area == 0 {
        //     unreachable!();
        // }
        // if self.perimeter == 0 {
        //     unreachable!();
        // }
        return self.area * self.perimeter;
    }

    fn cost_p2(&self) -> u64 {
        return self.area * self.num_corners;
    }
}

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
    fn up_right(&self) -> Coord {
        return Coord {
            row: self.row - 1,
            col: self.col + 1,
        };
    }

    #[inline(always)]
    fn up_left(&self) -> Coord {
        return Coord {
            row: self.row - 1,
            col: self.col - 1,
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
    fn down_right(&self) -> Coord {
        return Coord {
            row: self.row + 1,
            col: self.col + 1,
        };
    }

    #[inline(always)]
    fn down_left(&self) -> Coord {
        return Coord {
            row: self.row + 1,
            col: self.col - 1,
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
    seen: [[usize; GRID_SIZE + 2]; GRID_SIZE + 2],

    regions: Vec<Region>,
}

// impl Index<Coord> for Garden {
//     type Output = u8;
//     fn index<'a>(&'a self, coord: Coord) -> &'a u8 {
//         return &self.grid[coord.row][coord.col];
//     }
// }

impl Garden {
    fn new(input: &str) -> Self {
        let input = input.as_bytes();
        let mut grid: [[u8; GRID_SIZE + 2]; GRID_SIZE + 2] = [[0; GRID_SIZE + 2]; GRID_SIZE + 2];
        let mut i: usize = 0;
        for c in 1..GRID_SIZE + 1 {
            grid[1][c] = input[i];
            i += 1;
        }
        // if input[i] != b'\n' {
        //     unreachable!();
        // }
        i += 1; // input[i] is a newline

        for r in 2..=GRID_SIZE {
            for c in 1..=GRID_SIZE {
                grid[r][c] = input[i];
                i += 1;
            }
            // if i < input.len() && input[i] != b'\n' {
            //     unreachable!();
            // }
            i += 1; // input[i] is a newline
        }

        return Garden {
            grid: grid,
            seen: [[UNSEEN; GRID_SIZE + 2]; GRID_SIZE + 2],
            regions: Vec::with_capacity(MAX_REGIONS),
        };
    }

    fn next_unseen(&self) -> Option<Coord> {
        if self.regions.is_empty() {
            return Some(Coord::new(1, 1));
        }

        let mut coord = self.regions[self.regions.len() - 1].top_left;
        coord.col += 1;
        while coord.row <= GRID_SIZE {
            while coord.col <= GRID_SIZE {
                if !self.is_seen(coord) {
                    return Some(coord);
                }
                coord.col += 1;
            }
            coord.row += 1;
            coord.col = 1;
        }

        return None;
    }

    fn get_square(&self, coord: Coord) -> u8 {
        return self.grid[coord.row][coord.col];
    }

    fn get_region_id(&self, coord: Coord) -> usize {
        return self.seen[coord.row][coord.col];
    }

    fn num_corners(&self, coord: Coord) -> u64 {
        let region_id = self.get_region_id(coord);
        let region_id_right = self.get_region_id(coord.right());
        let region_id_left = self.get_region_id(coord.left());

        let mut corners = 0;
        if region_id == self.get_region_id(coord.up()) {
            if region_id == region_id_right && region_id != self.get_region_id(coord.up_right()) {
                // up and right, not diagonally though.
                corners += 1;
            }
            if region_id == region_id_left && region_id != self.get_region_id(coord.up_left()) {
                // up and left, not diagonally though.
                corners += 1;
            }
        } else {
            // not up
            if region_id != region_id_right {
                corners += 1;
            }
            if region_id != region_id_left {
                corners += 1;
            }
        }

        if region_id == self.get_region_id(coord.down()) {
            if region_id == region_id_right && region_id != self.get_region_id(coord.down_right()) {
                corners += 1;
            }
            if region_id == region_id_left && region_id != self.get_region_id(coord.down_left()) {
                corners += 1;
            }
        } else {
            // not down
            if region_id != region_id_right {
                corners += 1;
            }
            if region_id != region_id_left {
                corners += 1;
            }
        }

        return corners;
    }

    fn is_seen(&self, coord: Coord) -> bool {
        return self.seen[coord.row][coord.col] != UNSEEN;
    }

    fn see(&mut self, coord: Coord, region_id: usize) {
        self.seen[coord.row][coord.col] = region_id;
    }

    fn cost_p1(&self) -> u64 {
        return self.regions.iter().map(|region| region.cost_p1()).sum();
    }

    fn cost_p2(&mut self) -> u64 {
        for r in 1..=GRID_SIZE {
            for c in 1..=GRID_SIZE {
                let coord = Coord::new(r, c);
                let region_id = self.get_region_id(coord);
                self.regions[region_id].num_corners += self.num_corners(coord);
            }
        }
        return self.regions.iter().map(|region| region.cost_p2()).sum();
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

        let mut queue: VecDeque<Coord> = VecDeque::with_capacity(GRID_SIZE);
        let top_left = start.unwrap();
        queue.push_front(top_left);

        let mut region: Region = Region::new(top_left);
        let region_id: usize = self.regions.len();

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
            self.see(coord, region_id);
            region.area += 1;

            let mine = self.get_square(coord);

            // Look up
            let other = coord.up();
            if self.get_square(other) == mine {
                queue.push_back(other);
            } else {
                region.perimeter += 1;
            }

            // look right
            let other = coord.right();
            if self.get_square(other) == mine {
                queue.push_back(other);
            } else {
                region.perimeter += 1;
            }

            // Look down
            let other = coord.down();
            if self.get_square(other) == mine {
                queue.push_back(other);
            } else {
                region.perimeter += 1;
            }

            // look left
            let other = coord.left();
            if self.get_square(other) == mine {
                queue.push_back(other);
            } else {
                region.perimeter += 1;
            }
        }

        // if region.area == 0 || region.perimeter == 0 {
        //     let coord = start.unwrap();
        //     println!(
        //         "Bad region: {:?} at {:?} with {}",
        //         region, coord, self.grid[coord.row][coord.col]
        //     );
        //     unreachable!();
        // }

        return Some(region);
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> u64 {
    let mut g = Garden::new(input);
    g.fill_all_regions();

    return g.cost_p1();
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> u64 {
    let mut g = Garden::new(input);
    g.fill_all_regions();

    return g.cost_p2();
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
    fn part2_four() {
        assert_eq!(
            part2(
                "AAAA
BBCD
BBCC
EEEC"
            ),
            80
        );
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 978590)
    }
}
