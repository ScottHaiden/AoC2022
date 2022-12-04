use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    first: HashSet<char>,
    second: HashSet<char>,
}

impl Rucksack {
    fn new(line: String) -> Rucksack {
        let len = line.len();
        assert!(len & 1usize == 0);

        fn converter(slice: &str) -> HashSet<char> {
            return slice.chars().map(|i| i.clone()).collect();
        }

        let half = len / 2;
        let first = converter(&line[..half]);
        let second = converter(&line[half..]);

        return Rucksack{first: first, second: second};
    }

    fn all_items(&self) -> HashSet<char> {
        return self.first.union(&self.second)
            .copied()
            .collect();
    }

    fn item_priority(item: char) -> u32 {
        let as_u32 = u32::from(item);
        return match item {
            'a'..='z' => as_u32 - u32::from('a') + 1,
            'A'..='Z' => as_u32 - u32::from('A') + 1 + 26,
            _ => panic!("invalid letter {}", item),
        };
    }

    fn find_duplicate(&self) -> char {
        let mut intersected = self.first
            .intersection(&self.second)
            .copied()
            .collect::<HashSet<char>>();

        for c in intersected.drain() { return c; }
        panic!("Unreachable");
    }

    fn find_badge(rucksacks: &[Rucksack]) -> char {
        assert!(rucksacks.len() == 3);

        let in_all = rucksacks.iter()
            .map(|i| i.all_items())
            .reduce(|memo, i| {
                        memo.intersection(&i).copied().collect::<HashSet<char>>()
                    })
            .unwrap();

        assert!(in_all.len() == 1);
        for c in in_all { return c; }
        panic!("unreachable");
    }
}

fn main() {
    let rucksacks = std::io::stdin().lines()
        .map(|i| i.unwrap())
        .map(Rucksack::new)
        .collect::<Vec<Rucksack>>();

    let balance_prios = rucksacks.iter()
        .map(|i| i.find_duplicate())
        .map(Rucksack::item_priority)
        .sum::<u32>();

    let badge_prios = rucksacks
        .chunks(3)
        .into_iter()
        .map(Rucksack::find_badge)
        .map(Rucksack::item_priority)
        .sum::<u32>();

    println!("balance priority: {}", balance_prios);
    println!("badge priority: {}", badge_prios);
}
