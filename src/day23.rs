struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let input = input.as_bytes();
        let mut i: usize = 0;

        let mut g = Graph { nodes: Vec::new() };

        loop {
            let a = (input[0] - b'a') as u16 * 26 + (input[1] - b'a') as u16;
            let b = (input[3] - b'a') as u16 * 26 + (input[4] - b'a') as u16;

            g.add_edge(a, b);
            i += 6;
            if i >= input.len() {
                break;
            }
        }

        return g;
    }

    fn add_edge(&mut self, a: u16, b: u16) {
        let mut a_index = self.nodes.iter().position(|n| n.id == a);
        let mut b_index = self.nodes.iter().position(|n| n.id == b);
        if a_index.is_none() {
            a_index = Some(self.nodes.len());
            self.nodes.push(Node::new(a));
        }
        if b_index.is_none() {
            b_index = Some(self.nodes.len());
            self.nodes.push(Node::new(b));
        }
        let a_index = a_index.unwrap();
        let b_index = b_index.unwrap();

        self.nodes[a_index].add_edge_to(b_index);
        self.nodes[b_index].add_edge_to(a_index);
    }

    fn is_edge(&self, a_index: usize, b_index: usize) -> bool {
        return !self.nodes[a_index]
            .others
            .iter()
            .position(|n| *n == b_index)
            .is_none();
    }

    fn num_3_cycles(&self) -> u16 {
        let mut output = 0;

        for index in 0..self.nodes.len() {
            output += self.num_incrementing_3_cycles(index);
        }

        return output;
    }

    fn num_incrementing_3_cycles(&self, start: usize) -> u16 {
        let mut output = 0;

        for i in 0..self.nodes[start].others.len() {
            let one_step = self.nodes[start].others[i];
            if one_step <= start {
                continue;
            }
            for j in 0..self.nodes[one_step].others.len() {
                let two_step = self.nodes[one_step].others[j];
                if two_step <= one_step {
                    continue;
                }
                if self.is_edge(start, two_step) {
                    output += 1;
                }
            }
        }

        return output;
    }
}

struct Node {
    id: u16,
    others: Vec<usize>,
}

impl Node {
    fn new(id: u16) -> Self {
        Node {
            id,
            others: Vec::new(),
        }
    }

    fn add_edge_to(&mut self, other_index: usize) {
        self.others.push(other_index);
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> u16 {
    let g = Graph::new(input);
    return g.num_3_cycles();
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> u32 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day23.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_example_input() -> &'static str {
        return "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
    }

    fn get_example_input_2() -> &'static str {
        return "";
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 7);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 1)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input_2()), 1);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 1)
    }
}
