#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn new(input: &str) -> Self {
        let parts = input.split(",")
            .map(|i| i.parse())
            .map(Result::unwrap)
            .collect::<Vec<u32>>();

        if parts.len() != 2 {
            panic!("Invalid coordinate string {}", input);
        }

        let x = parts[0];
        let y = parts[1];

        return Coord{x: x, y: y};
    }

    fn parse_line(input: String) -> Vec<Self> {
        return input.split(" -> ")
            .map(Self::new)
            .collect::<Vec<Self>>();
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(Coord::parse_line)
        .collect::<Vec<Vec<Coord>>>();

    for line in lines.iter() {
        println!("{:?}", line);
    }
}
