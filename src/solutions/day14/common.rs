pub enum Field {
    Empty,
    RoundRock,
    SquareRock,
}

pub struct Grid {
    pub content: Vec<Field>,
    pub rows: isize,
    pub cols: isize
}

impl Grid {
    pub fn from_lines(lines: &[String]) -> Grid {
        let content: Vec<Field> = lines.concat().chars().map(|c| {
            match c {
                'O' => Field::RoundRock,
                '#' => Field::SquareRock,
                 _  => Field::Empty
            }
        }).collect();

        Grid {
            content: content,
            cols: lines.first().unwrap().len() as isize,
            rows: lines.len() as isize
        }
    }

    pub fn get(&self, c: &(isize, isize)) -> &Field {
        &self.content[self.to_index(c)]
    }

    pub fn set(&mut self, c: &(isize, isize), field: Field) {
        let index: usize = self.to_index(c);
        self.content[index] = field;
    }

    pub fn contains(&self, c: &(isize, isize)) -> bool {
        c.0 >= 0 && c.0 < self.rows && c.1 >= 0 && c.1 < self.cols
    }

    fn to_index(&self, c: &(isize, isize)) -> usize {
        (c.0 * self.cols + c.1) as usize
    }
}
