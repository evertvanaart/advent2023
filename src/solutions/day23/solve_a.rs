use crate::solutions::Solution;
use crate::solutions::day23::common::*;

// We convert the input grid into a graph by first looking for all nodes
// (i.e. all empty spaces with three or four adjecent empty spaces, and the
// start and end cells), assigning each node an ID starting from zero, and
// then finding the paths from each node to a connected node, keeping track
// of the number of steps. The output of this initial conversion step is a
// list of nodes, each consisting of a list of paths to adjacant nodes,
// plus the number of steps needed to reach that node.
//
// In the A part, we reject a neighboring cell if the current cell contains
// a slope and the relative position of the neighboring cell is in the wrong
// direction, e.g. if the current cell contains a "V" slope and the neighbor
// is located north of the current cell.
//
// Using this graph as input, we then perform a simple exhaustive recursive
// search. Keeping track of which nodes we've visited (using a vector, since
// we use zero-based node IDs), in each step we recurse to all non-visited 
// neighboring nodes, and take the maximum of the recursed lengths. We stop
// this recursive search when we reach the target node, or if there are no
// valid neighboring nodes left. Despite being exhaustive, this is still
// fairly fast, especially for the A part; the total graph contains only
// 35 nodes, and the slopes limit the number of paths.

impl Grid {
    fn parse_a(lines: &Vec<String>) -> Grid {
        let tiles: Vec<Tile> = lines.concat().chars().map(|c| {
            match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                '<' => Tile::SlopeL,
                '>' => Tile::SlopeR,
                'v' => Tile::SlopeD,
                '^' => Tile::SlopeU,
                _ => panic!()
            }
        }).collect();
    
        Grid {
            tiles: tiles,
            rows: lines.len() as isize,
            cols: lines[0].len() as isize,
        }
    }
}

pub fn solve(lines: &Vec<String>) -> Solution {
    let grid: Grid = Grid::parse_a(lines);
    let start_pos: (isize, isize) = (0, 1);
    let target_pos: (isize, isize) = (grid.rows - 1, grid.cols - 2);
    let graph: Graph = Graph::create(&grid, start_pos, target_pos);
    let visited: Vec<bool> = vec![false; graph.nodes.len()];

    let result: Option<usize> = find_longest_path(&graph, graph.start_id, visited);

    return Solution::Integer(result.unwrap() as i64)
}
