use std::collections::HashSet;

const MIN_T_ID: u16 = (b't' - b'a') as u16 * 26;
const MAX_T_ID: u16 = (b'u' - b'a') as u16 * 26;

struct Graph1 {
    nodes: Vec<Node1>,
}

impl Graph1 {
    #[inline(always)]
    fn new(input: &str) -> Self {
        let input = input.as_bytes();
        let mut i: usize = 0;

        let mut g = Graph1 { nodes: Vec::new() };

        loop {
            let a = (input[i] - b'a') as u16 * 26 + (input[i + 1] - b'a') as u16;
            let b = (input[i + 3] - b'a') as u16 * 26 + (input[i + 4] - b'a') as u16;

            g.add_edge(a, b);
            i += 6;
            if i >= input.len() {
                break;
            }
        }

        for i in 0..g.nodes.len() {
            g.nodes[i].sort();
        }

        return g;
    }

    #[inline(always)]
    fn add_edge(&mut self, a: u16, b: u16) {
        let mut a_index = self.nodes.iter().position(|n| n.id == a);
        let mut b_index = self.nodes.iter().position(|n| n.id == b);
        if a_index.is_none() {
            a_index = Some(self.nodes.len());
            self.nodes.push(Node1::new(a));
        }
        if b_index.is_none() {
            b_index = Some(self.nodes.len());
            self.nodes.push(Node1::new(b));
        }
        let a_index = a_index.unwrap();
        let b_index = b_index.unwrap();

        self.nodes[a_index].add_edge_to(b_index);
        self.nodes[b_index].add_edge_to(a_index);
    }

    #[inline(always)]
    fn is_edge(&self, a_index: usize, b_index: usize) -> bool {
        return self.nodes[a_index].is_edge(&b_index);
    }

    #[inline(always)]
    fn solve_part1(&self) -> u16 {
        let mut output = 0;

        for index in 0..self.nodes.len() {
            output += self.num_incrementing_3_cycles_containing_t(index);
        }

        return output;
    }

    #[inline(always)]
    fn starts_with_t(&self, node_index: usize) -> bool {
        return self.nodes[node_index].starts_with_t;
    }

    #[inline(always)]
    fn num_incrementing_3_cycles_containing_t(&self, start: usize) -> u16 {
        let mut output = 0;
        let is_t = self.starts_with_t(start);

        for i in (0..self.nodes[start].others.len()).rev() {
            let one_step = self.nodes[start].others[i];
            if one_step <= start {
                break;
            }
            let is_t = is_t || self.starts_with_t(one_step);

            for j in (0..self.nodes[one_step].others.len()).rev() {
                let two_step = self.nodes[one_step].others[j];
                if two_step <= one_step {
                    break;
                }
                if !self.is_edge(start, two_step) {
                    continue;
                }

                if is_t || self.starts_with_t(two_step) {
                    output += 1;
                }
            }
        }

        return output;
    }
}

struct Node1 {
    id: u16,
    starts_with_t: bool,
    others: Vec<usize>,
}

impl Node1 {
    #[inline(always)]
    fn new(id: u16) -> Self {
        Node1 {
            id,
            starts_with_t: MIN_T_ID <= id && id < MAX_T_ID,
            others: Vec::with_capacity(13),
        }
    }

    #[inline(always)]
    fn is_edge(&self, check_index: &usize) -> bool {
        return self.others.binary_search(check_index).is_ok();
        // return !self
        //     .others
        //     .iter()
        //     .position(|other_index| *other_index == check_index)
        //     .is_none();
    }

    #[inline(always)]
    fn add_edge_to(&mut self, other_index: usize) {
        if self.is_edge(&other_index) {
            return;
        }
        self.others.push(other_index);
    }

    #[inline(always)]
    fn sort(&mut self) {
        self.others.sort();
    }
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    #[inline(always)]
    fn new(input: &str) -> Self {
        let input = input.as_bytes();
        let mut i: usize = 0;

        let mut g = Graph { nodes: Vec::new() };

        loop {
            let a = (input[i] - b'a') as u16 * 26 + (input[i + 1] - b'a') as u16;
            let b = (input[i + 3] - b'a') as u16 * 26 + (input[i + 4] - b'a') as u16;

            g.add_edge(a, b);
            i += 6;
            if i >= input.len() {
                break;
            }
        }

        return g;
    }

    #[inline(always)]
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

    #[inline(always)]
    fn solve_part2(&self) -> String {
        let mut ids = self.get_ids_of_maximal_complete_subgraph();
        ids.sort();

        let num_connected = ids.len();
        // betting on there being less than 100 nodes in the clique
        let mut array: [u8; 300] = [b','; 300];
        for i in 0..num_connected {
            let node_id = ids[i];
            array[i * 3] = b'a' + (node_id / 26) as u8;
            array[i * 3 + 1] = b'a' + (node_id % 26) as u8;
        }
        return String::from_utf8_lossy(&array[0..(num_connected * 3) - 1]).to_string();
    }

    #[inline(always)]
    fn get_ids_of_maximal_complete_subgraph(&self) -> Vec<u16> {
        let mut best = HashSet::new();
        self.bron_kerbosch2(
            &mut best,
            &mut HashSet::new(),
            HashSet::from_iter(0..self.nodes.len()),
            HashSet::new(),
        );
        return best
            .iter()
            .map(|index| self.nodes[*index].id)
            .collect::<Vec<u16>>();
    }

    // https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
    // algorithm BronKerbosch2(R, P, X) is
    // if P and X are both empty then
    //     report R as a maximal clique
    // choose a pivot vertex u in P ⋃ X
    // for each vertex v in P \ N(u) do
    //     BronKerbosch2(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
    //     P := P \ {v}
    //     X := X ⋃ {v}
    fn bron_kerbosch2(
        &self,
        best: &mut HashSet<usize>,
        r: &mut HashSet<usize>,
        p: HashSet<usize>,
        mut x: HashSet<usize>,
    ) {
        if p.is_empty() {
            if x.is_empty() && best.len() < r.len() {
                // found a better clique!
                *best = r.clone();
            }
            return;
        }
        // thanks to https://github.com/bertptrs/adventofcode/blob/48824288b04bf25c88d2c11a3f9575b74bbe37ed/2018/src/day23.rs#L12-L63
        // now i understand a little bit more what is happening.

        let mut p_clone = p.clone();
        let pivot = *p
            .union(&x)
            .max_by_key(|&&v| self.nodes[v].set.len())
            .unwrap();

        for &v in p.difference(&self.nodes[pivot].set) {
            r.insert(v);
            let p1: HashSet<usize> = p_clone.intersection(&self.nodes[v].set).cloned().collect();
            let x1: HashSet<usize> = x.intersection(&self.nodes[v].set).cloned().collect();
            self.bron_kerbosch2(best, r, p1, x1);

            r.remove(&v);

            p_clone.remove(&v);
            x.insert(v);
        }
    }
}

struct Node {
    id: u16,
    set: HashSet<usize>,
}

impl Node {
    #[inline(always)]
    fn new(id: u16) -> Self {
        Node {
            id,
            set: HashSet::with_capacity(13),
        }
    }

    #[inline(always)]
    fn add_edge_to(&mut self, other_index: usize) {
        self.set.insert(other_index);
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> u16 {
    let g = Graph1::new(input);
    return g.solve_part1();
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
    let g = Graph::new(input);
    return g.solve_part2();
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

    #[test]
    fn part1_example() {
        let g = Graph1::new(&get_example_input());
        assert_eq!(g.solve_part1(), 7);
        assert_eq!(part1(&get_example_input()), 7);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 1077)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), "co,de,ka,ta");
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(
            part2(&get_input()),
            "bc,bf,do,dw,dx,ll,ol,qd,sc,ua,xc,yu,zt"
        )
    }
}
