type Coord = (usize, usize);

#[derive(PartialEq)]
enum Position { Normal, Start, End }

struct Cell {
    elevation: usize,
    position: Position,
}

impl Cell {
    fn get_height(height: u8) -> usize {
        return match height as char {
            'S' => Cell::get_height('a' as u8),
            'E' => Cell::get_height('z' as u8),
            'a'..='z' => (height - 'a' as u8).into(),
            _ => panic!("Invalid elevation {}", height),
        };
    }

    fn get_position(height: u8) -> Position {
        return match height as char {
            'S' => Position::Start,
            'E' => Position::End,
            _ => Position::Normal,
        };
    }

    fn new(height: u8) -> Self {
        return Self{
            elevation: Cell::get_height(height),
            position: Cell::get_position(height),
        };
    }
}

impl Into<usize> for Cell {
    fn into(self) -> usize { self.elevation }
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        let (d_begin, d_end) = match self.position {
            Position::Normal => (" ", " "),
            _ => ("[", "]"),
        };

        return format!("{}{:2}{}", d_begin, self.elevation, d_end);
    }
}

struct Board {
    cells: Vec<Vec<Cell>>,
    begin: Coord,
    end: Coord,
}

impl Board {
    fn new(lines: Vec<String>) -> Self {
        let mut rows: Vec<Vec<Cell>> = Vec::new();
        for line in lines {
            rows.push(line.bytes().map(Cell::new).collect());
        }
        let mut start: Coord = (0, 0);
        let mut end: Coord = (0, 0);

        for (i, line) in rows.iter().enumerate() {
            for (j, cell) in line.iter().enumerate() {
                if cell.position == Position::Start {
                    start = (i, j);
                }
                if cell.position == Position::End {
                    end = (i, j);
                }
            }
        }

        return Self{cells: rows, begin: start, end: end};
    }

    fn get_size(&self) -> Coord {
        return (self.cells.len(), self.cells[0].len());
    }

    fn get_cell(&self, coord: Coord) -> &Cell {
        return &self.cells[coord.0][coord.1];
    }

    fn distance_to_end(&self, coord: Coord) -> usize {
        let (crow, ccol) = coord;
        let (erow, ecol) = self.end;

        return (
            (erow as i64 - crow as i64).abs() +
            (ecol as i64 - ccol as i64).abs()
        ) as usize;
    }

    fn get_adjacent(&self, from: Coord) -> Vec<Coord> {
        let height = self.get_cell(from).elevation;
        let (row, col) = from;
        let (r_max, c_max) = self.get_size();

        let mut ret = Vec::new();
        if row > 0 { ret.push((row - 1, col)); }
        if col > 0 { ret.push((row, col - 1)); }
        if row < r_max - 1 { ret.push((row + 1, col)); }
        if col < c_max - 1 { ret.push((row, col + 1)); }
        return ret.into_iter()
            .filter(|coord| self.get_cell(*coord).elevation <= height + 1)
            .collect();
    }

    fn display(&self) {
        for row in self.cells.iter() {
            let cur = row.iter()
                .map(Cell::to_string)
                .collect::<Vec<String>>()
                .join("");
            println!("{}", cur);
        }
    }
}

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();
    let board = Board::new(input);
    board.display();
    println!("{:?}", board.get_adjacent(board.begin));
}
