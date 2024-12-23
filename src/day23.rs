use std::collections::{hash_set::Union, HashSet};

const MIN_T_ID: u16 = (b't' - b'a') as u16 * 26;
const MAX_T_ID: u16 = (b'u' - b'a') as u16 * 26;

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
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

        for i in 0..g.nodes.len() {
            g.nodes[i].sort();
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
        return self.nodes[a_index].is_edge(b_index);
    }

    fn solve_part2(&self) -> String {
        let best = self.get_maximal_complete_subgraph();
        let mut ids = best
            .iter()
            .map(|index| self.nodes[*index].id)
            .collect::<Vec<u16>>();
        ids.sort();

        let num_connected = best.len();
        let mut array: [u8; 3 * 5000] = [b','; 3 * 5000];
        for i in 0..num_connected {
            // let node_index = best[i];
            let node_id = ids[i];
            array[i * 3] = b'a' + (node_id / 26) as u8;
            array[i * 3 + 1] = b'a' + (node_id % 26) as u8;
        }
        return String::from_utf8_lossy(&array[0..(num_connected * 3) - 1]).to_string();
    }

    fn get_maximal_complete_subgraph(&self) -> Vec<usize> {
        return self
            .bron_keybosch1(
                HashSet::new(),
                HashSet::from_iter(0..self.nodes.len()),
                HashSet::new(),
            )
            .iter()
            .map(|index| *index)
            .collect::<Vec<usize>>();
    }

    // https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
    // algorithm BronKerbosch1(R, P, X) is
    // if P and X are both empty then
    //     report R as a maximal clique
    // for each vertex v in P do
    //     BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
    //     P := P \ {v}
    //     X := X ⋃ {v}
    fn bron_keybosch1(
        &self,
        r: HashSet<usize>,
        p: HashSet<usize>,
        x: HashSet<usize>,
    ) -> HashSet<usize> {
        if p.is_empty() {
            if x.is_empty() {
                return r.clone();
            }
            return HashSet::new();
        }

        let mut x = x.clone();
        let mut best: HashSet<usize> = HashSet::new();

        for v in p.iter() {
            // For each vertex v chosen from P, it makes a recursive call
            // in which v is added to R and in which P and X are restricted
            // to the neighbor set N(v) of v, which finds and reports all
            // clique extensions of R that contain v. Then, it moves v from
            // P to X to exclude it from consideration in future cliques and
            // continues with the next vertex in P.
            let r_union_v =
                HashSet::from_iter(r.union(&HashSet::from([*v])).into_iter().map(|e| *e));
            let v_neighbors: HashSet<usize> =
                HashSet::from_iter(self.nodes[*v].others.iter().map(|e| *e));
            let p_intersection_v_neighbors =
                HashSet::from_iter(p.intersection(&v_neighbors).into_iter().map(|e| *e));
            let x_intersection_v_neighbors =
                HashSet::from_iter(x.intersection(&v_neighbors).into_iter().map(|e| *e));
            let answer = self.bron_keybosch1(
                r_union_v,
                p_intersection_v_neighbors,
                x_intersection_v_neighbors,
            );
            x.insert(*v);
            if answer.is_empty() {
                continue;
            }
            if best.len() < answer.len() {
                best = answer;
            }
        }

        return best;
    }

    fn solve_part1(&self) -> u16 {
        let mut output = 0;

        for index in 0..self.nodes.len() {
            output += self.num_incrementing_3_cycles_containing_t(index);
        }

        return output;
    }

    fn starts_with_t(&self, node_index: usize) -> bool {
        let id = self.nodes[node_index].id;
        return MIN_T_ID <= id && id < MAX_T_ID;
    }

    fn num_incrementing_3_cycles_containing_t(&self, start: usize) -> u16 {
        let mut output = 0;

        for i in (0..self.nodes[start].others.len()).rev() {
            let one_step = self.nodes[start].others[i];
            if one_step <= start {
                break;
            }
            for j in (0..self.nodes[one_step].others.len()).rev() {
                let two_step = self.nodes[one_step].others[j];
                if two_step <= one_step {
                    break;
                }
                if !self.is_edge(start, two_step) {
                    continue;
                }
                if self.starts_with_t(start)
                    || self.starts_with_t(one_step)
                    || self.starts_with_t(two_step)
                {
                    output += 1;
                }
            }
        }

        return output;
    }

    fn print_me(&self) {
        println!("Graph");
        println!(" Nodes: {}", self.nodes.len());
        println!("");
        for index in 0..self.nodes.len() {
            let node = &self.nodes[index];
            println!(
                " nodes[{index}] = {} = {}. Connected to {} others",
                node.id,
                convert_id_to_string(node.id),
                node.others.len()
            );
            print!("   -> ");
            for i in 0..node.others.len() {
                if i > 0 {
                    print!(", ")
                }
                let other_index = node.others[i];
                let other_node = &self.nodes[other_index];
                print!("{}", other_node.id);
            }
            println!("");
        }
    }
}

fn convert_id_to_string(id: u16) -> String {
    String::from_utf8_lossy(&[(id / 26) as u8 + b'a', (id % 26) as u8 + b'a']).to_string()
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

    fn is_edge(&self, check_index: usize) -> bool {
        // TODO position could be improved if we assume that the others are sorted.
        return !self
            .others
            .iter()
            .position(|other_index| *other_index == check_index)
            .is_none();
    }

    fn add_edge_to(&mut self, other_index: usize) {
        if self.is_edge(other_index) {
            return;
        }
        self.others.push(other_index);
    }

    fn sort(&mut self) {
        self.others.sort();
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> u16 {
    let g = Graph::new(input);
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
        let g = Graph::new(&get_example_input());
        g.print_me();
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
        assert_eq!(part2(&get_input()), "")
    }
}
