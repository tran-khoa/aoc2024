use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph {
    adjs: HashMap<usize, HashSet<usize>>,
    nodes: Vec<String>,
}
impl Graph {
    fn from_string(input: &str) -> Self {
        let edges: Vec<(String, String)> = input
            .lines()
            .map(|line| {
                let mut parts = line.split("-");
                (
                    parts.next().unwrap().to_string(),
                    parts.next().unwrap().to_string(),
                )
            })
            .collect();
        let nodes: Vec<String> = edges
            .iter()
            .flat_map(|(a, b)| [a.clone(), b.clone()])
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        let mut adjs: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (a, b) in edges {
            let a_idx = nodes.iter().position(|n| n == &a).unwrap();
            let b_idx = nodes.iter().position(|n| n == &b).unwrap();
            adjs.entry(a_idx).or_default().insert(b_idx);
            adjs.entry(b_idx).or_default().insert(a_idx);
        }
        Graph { adjs, nodes }
    }

    fn degree_sort_nodes(&self) -> Self {
        // sort by degree, highest first
        let mut sorted_nodes: Vec<_> = self.nodes.iter().enumerate().collect();
        sorted_nodes.sort_by(|(a_idx, _), (b_idx, _)| {
            let a_degree = self.adjs[a_idx].len();
            let b_degree = self.adjs[b_idx].len();
            a_degree.cmp(&b_degree)
        });
        let mut map_idx_old_new: Vec<usize> = vec![0; self.nodes.len()];
        for (new_idx, (old_idx, _)) in sorted_nodes.iter().enumerate() {
            map_idx_old_new[*old_idx] = new_idx;
        }
        let new_adjs: HashMap<usize, HashSet<usize>> = self
            .adjs
            .iter()
            .map(|(k, vs)| {
                (
                    map_idx_old_new[*k],
                    vs.iter().map(|v| map_idx_old_new[*v]).collect(),
                )
            })
            .collect();
        let new_nodes = sorted_nodes.iter().map(|&(_, n)| n.clone()).collect();
        Graph {
            adjs: new_adjs,
            nodes: new_nodes,
        }
    }

    fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
}

fn part1(graph: &Graph) -> usize {
    // Implements Chiba & Nishizeki algorithm (1985)
    // O(m^(3/2)) time complexity, where m is the number of edges
    let sorted_graph = graph.degree_sort_nodes();
    let mut triangles_with_t = 0usize;
    for node in 0..graph.num_nodes() {
        let neighbors: &HashSet<usize> = &sorted_graph.adjs[&node];
        for neighbor in neighbors.iter().filter(|&&n| n > node) {
            for neighbor_neighbor in sorted_graph.adjs[neighbor].iter().filter(|&&n| n > node) {
                if neighbors.contains(neighbor_neighbor) {
                    // triangle, but does it qualify?
                    if graph.nodes[node].starts_with('t')
                        || graph.nodes[*neighbor].starts_with('t')
                        || graph.nodes[*neighbor_neighbor].starts_with('t')
                    {
                        triangles_with_t += 1;
                    }
                }
            }
        }
    }

    // since we ignore previous nodes, we need to divide only by equivalent permutations
    // e.g. ka co ta is the same as ka ta co, but co ka ta cannot occur
    triangles_with_t / 2
}

fn bron_kerbosch(
    graph: &Graph,
    current: HashSet<usize>,
    mut potential: HashSet<usize>,
    mut excluded: HashSet<usize>,
) -> Vec<HashSet<usize>> {
    // Implements Bron-Kerbosch algorithm (1973)
    // O(3^(n/3)) time complexity, where n is the number of nodes
    // It maintains sets R, P, X, where
    //  R is the current clique,
    //  P is the potential clique,
    //  X is the excluded nodes
    // In each recursive call, as we add some node to R,
    // the potential cliques are reduced to nodes that are connected to the added node
    // i.e. by induction every node in P is connected to every node in R
    if potential.is_empty() && excluded.is_empty() {
        return vec![current];
    }

    let mut cliques = Vec::new();
    for pnode in potential.clone().iter() {
        let mut new_current = current.clone();
        new_current.insert(*pnode);
        let new_potential: HashSet<_> = potential
            .intersection(&graph.adjs[pnode])
            .cloned()
            .collect();
        let new_excluded: HashSet<_> = excluded.intersection(&graph.adjs[pnode]).cloned().collect();
        cliques.extend(bron_kerbosch(
            graph,
            new_current,
            new_potential,
            new_excluded,
        ));
        potential.remove(pnode);
        excluded.insert(*pnode);
    }
    cliques
}

fn part2(graph: &Graph) -> String {
    let maximal_cliques = bron_kerbosch(
        graph,
        HashSet::new(),
        (0..graph.num_nodes()).collect(),
        HashSet::new(),
    );
    let maximum_clique = maximal_cliques
        .iter()
        .max_by_key(|clique| clique.len())
        .unwrap();
    let mut computers: Vec<_> = maximum_clique
        .iter()
        .map(|idx| graph.nodes[*idx].clone())
        .collect();
    computers.sort();
    computers.join(",")
}

fn main() {
    let inputs = std::fs::read_to_string("../inputs.txt").unwrap();
    let input_graph = Graph::from_string(&inputs);
    println!("Part 1: {}", part1(&input_graph));
    println!("Part 2: {}", part2(&input_graph));
}
