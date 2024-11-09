/* -------------------------------- Direction ------------------------------- */

pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn parse_a(input: &str) -> Direction {
        match input {
            "R" => Direction::East,
            "L" => Direction::West,
            "U" => Direction::North,
            "D" => Direction::South,
            _ => panic!()
        }
    }

    pub fn parse_b(input: &str) -> Direction {
        match input {
            "0" => Direction::East,
            "2" => Direction::West,
            "3" => Direction::North,
            "1" => Direction::South,
            _ => panic!()
        }
    }
}
