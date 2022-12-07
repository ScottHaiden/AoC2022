use std::collections::HashMap;

struct File {
    size: usize,
}

impl File {
    fn new(size: usize) -> Self {
        return File{size: size};
    }
}


struct Directory {
    subdirs: HashMap<String, Directory>,
    files: HashMap<String, File>,
    size: usize,
}

impl Directory {
    fn new() -> Self {
        return Directory{
            subdirs: HashMap::new(),
            files: HashMap::new(),
            size: 0usize,
        };
    }

    // Prepopulate all the sizes on all the directories. Probably not necessary for this challenge,
    // but I didn't know that when I started.
    fn calculate_size(&mut self) -> usize {
        let subdirs_size: usize = self.subdirs.values_mut()
            .map(Directory::calculate_size)
            .sum();

        let files_sizes: usize = self.files.values()
            .map(|file| file.size)
            .sum();

        let total = subdirs_size + files_sizes;
        self.size = total;
        return total;
    }

    // Find the total combined size of all subdirectories each of which has a size less than
    // |limit|.
    fn sum_smaller_than(&self, limit: usize) -> usize {
        let subdirs = self.subdirs.values()
            .map(|i| i.sum_smaller_than(limit))
            .sum();

        let self_size = self.size;
        if self_size > limit { return subdirs; }
        return self_size + subdirs;
    }

    // Find the smallest subdirectory whose size is greater than |limit|.
    fn find_smallest_above(&self, limit: usize) -> usize {
        if self.size < limit { return usize::MAX; }
        let subdirs_above: usize = self.subdirs.values()
            .map(|d| d.find_smallest_above(limit))
            .min()
            .unwrap_or(usize::MAX);

        return std::cmp::min(self.size, subdirs_above);
    }

    fn add_dir(&mut self, name: String, dir: Directory) {
        self.subdirs.insert(name, dir);
    }
    fn add_file(&mut self, name: String, file: File) {
        self.files.insert(name, file);
    }

    // Add a file or directory based upon |line| to this directory.
    fn parse_line(&mut self, line: &String) {
        let mut parts = line.split(" ");
        let leader = parts.next().expect("expected a leader");
        let name = parts.next().expect("expected a name").to_owned();
        assert_eq!(parts.next(), None);

        if leader == "dir" {
            self.add_dir(name, Directory::new());
            return;
        }

        let size = leader.parse::<usize>().expect("expected 'dir' or a size");
        self.add_file(name, File::new(size));
    }
}

fn parse(dir: &mut Directory, lines: &mut dyn Iterator<Item=&String>) {
    let command = lines.next().expect("Expected a line.");
    assert_eq!(command, "$ ls");

    loop {
        let line = lines.next();
        if line.is_none() { return; }
        let line = line.unwrap();

        if line == "$ cd .." { return; }
        if line.starts_with("$ cd ") {
            let dirname: &str = &line[5..];
            let mut subdir = dir.subdirs
                .get_mut(dirname)
                .expect("dir not found");
            parse(&mut subdir, lines);
            continue;
        }
        if !line.starts_with("?") {
            dir.parse_line(line);
            continue;
        }
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .skip(1)
        .collect::<Vec<String>>();

    let mut root = Directory::new();
    parse(&mut root, &mut lines.iter());
    let disk_size = 70000000usize;
    let used_size = root.calculate_size();
    let free_size = disk_size - used_size;
    let need_size = 30000000usize - free_size;

    println!("total:              {}", root.size);
    println!("< 100000:           {}", root.sum_smaller_than(100000));
    println!("extra space needed: {}", need_size);
    println!("smallest to delete: {}", root.find_smallest_above(need_size));
}
