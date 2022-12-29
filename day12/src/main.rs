use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Coord = (usize, usize);

#[derive(PartialEq)]
enum Position { Normal, Start, End }

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: Coord,
}

impl State {
    fn new(cost: usize, position: Coord) -> Self {
        return Self{cost: cost, position: position};
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let ret = other.cost.cmp(&self.cost)
            .then_with(|| other.position.cmp(&self.position));
        // println!("{:?} <=> {:?} = {:?}", self, other, ret);
        return ret;
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

struct Cell {
    elevation: usize,
    position: Position,
    cost: usize,
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
            cost: usize::MAX,
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
    fn new(lines: &Vec<String>) -> Self {
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

    fn get_cell_mut(&mut self, coord: Coord) -> &mut Cell {
        return &mut self.cells[coord.0][coord.1];
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

    #[allow(dead_code)]
    fn show_costs(&self) {
        for row in self.cells.iter() {
            let cur = row.iter()
                .map(|c| c.cost)
                .map(|c| format!("{:2}", c))
                .collect::<Vec<String>>()
                .join(" ");
            println!("{}", cur);
        }
    }

    fn find_all_a_elevations(&self) -> Vec<Coord> {
        let mut ret = Vec::new();

        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell.elevation > 0 { continue; }
                ret.push((i, j));
            }
        }

        return ret;
    }
}

// Find the shortest path from board.start to end. If any_a is true, then the
// cost to reach any square at elevation a is 0.
fn shortest_path(board: &mut Board, any_a: bool) -> Option<usize> {
    let mut positions = BinaryHeap::<State>::new();
    let initial_positions = match any_a {
        true => board.find_all_a_elevations(),
        false => vec![board.begin],
    };
    for cell in initial_positions {
        positions.push(State::new(0, cell));
        board.get_cell_mut(cell).cost = 0;
    }

    while let Some(State { cost, position }) = positions.pop() {
        if position == board.end { return Some(cost); }
        let cell = board.get_cell(position);
        if cost > cell.cost { continue; }

        for neighbor in board.get_adjacent(position) {
            let cell = board.get_cell_mut(neighbor);
            let next_cost = match any_a && cell.elevation == 0 {
                true => 0,
                false => cost + 1,
            };
            let next = State::new(next_cost, neighbor);
            if next.cost < cell.cost {
                positions.push(next);
                board.get_cell_mut(neighbor).cost = next.cost;
            }
        }
    }

    return None;
}

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();
    {
        let mut board = Board::new(&input);
        board.display();
        println!("Cost from start: {:?}", shortest_path(&mut board, false));
    }
    {
        let mut board = Board::new(&input);
        println!("Cost from any a: {:?}", shortest_path(&mut board, true));
    }
}
