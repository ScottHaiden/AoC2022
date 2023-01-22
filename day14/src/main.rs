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

#[derive(Clone)]
enum Cell {
    Empty,
    Wall,
    Sand,
}

impl Cell {
    fn new() -> Self {
        return Self::Empty;
    }

    fn get_char(&self) -> &'static str {
        return match self {
            Self::Empty => ".",
            Self::Wall => "#",
            Self::Sand => "o",
        };
    }
}

struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new() -> Self {
        return Board{cells: vec![vec![Cell::Empty]]};
    }

    fn set_cell(&mut self, coord: Coord, value: Cell) {
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
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(Coord::parse_line)
        .collect::<Vec<Vec<Coord>>>();

    let mut board = Board::new();
    for line in lines.iter() {
        for coord in Coord::cells_in_wall(line) {
            board.set_cell(coord, Cell::Wall);
        }
    }
    board.print();
}
