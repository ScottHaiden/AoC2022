use std::collections::HashSet;

type Coord = (i32, i32);

#[derive(Clone, Copy, Debug)]
enum Direction {
    Unspec,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Move {
    direction: Direction,
    distance: usize,
}

impl Move {
    fn new(direction: Direction, distance: usize) -> Self {
        return Self{direction: direction, distance: distance};
    }

    fn next(&self) -> Self {
        if self.distance == 0 { panic!("Can't decrement 0 vector"); }
        return Self::new(self.direction, self.distance - 1);
    }
}

fn move_coord(coord: Coord, direction: Direction) -> Coord {
    let (x, y) = coord;
    return match direction {
        Direction::Unspec => panic!("Invalid direction!"),
        Direction::Up => (x, y + 1),
        Direction::Down => (x, y - 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    };
}

fn follow_coord(head: Coord, tail: Coord) -> Coord {
    let (h_x, h_y) = head;
    let (t_x, t_y) = tail;

    let delta_x = t_x - h_x;
    let delta_y = t_y - h_y;

    // println!("{:?}-{:?} {:?}", head, tail, (delta_x, delta_y));

    return match (delta_x, delta_y) {
        // cases where the tail does not need to move.
        ( 0,  0) => tail,
        (-1,  0) => tail,
        ( 1,  0) => tail,
        ( 0, -1) => tail,
        ( 0,  1) => tail,
        (-1, -1) => tail,
        (-1,  1) => tail,
        ( 1, -1) => tail,
        ( 1,  1) => tail,
        // cases where the tail needs to move in only the x or y direction.
        (-2,  0) => (t_x + 1, t_y),
        ( 2,  0) => (t_x - 1, t_y),
        ( 0, -2) => (t_x, t_y + 1),
        ( 0,  2) => (t_x, t_y - 1),
        // Cases where we need to move in both directions to catch up.
        (-2, -1) => (t_x + 1, t_y + 1),
        (-2,  1) => (t_x + 1, t_y - 1),
        (-1,  2) => (t_x + 1, t_y - 1),
        ( 1,  2) => (t_x - 1, t_y - 1),
        ( 2, -1) => (t_x - 1, t_y + 1),
        ( 2,  1) => (t_x - 1, t_y - 1),
        ( 1, -2) => (t_x - 1, t_y + 1),
        (-1, -2) => (t_x + 1, t_y + 1),
        // Cases where the tail needs to move diagonally
        (-2, -2) => (t_x + 1, t_y + 1),
        (-2,  2) => (t_x + 1, t_y - 1),
        ( 2,  2) => (t_x - 1, t_y - 1),
        ( 2, -2) => (t_x - 1, t_y + 1),
        // Shouldn't be possible:
        _ => panic!("Invalid delta x / delta y {:?}", (delta_x, delta_y)),
    };
}

struct Rope {
    head: Coord,
    tail: Option<Box<Rope>>,
}

impl Rope {
    fn new(len: usize) -> Self {
        let zero = (0, 0);
        if len == 0 { return Self{head: zero, tail: None}; }
        let cdr = Box::new(Self::new(len - 1));
        return Self{head: zero, tail: Some(cdr)};
    }

    fn describe(&self, depth: usize) {
        println!("{}:{:?}", depth, self.head);
        if self.tail.is_none() { return; }
        return self.tail.as_ref().unwrap().describe(depth + 1);
    }

    fn follow_head(&mut self, head: Coord) -> Coord {
        self.head = follow_coord(head, self.head);
        if self.tail.is_none() { return self.head; }
        return self.tail.as_mut()
            .unwrap()
            .follow_head(self.head);
    }

    // Assumes that this has a tail, otherwise it will crash.
    fn apply_move(&mut self, m: Move, visited: &mut HashSet<Coord>) {
        if m.distance == 0 { return; }
        self.head = move_coord(self.head, m.direction);
        let tail_pos = self.tail.as_mut()
            .unwrap()
            .follow_head(self.head);
        visited.insert(tail_pos);
        return self.apply_move(m.next(), visited);
    }
}

fn main() {
    let moves: Vec<Move> = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| -> Move {
            let parts: Vec<&str> = line.split(" ").collect();
            let direction = match parts[0] {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("Unknown direction {}", parts[0]),
            };
            let distance = parts[1].parse::<usize>().expect("failed to parse");
            return Move::new(direction, distance);
        })
        .collect();

    let mut short_rope = Rope::new(1);
    let mut short_visited = HashSet::new();

    let mut long_rope = Rope::new(9);
    let mut long_visited = HashSet::new();

    for m in moves {
        long_rope.apply_move(m, &mut long_visited);
        short_rope.apply_move(m, &mut short_visited);
    }
    println!("short spots visited: {}", short_visited.len());
    println!("long spots visited: {}", long_visited.len());
}
