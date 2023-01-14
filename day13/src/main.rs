enum Packet {
    None,
    Int(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn new(s: String) -> Self {

    }
}

fn main() {
    println!("Hello, world!");
}
