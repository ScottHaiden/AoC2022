use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn new(s: String) -> Self {
        assert_eq!(s.chars().nth(0).unwrap(), '[');

        let (remainder, ret) = Self::parse_list(&s[1..], Vec::new());

        assert_eq!(remainder, "");

        return Packet::List(ret);
    }

    fn parse_int(input: &str, cur: i32) -> (&str, i32) {
        let cur_char = match input.chars().nth(0) {
            Some(c) => c,
            None => return (input, cur),
        };

        let digit = match cur_char {
            '0'..='9' => cur_char as i32 - '0' as i32,
            _ => return (input, cur),
        };

        return Self::parse_int(&input[1..], cur * 10 + digit);
    }

    fn parse_list(input: &str, mut cur: Vec<Packet>) -> (&str, Vec<Packet>) {
        let cur_char = match input.chars().nth(0) {
            Some(c) => c,
            None => return (input, cur),
        };

        if cur_char == ']' {
            return (&input[1..], cur);
        }

        if cur_char == ',' {
            return Self::parse_list(&input[1..], cur);
        }

        if cur_char == '[' {
            let (remainder, list) = Self::parse_list(&input[1..], Vec::new());
            cur.push(Packet::List(list));
            return Self::parse_list(&remainder, cur);
        }

        if ('0'..='9').contains(&cur_char) {
            let (remainder, number) = Self::parse_int(&input, 0);
            cur.push(Packet::Int(number));
            return Self::parse_list(&remainder, cur);
        }

        panic!("Unexpected character encountered! '{}'", cur_char);
    }

    fn compare(&self, other: &Self) -> i32 {
        return match self {
            Self::Int(_) => self.compare_int(other),
            Self::List(_) => self.compare_list(other),
        };
    }

    fn compare_int(&self, other: &Self) -> i32 {
        let self_value = match self {
            Self::Int(i) => i,
            _ => panic!("compare_int called on non int packet!"),
        };

        if let Self::Int(other_value) = other {
            return (self_value - other_value).signum();
        }

        return Self::List(vec![self.clone()]).compare(other);
    }

    fn compare_list(&self, other: &Self) -> i32 {
        let self_list = match self {
            Self::List(l) => l,
            _ => panic!("compare_list called on non list packet!"),
        };
        let self_len = self_list.len();

        if let Self::List(other_list) = other {
            let other_len = other_list.len();
            if self_len > other_len {
                return -other.compare_list(self);
            }

            for i in 0..self_len {
                let self_cur = &self_list[i];
                let other_cur = &other_list[i];
                let comparison = self_cur.compare(other_cur);
                if comparison != 0 { return comparison; }
            }
            if self_len == other_len { return 0; }
            return -1;
        }

        return self.compare_list(&Self::List(vec![other.clone()]));
    }
}

fn main() {
    let mut lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|i| i.len() > 0)
        .map(Packet::new)
        .collect::<Vec<Packet>>();

    let mut sum = 0;
    for (i, pair) in (1..).zip(lines.chunks(2)) {
        let comp = pair[0].compare(&pair[1]);
        let in_order = comp <= 0;
        println!("{:2}:", i);
        println!("  {:?}", pair[0]);
        println!("  {:?}", pair[1]);
        println!("  in the right order? {}", in_order);
        println!("");

        if in_order { sum += i; }
    }

    println!("Sum of in order indices: {}", sum);

    let divider_two = Packet::new("[[2]]".to_string());
    let divider_six = Packet::new("[[6]]".to_string());

    lines.push(divider_two.clone());
    lines.push(divider_six.clone());

    lines.sort_by(|a, b| match a.compare(&b) {
        -1 => Ordering::Less,
        0 => Ordering::Equal,
        1 => Ordering::Greater,
        _ => panic!("Invalid comparison result"),
    });

    let divider_two_location = lines.iter()
        .position(|i| i.compare(&divider_two) == 0)
        .unwrap() + 1;
    let divider_six_location = lines.iter()
        .position(|i| i.compare(&divider_six) == 0)
        .unwrap() + 1;

    println!("part two: {}", divider_two_location * divider_six_location);
}
