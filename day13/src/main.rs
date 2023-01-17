#[derive(Debug)]
enum Packet {
    None,
    Int(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn new(s: String) -> Self {
        assert_eq!(s.chars().nth(0).unwrap(), '[');

        let (remainder, ret) = Self::parse_list(&s[1..], Vec::new());

        assert_eq!(remainder, "");

        return Packet::List(ret);
    }

    fn parse_int(input: &str, cur: u32) -> (&str, u32) {
        let cur_char = match input.chars().nth(0) {
            Some(c) => c,
            None => return (input, cur),
        };

        let digit = match cur_char {
            '0'..='9' => cur_char as u32 - '0' as u32,
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
}

fn main() {
    let packet = "[1,2,[[],4,5],3]".to_string();

    println!("{}", packet);
    println!("{:?}", Packet::new(packet));
}
