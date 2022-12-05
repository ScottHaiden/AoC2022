type Stack = std::collections::VecDeque<char>;

#[derive(Debug, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn new(line: String) -> Self {
        fn parse(n: &str) -> usize {
            n.parse::<usize>().unwrap()
        }

        let pieces = line.split(" ")
            .collect::<Vec<&str>>();
        let count = parse(pieces[1]);
        let from = parse(pieces[3]) - 1;
        let to = parse(pieces[5]) - 1;

        return Move{count: count, from: from, to: to};
    }
}

#[derive(Debug, Clone)]
struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {
    fn new(nstacks: usize) -> Self {
        let stacks = (0..nstacks).into_iter()
            .map(|_| Stack::new())
            .collect();
        return Self{stacks: stacks};
    }

    fn parse_initial(&mut self, rows: Vec<String>) {
        fn cell_index(cell: usize) -> usize {
            return cell * 4 + 1;
        }
        let mut parse_row = |row: &String| {
            for i in 0..self.stacks.len() {
                let index = cell_index(i);
                let box_at = row.chars().nth(index).unwrap();
                if box_at == ' ' { continue; }
                self.stacks[i].push_front(box_at);
            }
        };
        for row in rows {
            if !row.contains("[") { break; }
            parse_row(&row);
        }
    }
    
    fn perform_part1(&self, mv: &Move) -> Self {
        let mut ret = self.clone();

        for _ in 0..mv.count {
            let cur = ret.stacks[mv.from].pop_back().expect("Move from empty");
            ret.stacks[mv.to].push_back(cur);
        }

        return ret;
    }

    fn perform_part2(&self, mv: &Move) -> Self {
        let mut ret = self.clone();

        let mut boxes = Vec::new();
        for _ in 0..mv.count {
            boxes.push(ret.stacks[mv.from].pop_back().expect("move from empty"));
        }
        boxes.reverse();
        for i in boxes {
            ret.stacks[mv.to].push_back(i);
        }
        return ret;
    }

    fn tops(&self) -> String {
        let mut ret = String::new();

        for stack in &self.stacks {
            if stack.is_empty() { continue; }
            ret.push(*stack.back().unwrap());
        }
        return ret;
    }
}

fn main() {
    let stacks = std::env::args()
        .nth(1)
        .expect("Expected one argument")
        .parse::<usize>()
        .expect("Argument should parse");

    let mut lines = std::io::stdin()
        .lines()
        .map(|i| i.unwrap());

    let crates = (&mut lines)
        .take_while(|i| !i.is_empty())
        .collect::<Vec<String>>();
    let moves = lines.into_iter().map(Move::new).collect::<Vec<Move>>();

    let mut towers = Stacks::new(stacks);
    towers.parse_initial(crates);

    let part1 = moves.iter().fold(towers.clone(), |acc, i| acc.perform_part1(&i));
    let part2 = moves.into_iter().fold(towers, |acc, i| acc.perform_part2(&i));

    println!("part 1: {}", part1.tops());
    println!("part 2: {}", part2.tops());
}
