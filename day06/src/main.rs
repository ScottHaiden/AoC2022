use std::collections::HashMap;

#[derive(Debug)]
struct Scanner<'a> {
    line: &'a String,
    length: usize,
}

impl <'a> Scanner<'a> {
     fn new(length: usize, line: &'a String) -> Self {
        return Scanner{length: length, line: line};
    }

    fn scan(&self) -> Option<usize> {
        return self._scan(0, HashMap::new());
    }

    fn _scan(&self, i: usize, seen: HashMap<char, usize>) -> Option<usize> {
        if seen.len() == self.length { return Some(i); }

        // If we reach the end and haven't found anything, give up.
        if i >= self.line.len() { return None; }
        let cur = self.line[i..i+1].chars().nth(0).unwrap();

        // Add the current letter to the list.
        let mut new_seen = seen;
        new_seen.insert(cur, new_seen.get(&cur).unwrap_or(&0) + 1);

        // If we haven't read |length| letters yet, scan on.
        if i < self.length { return self._scan(i + 1, new_seen); }

        // Drop the last letter and scan on...
        let last_index = i - self.length;
        let last = self.line[last_index..last_index+1].chars().nth(0).unwrap();
        let last_count = new_seen.remove(&last).expect("Last wasn't present");
        if last_count > 1 { new_seen.insert(last, last_count - 1); }

        // Recurse.
        return self._scan(i + 1, new_seen);
    }
}

fn make_iter(len: usize, lines: &Vec<String>) -> impl Iterator<Item = usize> + '_ {
    return lines.iter()
        .map(move |i| Scanner::new(len, i))
        .map(move |i| i.scan().expect("did not find"));
}

fn main() {
    let sop_len = 4usize;
    let som_len = 14usize;

    let lines = std::io::stdin()
        .lines()
        .map(|i| i.unwrap().to_owned())
        .collect::<Vec<String>>();

    let sops = make_iter(sop_len, &lines);
    let soms = make_iter(som_len, &lines);
    let lens = sops.zip(soms);

    for (len_pair, line) in lens.zip(lines.iter()) {
        let (sop, som) = len_pair;
        println!("[start of packet {:2}] [start of message {:2}] text {}", sop, som, line);
    }
}
