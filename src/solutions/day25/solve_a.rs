use std::collections::HashMap;
use std::collections::HashSet;

use crate::solutions::Solution;

// A slightly simplified version of the Kernighan–Lin algorithm. After parsing
// the graph, we create an initial partition by picking an arbitrary node, and
// doing a breadth-first expansion until we've reached roughly 50% of the nodes;
// the nodes inside this expansion are assigned to one group, and the remaining
// nodes are assigned to a second group (called groups A and B below).
//
// We then calculate the internal and external costs for each node. The internal
// cost is the number of neighboring nodes that are in the same group as the
// current node, the external cost is the number of neighboring nodes in the
// other group. We also calculate the difference between these two costs.
//
// We then perform a series of optimization steps. In each step, we find the
// node with the highest cost difference, and swap its group. After updating
// the cost values of this node and its neighbors accordingly, we check if the
// total cost of the graph is equal to three, in which case we've found our
// optimal partition, and can simply multiply the sizes of the two groups.
//
// I wasn't sure if this would always work, or if the algorithm might get stuck
// in an local optimum. Just in case, I added a clause where the whole solution
// is retried (relying on the sets used during parsing to select a different
// starting node) in case there's no possible improvement and the total cost
// is still more than three. This clause doesn't get used in practice however,
// so it's possible that we will always reach the optimal partition.

/* ---------------------------------- Group --------------------------------- */

#[derive(PartialEq, Copy, Clone)]
enum Group {
    A,
    B
}

impl Group {
    fn other(&self) -> Group {
        match self {
            Group::A => Group::B,
            Group::B => Group::A
        }
    }
}

/* ---------------------------------- Node ---------------------------------- */

struct Node {
    to: Vec<usize>,
    group: Group,
    internal_cost: isize,
    external_cost: isize,
    cost_diff: isize
}

impl Node {
    fn new(to: Vec<usize>) -> Node {
        Node {
            to: to,
            group: Group::A,
            internal_cost: 0,
            external_cost: 0,
            cost_diff: 0
        }
    }
}

/* ---------------------------------- Graph --------------------------------- */

struct Graph {
    nodes: Vec<Node>,
    cost: isize
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: Vec::new(), cost: 0 }
    }

    pub fn parse(lines: &Vec<String>) ->  Graph {
        let mut node_map: HashMap<String, usize> = HashMap::new();
        let mut graph_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    
        for line in lines {
            let (from_str, to_field) = line.split_once(':').unwrap();
            let from_id: usize = Self::get_node_id(&mut node_map, from_str);
    
            let to_strs: Vec<&str> = to_field.trim().split(' ').collect();
            
            to_strs.into_iter()
                .map(|to_str| Self::get_node_id(&mut node_map, to_str))
                .for_each(|to_id| Self::add_links(&mut graph_map, from_id, to_id));
        }

        let mut graph: Graph = Graph::new();
    
        for node_id in 0 .. graph_map.len() {
            let node_id_set: &HashSet<usize> = graph_map.get(&node_id).unwrap();
            let node_id_vec: Vec<usize> = node_id_set.into_iter().cloned().collect();
            let node: Node = Node::new(node_id_vec);
            graph.nodes.push(node);
        }
    
        graph
    }

    fn get_node_id(node_map: &mut HashMap<String, usize>, key: &str) -> usize {
        let next_id: usize = node_map.len();
        *node_map.entry(String::from(key)).or_insert(next_id)
    }
    
    fn add_links(graph_map: &mut HashMap<usize, HashSet<usize>>, from_id: usize, to_id: usize) {
        graph_map.entry(from_id).or_insert(HashSet::new()).insert(to_id);
        graph_map.entry(to_id).or_insert(HashSet::new()).insert(from_id);
    }
}

/* ----------------------------- Initialization ----------------------------- */

fn create_initial_split(graph: &mut Graph) {
    let mut queue: Vec<usize> = vec!(0);
    let mut nr_processed: usize = 0;
    let mut index: usize = 0;

    while index < queue.len() {
        let node_id: usize = queue[index];
        let node: &mut Node = &mut graph.nodes[node_id];

        let not_processed: bool = match node.group {
            Group::A => true,
            Group::B => false
        };

        if not_processed {
            nr_processed += 1;
            node.group = Group::B;
            node.to.iter().for_each(|id| queue.push(*id));
        }

        if nr_processed > graph.nodes.len() / 2 {
            break;
        }

        index += 1;
    }
}

fn compute_initial_costs(graph: &mut Graph) {
    let mut total_cost: isize = 0;

    for node_id in 0 .. graph.nodes.len() {
        let my_group: Group = graph.nodes[node_id].group;

        let same_group_count: isize = graph.nodes[node_id].to.iter().filter(|&to_id| {
            my_group == graph.nodes[*to_id].group
        }).count() as isize;

        let node: &mut Node = &mut graph.nodes[node_id];

        node.internal_cost = same_group_count;
        node.external_cost = node.to.len() as isize - same_group_count;
        node.cost_diff = node.external_cost - node.internal_cost;

        if node.group == Group::A {
            total_cost += node.external_cost;
        }
    }

    graph.cost = total_cost;
}

/* ------------------------------- Main logic ------------------------------- */

fn step(graph: &mut Graph) -> bool {
    let mut max_cost_diff: isize = isize::MIN;
    let mut target_node_id: usize = 0;

    for node_id in 0 .. graph.nodes.len() {
        let node: &Node = &graph.nodes[node_id];

        if node.cost_diff > max_cost_diff {
            max_cost_diff = node.cost_diff;
            target_node_id = node_id;
        }
    }

    if max_cost_diff <= 0 {
        return false;
    }

    let target_node: &mut Node = &mut graph.nodes[target_node_id];
    let current_group: Group = target_node.group;
    graph.cost -= target_node.cost_diff;

    target_node.group = current_group.other();
    target_node.external_cost = target_node.internal_cost;
    target_node.internal_cost = target_node.to.len() as isize - target_node.internal_cost;
    target_node.cost_diff = target_node.external_cost - target_node.internal_cost;
    
    for to_node_id in graph.nodes[target_node_id].to.clone() {
        let to_node: &mut Node = &mut graph.nodes[to_node_id];

        if to_node.group == current_group {
            to_node.external_cost += 1;
            to_node.internal_cost -= 1;
        } else {
            to_node.external_cost -= 1;
            to_node.internal_cost += 1;
        }

        to_node.cost_diff = to_node.external_cost - to_node.internal_cost;
    }

    true
}

fn try_solve(lines: &Vec<String>) -> Option<usize> {
    let mut graph: Graph = Graph::parse(lines);
    create_initial_split(&mut graph);
    compute_initial_costs(&mut graph);

    while graph.cost > 3 {
        let ok: bool = step(&mut graph);

        if !ok {
            println!("Failed attempt");
            return None
        }
    }

    let group_a_size: usize = graph.nodes.iter()
        .filter(|&node| node.group == Group::A).count();
    let group_b_size: usize = graph.nodes.len() - group_a_size;
    Some(group_a_size * group_b_size)
}

pub fn solve(lines: &Vec<String>) -> Solution {
    loop {
        if let Some(result) = try_solve(lines) {
            return Solution::Integer(result as i64);
        }
    }
}

// 543906 too high
