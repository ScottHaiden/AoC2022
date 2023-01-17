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
    let packets = std::env::args()
        .skip(1)
        .map(Packet::new)
        .collect::<Vec<Packet>>();

    if packets.len() < 2 { std::process::exit(1); }

    let a = &packets[0];
    let b = &packets[1];

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", a.compare(b));
}
