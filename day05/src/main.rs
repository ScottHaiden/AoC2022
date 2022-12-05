type Stack = std::collections::VecDeque<char>;

#[derive(Debug)]
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

#[derive(Debug)]
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
    
    fn perform(&mut self, mv: &Move) {
        for i in 0..mv.count {
            let cur = self.stacks[mv.from].pop_back().expect("Move from empty");
            self.stacks[mv.to].push_back(cur);
        }
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
    let mut towers = Stacks::new(stacks);

    let lines = std::io::stdin()
        .lines()
        .map(|i| i.unwrap())
        .collect::<Vec<String>>();

    let mut lines = lines.into_iter();

    let s = (&mut lines).take_while(|i| !i.is_empty()).collect::<Vec<String>>();
    towers.parse_initial(s);

    let moves = lines.into_iter().map(Move::new);
    for mv in moves {
        towers.perform(&mv);
    }

    println!("{:?}", towers);
    println!("{}", towers.tops());
}
