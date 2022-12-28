use std::collections::{HashSet, VecDeque};

#[derive(Clone)]
struct Monkey {
    items: VecDeque<i64>,
    operation: fn(i64, i64) -> i64,
    test: i64,
    dest_true: usize,
    dest_false: usize,
    inspections: usize
}

impl Monkey {
    fn new(items: &[i64], op: fn(i64, i64)->i64, factor: i64, t: usize, f: usize) -> Self {
        return Self {
            items: items.iter().copied().collect(),
            operation: op,
            test: factor,
            dest_true: t,
            dest_false: f,
            inspections: 0usize,
        };
    }

    fn take_item(&mut self, item: i64) {
        self.items.push_back(item);
    }

    fn make_move(&mut self, divisor: i64) -> Option<(i64, usize)> {
        if self.items.is_empty() { return None; }
        self.inspections += 1;
        let worry = self.items.pop_front().unwrap();
        let worry = (self.operation)(worry, worry) / divisor;
        let dest = match worry % self.test == 0 {
            true => self.dest_true,
            false => self.dest_false,
        };
        return Some((worry, dest));
    }
}

fn get_example() -> Vec<Monkey> {
    let mut ret = Vec::new();
    ret.push(Monkey::new(&[79, 98], |old, _new| { old * 19 }, 23, 2, 3));
    ret.push(Monkey::new(&[54, 65, 75, 74], |old, _new| { old + 6 }, 19, 2, 0));
    ret.push(Monkey::new(&[79, 60, 97], |old,  new| { new * old }, 13, 1, 3));
    ret.push(Monkey::new(&[74], |old, _new| { old + 3 }, 17, 0, 1));
    return ret;
}

fn get_input() -> Vec<Monkey> {
    let mut ret = Vec::new();
    ret.push(Monkey::new(&[52, 60, 85, 69, 75, 75], |old, _new| { old * 17 }, 13, 6, 7));
    ret.push(Monkey::new(&[96, 82, 61, 99, 82, 84, 85], |old, _new| { old + 8 }, 7, 0, 7));
    ret.push(Monkey::new(&[95, 79], |old, _new| { old + 6 }, 19, 5, 3));
    ret.push(Monkey::new(&[88, 50, 82, 65, 77], |old, _new| { old * 19 }, 2, 4, 1));
    ret.push(Monkey::new(&[66, 90, 59, 90, 87, 63, 53, 88], |old, _new| { old + 7 }, 5, 1, 0));
    ret.push(Monkey::new(&[92, 75, 62], |old, _new| { old * old }, 3, 3, 4));
    ret.push(Monkey::new(&[94, 86, 76, 67], |old, _new| { old + 1 }, 11, 5, 2));
    ret.push(Monkey::new(&[57], |old, _new| { old + 2 }, 17, 6, 2));
    return ret;
}

fn turn(monkeys: &mut [Monkey], index: usize, divisor: i64, factor: i64) {
    loop {
        let result = monkeys[index].make_move(divisor);
        if result.is_none() { break; }
        let (worry, dest) = result.unwrap();
        monkeys[dest].take_item(worry % factor);
    }
}

fn round_part1(monkeys: &mut [Monkey], factor: i64) {
    for i in 0..monkeys.len() { turn(monkeys, i, 3, factor); }
}

fn round_part2(monkeys: &mut [Monkey], factor: i64) {
    for i in 0..monkeys.len() { turn(monkeys, i, 1, factor); }
}

fn print_inspections(monkeys: &[Monkey]) {
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("{}: {}", i, monkey.inspections);
    }
}

fn calculate_monkey_business(monkeys: &[Monkey]) -> usize {
    let mut inspections: Vec<usize> = monkeys.iter()
        .map(|i| i.inspections)
        .collect();
    inspections.sort();
    return inspections.into_iter()
        .rev()
        .take(2)
        .reduce(|memo, cur| memo * cur)
        .expect("expected a value");
}

fn find_lcm(monkeys: &[Monkey]) -> i64 {
    return monkeys.iter()
        .map(|i| i.test)            // find the test
        .collect::<HashSet<i64>>()  // remove dupes
        .into_iter()                // iterate
        .reduce(|memo, i| memo * i) // multiply
        .unwrap();
}

fn do_part_1(monkeys: &mut [Monkey], msg: &str) {
    let lcm = find_lcm(monkeys);
    for _ in 0..20 { round_part1(monkeys, lcm); }
    println!("after 20 rounds ({}): {}", msg, calculate_monkey_business(monkeys));
}

fn do_part_2(monkeys: &mut [Monkey], msg: &str) {
    let lcm = find_lcm(monkeys);
    for _ in 0..10000 { round_part2(monkeys, lcm); }
    println!("after 10000 rounds ({}): {}", msg, calculate_monkey_business(monkeys));
}

fn main() {
    do_part_1(&mut get_example(), "example");
    do_part_1(&mut get_input(), "input");

    do_part_2(&mut get_example(), "example");
    do_part_2(&mut get_input(), "input");
}
