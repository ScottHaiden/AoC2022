use std::collections::BinaryHeap;

struct CalorieList<'a> {
    lines: &'a mut dyn Iterator<Item = String>,
}

impl <'a> CalorieList<'a> {
    fn new(lines: &'a mut dyn Iterator<Item = String>) -> CalorieList<'a> {
        return CalorieList{lines: lines};
    }
}

impl <'a> Iterator for CalorieList<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let lines = &mut self.lines;

        let ret = lines
            .take_while(|i| !i.is_empty())
            .map(|i| i.parse::<u32>())
            .map(|i| i.expect("parse failed"))
            .sum();

        if ret == 0 {
            return None;
        }

        return Some(ret);
    }
}

struct BinaryHeapDescender {
    heap: BinaryHeap<u32>,
}

impl BinaryHeapDescender {
    fn new(heap: BinaryHeap<u32>) -> BinaryHeapDescender {
        return BinaryHeapDescender{heap: heap};
    }
}

impl Iterator for BinaryHeapDescender {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> { return self.heap.pop(); }
}

fn main() {
    let num_elves = std::env::args()
        .nth(1)
        .expect("Expected 1 argument")
        .parse::<usize>()
        .expect("Argument should be an integer");

    let mut calories = std::io::stdin().lines().map(|i| i.unwrap());
    let calories = CalorieList::new(&mut calories).collect::<BinaryHeap<u32>>();
    let calories = BinaryHeapDescender::new(calories);
    let n: u32 = calories.take(num_elves).sum();

    println!("{}", n);
}
