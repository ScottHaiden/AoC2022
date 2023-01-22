use std::cmp;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        return Coord{x: x, y: y};
    }

    fn parse(input: &str) -> Self {
        let parts = input.split(",")
            .map(|i| i.parse())
            .map(Result::unwrap)
            .collect::<Vec<usize>>();

        if parts.len() != 2 {
            panic!("Invalid coordinate string {}", input);
        }

        let x = parts[0];
        let y = parts[1];

        return Coord::new(x, y);
    }

    fn parse_line(input: String) -> Vec<Self> {
        return input.split(" -> ")
            .map(Self::parse)
            .collect::<Vec<Self>>();
    }

    fn down(&self) -> Self {
        return Self::new(self.x, self.y + 1);
    }

    fn down_left(&self) -> Self {
        return Self::new(self.x - 1, self.y + 1);
    }

    fn down_right(&self) -> Self {
        return Self::new(self.x + 1, self.y + 1);
    }

    fn cells_in_wall(vertices: &Vec<Coord>) -> Vec<Coord> {
        let mut ret = Vec::new();

        let mut inner = |from: Coord, to: Coord| {
            let Coord{x: fx, y: fy} = from;
            let Coord{x: tx, y: ty} = to;

            let minx = cmp::min(fx, tx);
            let maxx = cmp::max(fx, tx);

            let miny = cmp::min(fy, ty);
            let maxy = cmp::max(fy, ty);

            let xchanged = minx != maxx;
            let ychanged = miny != maxy;

            assert_ne!(xchanged, ychanged);

            if xchanged {
                for x in minx..=maxx {
                    ret.push(Coord::new(x, miny));
                }
            } else {
                for y in miny..=maxy {
                    ret.push(Coord::new(minx, y));
                }
            }
        };

        for i in 1..vertices.len() {
            inner(vertices[i - 1], vertices[i]);
        }

        return ret;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Sand,
}

impl Cell {
    fn get_char(&self) -> &'static str {
        return match self {
            Self::Empty => ".",
            Self::Wall => "#",
            Self::Sand => "o",
        };
    }
}

#[derive(Clone)]
struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new() -> Self {
        return Board{cells: vec![vec![Cell::Empty]]};
    }

    fn set(&mut self, coord: Coord, value: Cell) {
        let Coord{x, y} = coord;

        let rows = self.rows();
        let cols = self.cols();

        if rows <= y {
            for _ in rows..=y {
                self.cells.push(vec![Cell::Empty; cols]);
            }
        }

        if cols <= x {
            for row in self.cells.iter_mut() { row.resize(x + 1, Cell::Empty); }
        }

        self.cells[y][x] = value;
    }

    fn rows(&self) -> usize { return self.cells.len(); }
    fn cols(&self) -> usize { return self.cells[0].len(); }

    fn get(&self, coord: Coord) -> Option<Cell> {
        let Coord{x, y} = coord;
        if x >= self.cols() { return None; }
        if y >= self.rows() { return None; }
        return Some(self.cells[y][x]);
    }

    // Returns true, if the sand falls into the abyss, otherwise records where the sand fell and
    // returns false.
    fn simulate_grain(&mut self, coord: Coord) -> bool {
        if self.get(coord) == None { return false; }

        let can_try_cell = |coord: Coord| -> bool {
            return match self.get(coord) {
                None => true,
                Some(Cell::Empty) => true,
                _ => false,
            };
        };

        if can_try_cell(coord.down()) {
            return self.simulate_grain(coord.down());
        }
        if can_try_cell(coord.down_left()) {
            return self.simulate_grain(coord.down_left());
        }
        if can_try_cell(coord.down_right()) {
            return self.simulate_grain(coord.down_right());
        }

        self.set(coord, Cell::Sand);
        return true;
    }

    fn describe(&self) {
        println!("board {} x {}", self.rows(), self.cols());
    }

    fn print(&self) {
        self.describe();
        for row in self.cells.iter() {
            for cell in row {
                print!("{}", cell.get_char());
            }
            println!("");
        }
    }

    fn add_walls(&mut self, wall_vertices: &Vec<Vec<Coord>>) {
        for wall in wall_vertices {
            for coord in Coord::cells_in_wall(wall) {
                self.set(coord, Cell::Wall);
            }
        }
    }
}

fn part_one(mut board: Board) {
    for grains in 0.. {
        let keep_going = board.simulate_grain(Coord::new(500, 0));
        if keep_going { continue; }
        board.print();
        println!("held {} grains", grains);
        break;
    }
}

fn part_two(mut board: Board) {
    let floor_y = board.rows() + 1;
    let bottom_wall = vec![Coord::new(0, floor_y), Coord::new(1000, floor_y)];
    board.add_walls(&vec![bottom_wall]);

    for grains in 1.. {
        let keep_going = board.simulate_grain(Coord::new(500, 0));
        assert!(keep_going);

        if board.get(Coord::new(500, 0)) != Some(Cell::Sand) { continue; }
        
        board.print();
        println!("held {} grains", grains);
        break;
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(Coord::parse_line)
        .collect::<Vec<Vec<Coord>>>();

    let mut board = Board::new();
    board.add_walls(&lines);

    part_one(board.clone());
    part_two(board);
}
