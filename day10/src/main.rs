use std::collections::VecDeque;

trait Instruction {
    fn cycles(&self) -> usize;
    fn count_down(&mut self);
    fn apply(&self, register: i64) -> i64;
    fn describe(&self) -> String;
}

#[derive(Debug)]
struct Noop {
    cycles: usize,
}
impl Noop {
    fn new() -> Self { return Self{cycles: 1}; }
}
impl Instruction for Noop {
    fn cycles(&self) -> usize { return self.cycles; }
    fn count_down(&mut self) { self.cycles -= 1; }
    fn apply(&self, register: i64) -> i64 { return register; }
    fn describe(&self) -> String {
        return format!("{:?}", self);
    }
}

#[derive(Debug)]
struct AddX {
    cycles: usize,
    operand: i64,
}
impl AddX {
    fn new(operand: i64) -> Self {
        return Self{cycles: 2, operand: operand};
    }
}
impl Instruction for AddX {
    fn cycles(&self) -> usize { return self.cycles; }
    fn count_down(&mut self) { self.cycles -= 1; }
    fn apply(&self, register: i64) -> i64 { return register + self.operand; }
    fn describe(&self) -> String {
        return format!("{:?}", self);
    }
}

fn parse_insn(line: String) -> Box<dyn Instruction> {
    let parts = line.split(" ").collect::<Vec<&str>>();

    return match parts[0] {
        "noop" => Box::new(Noop::new()),
        "addx" => Box::new(AddX::new(parts[1].parse::<i64>().unwrap())),
        _ => panic!("Invalid insn"),
    };
}

fn is_signal_turn(i: usize) -> bool {
    if i < 20 { return false; }
    let shifted = i - 20;
    if shifted % 40 == 0 { return true; }
    return false;
}

fn main() {
    let mut insns = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(parse_insn)
        .collect::<VecDeque<Box<dyn Instruction>>>();

    let mut signal = 0i64;
    let mut register = 1;
    let mut cycle = 0;
    loop {
        if insns.is_empty() { break; }
        let insn = &mut insns[0];
        insn.count_down();

        let before = register;
        if insn.cycles() == 0 {
            register = insn.apply(register);
            insns.pop_front();
        }
        cycle += 1;
        if is_signal_turn(cycle) { signal += before * cycle as i64; }
    }
    println!("Signal strength is {}", signal);
}
