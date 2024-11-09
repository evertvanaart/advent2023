use crate::solutions::Solution;
use crate::solutions::day23::common::*;

// Exactly the same as the A part; the only difference is that slope characters
// are interpreted as empty space when parsing the grid. Since the graph has
// more connections as a result, the exhaustive search for the longest path
// is significantly slower, but at under a second it is still acceptable.

impl Grid {
    fn parse_b(lines: &Vec<String>) -> Grid {
        let tiles: Vec<Tile> = lines.concat().chars().map(|c| {
            match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                '<' => Tile::Empty,
                '>' => Tile::Empty,
                'v' => Tile::Empty,
                '^' => Tile::Empty,
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
    let grid: Grid = Grid::parse_b(lines);
    let start_pos: (isize, isize) = (0, 1);
    let target_pos: (isize, isize) = (grid.rows - 1, grid.cols - 2);
    let graph: Graph = Graph::create(&grid, start_pos, target_pos);
    let visited: Vec<bool> = vec![false; graph.nodes.len()];

    let result: Option<usize> = find_longest_path(&graph, graph.start_id, visited);

    return Solution::Integer(result.unwrap() as i64)
}
