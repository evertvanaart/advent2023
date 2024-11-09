use std::collections::HashMap;
use std::collections::HashSet;

/* ---------------------------------- Tile ---------------------------------- */

pub enum Tile {
    Empty,
    Wall,
    SlopeL,
    SlopeR,
    SlopeU,
    SlopeD
}

impl Tile {
    fn is_wall(&self) -> bool {
        match self {
            Tile::Wall => true,
            _          => false
        }
    }
}

/* ---------------------------------- Grid ---------------------------------- */

pub struct Grid {
    pub tiles: Vec<Tile>,
    pub rows: isize,
    pub cols: isize
}

impl Grid {
    fn contains(&self, pos: &(isize, isize)) -> bool {
        pos.0 >= 0 && pos.0 < self.rows && pos.1 >= 0 && pos.1 <= self.cols
    }

    fn get_tile(&self, pos: &(isize, isize)) -> &Tile {
        &self.tiles[self.to_index(pos)]
    }

    fn to_index(&self, pos: &(isize, isize)) -> usize {
        (pos.0 * self.cols + pos.1) as usize
    }

    fn is_valid_slope(&self, pos: &(isize, isize), candidate: &(isize, isize)) -> bool {
        match self.get_tile(pos) {
            Tile::SlopeL => candidate.1 < pos.1,
            Tile::SlopeR => candidate.1 > pos.1,
            Tile::SlopeU => candidate.0 < pos.0,
            Tile::SlopeD => candidate.0 > pos.0,
            _ => true
        }
    }

    fn count_open_neighbors(&self, row: isize, col: isize) -> usize {
        let n: usize =  if self.get_tile(&(row - 1, col)).is_wall() { 0 } else { 1 };
        let s: usize =  if self.get_tile(&(row + 1, col)).is_wall() { 0 } else { 1 };
        let w: usize =  if self.get_tile(&(row, col - 1)).is_wall() { 0 } else { 1 };
        let e: usize =  if self.get_tile(&(row, col + 1)).is_wall() { 0 } else { 1 };
        n + s + w + e
    }

    fn find_nodes(&self) -> Vec<(isize, isize)> {
        let mut nodes: Vec<(isize, isize)> = vec!();

        for row in 1 .. self.rows - 1 {
            for col in 1 .. self.cols - 1 {
                if self.get_tile(&(row, col)).is_wall() {
                    continue;
                }

                if self.count_open_neighbors(row, col) > 2 {
                    nodes.push((row, col));
                }
            }
        }

        nodes
    }

    fn get_valid_neighbors(&self, pos: &(isize, isize), visited: &HashSet<(isize, isize)>) -> Vec<(isize, isize)> {
        let candidates: Vec<(isize, isize)> = vec!(
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1)
        );

        candidates.into_iter()
            .filter(|c| !visited.contains(c))
            .filter(|c| self.contains(c))
            .filter(|c| !self.get_tile(c).is_wall())
            .filter(|c| self.is_valid_slope(pos, c))
            .collect()
    }
}

/* ---------------------------------- Path ---------------------------------- */

pub struct Path {
    pub from: usize,
    pub to: usize,
    pub steps: usize
}

impl Path {
    fn new(from: usize, to: usize, steps: usize) -> Path {
        Path { from, to, steps }
    }
}

fn find_path(
    grid: &Grid,
    from_id: usize,
    start_pos: (isize, isize),
    initial_visited: &HashSet<(isize, isize)>,
    nodes_map: &HashMap<(isize, isize), usize>
) -> Option<Path> {
    let mut visited: HashSet<(isize, isize)> = initial_visited.clone();
    let mut position: (isize, isize) = start_pos.clone();
    let mut steps: usize = 1;

    loop {
        if nodes_map.contains_key(&position) {
            break;
        }

        let neighbors: Vec<(isize, isize)> = grid.get_valid_neighbors(&position, &visited);

        if neighbors.len() != 1 {
            return None
        }

        visited.insert(position);
        position = neighbors[0];
        steps += 1;
    }

    Some(Path::new(from_id, nodes_map[&position], steps))
}

fn find_paths(
    grid: &Grid,
    from_id: usize,
    from_pos: (isize, isize),
    nodes_map: &HashMap<(isize, isize), usize>
) -> Vec<Path> {
    let initial_visited: HashSet<(isize, isize)> = HashSet::from([from_pos]);
    let initial_neighbors: Vec<(isize, isize)> = grid.get_valid_neighbors(&from_pos, &initial_visited);

    initial_neighbors.into_iter()
        .map(|start_pos| find_path(
            grid,
            from_id,
            start_pos,
            &initial_visited,
            nodes_map
        )).filter_map(|v| v).collect()
}

/* ---------------------------------- Graph --------------------------------- */

pub struct Graph {
    pub nodes: Vec<Vec<Path>>,
    pub start_id: usize,
    pub target_id: usize
}

impl Graph {
    pub fn create(grid: &Grid, start: (isize, isize), target: (isize, isize)) -> Graph {
        let mut nodes: Vec<(isize, isize)> = grid.find_nodes();
        nodes.push(start);
        nodes.push(target);

        let nodes_map: HashMap<(isize, isize), usize> = nodes.iter().enumerate()
            .map(|(i, v)| (*v, i)).collect();

        let output_nodes: Vec<Vec<Path>> = nodes.into_iter()
            .map(|pos| find_paths(
                grid, 
                nodes_map[&pos], 
                pos,
                &nodes_map
            )).collect();

        let start_id: usize = nodes_map[&start];
        let target_id: usize = nodes_map[&target];

        Graph {
            nodes: output_nodes,
            start_id: start_id,
            target_id: target_id
        }
    }
}

/* ------------------------------- Main logic ------------------------------- */

pub fn find_longest_path(graph: &Graph, current_node_id: usize, visited: Vec<bool>) -> Option<usize> {
    if current_node_id == graph.target_id {
        return Some(0);
    }

    graph.nodes[current_node_id].iter()
        .filter(|path| !visited[path.to])
        .map(|path| {
            let mut new_visited: Vec<bool> = visited.clone();
            new_visited[current_node_id] = true;

            find_longest_path(graph, path.to, new_visited)
                .map(|best| best + path.steps)
        }).max().unwrap_or(None)
}
