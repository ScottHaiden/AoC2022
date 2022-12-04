#[derive(Debug, Clone, Copy)]
struct Interval {
    begin: u32,
    end: u32,
}

impl Interval {
    fn new(line: &String) -> Interval {
        let items = line.split("-")
            .map(|i| i.parse::<u32>())
            .map(|i| i.unwrap())
            .collect::<Vec<u32>>();
        assert_eq!(items.len(), 2);

        let mut items = items.into_iter();
        let first = items.next().unwrap();
        let second = items.next().unwrap();

        return Interval{begin: first, end: second};
    }

    fn contains(&self, section: u32) -> bool {
        if self.begin > section { return false; }
        if self.end < section { return false; }
        return true;
    }

    fn overlaps(&self, other: &Self) -> bool {
        return self.contains(other.begin) || self.contains(other.end);
    }

    fn encloses(&self, other: &Self) -> bool {
        return self.contains(other.begin) && self.contains(other.end);
    }
}

fn interval_pair(line: &String) -> (Interval, Interval) {
    let halves = line.split(",")
        .map(|i| i.to_string())
        .map(|i| Interval::new(&i))
        .collect::<Vec<Interval>>();
    assert_eq!(halves.len(), 2);
    return (halves[0], halves[1]);
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|i| i.unwrap())
        .filter(|i| !i.is_empty())
        .map(|i| interval_pair(&i))
        .collect::<Vec<(Interval, Interval)>>();

    let enclosures = lines.iter()
        .filter(|(i, j)| i.encloses(&j) || j.encloses(&i))
        .fold(0, |acc, _| acc + 1);

    let overlaps = lines.into_iter()
        .filter(|(i, j)| i.overlaps(&j) || j.overlaps(&i))
        .fold(0, |acc, _| acc + 1);

    println!("enclosures: {}", enclosures);
    println!("overlaps:   {}", overlaps);
}
