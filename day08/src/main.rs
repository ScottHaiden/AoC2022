#[derive(Debug, Clone, Copy)]
struct Tree {
    height: usize,
    visible: bool,
}

impl Tree {
    fn new(height: usize) -> Self {
        return Tree{height: height, visible: false};
    }
}

#[derive(Debug)]
struct Map {
    trees: Vec<Vec<Tree>>,
}

impl Map {
    fn new(heights: Vec<Vec<usize>>) -> Self {
        fn to_trees(row: Vec<usize>) -> Vec<Tree> {
            return row.into_iter()
                .map(Tree::new)
                .collect();
        }

        let trees = heights.into_iter()
            .map(to_trees)
            .collect();

        return Map{trees: trees};
    }

    fn print_map(&self) {
        for row in &self.trees {
            for col in row {
                let msg = match col.visible {
                    true => 'x',
                    false => ' ',
                };
                print!("{}", msg);
            }
            println!("");
        }
    }

    fn get_column(&mut self, col: usize) -> Vec<&mut Tree> {
        return self.trees.iter_mut()
            .map(|row| &mut row[col])
            .collect();
    }

    fn update_visibilities(&mut self) {
        fn mark_visibles(trees: &mut dyn Iterator<Item=&mut Tree>) {
            let mut prev_max = -1i32;
            for tree in trees {
                let height: i32 = tree.height.try_into().unwrap();
                if height <= prev_max.try_into().unwrap() { continue; }
                prev_max = height;
                tree.visible = true;
            }
        }

        for row in &mut self.trees {
            mark_visibles(&mut row.iter_mut());
            mark_visibles(&mut row.iter_mut().rev());
        }
        for col in 0..self.trees.len() {
            mark_visibles(&mut self.get_column(col).into_iter());
            mark_visibles(&mut self.get_column(col).into_iter().rev());
        }
    }
}

fn main() {
    let heights = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(String::into_bytes)
        .map(|bytes| {
            bytes.into_iter()
                .map(|c| c - ('0' as u8))
                .map(usize::from)
                .collect()
        })
        .collect();

    let mut map = Map::new(heights);
    map.update_visibilities();
    let visible_trees = map.trees.iter()
        .flatten()
        .filter(|i| i.visible)
        .count();

    map.print_map();
    println!("there are {} visible trees.", visible_trees);
}
