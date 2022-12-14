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

    fn get_column_mut(&mut self, col: usize) -> Vec<&mut Tree> {
        return self.trees.iter_mut()
            .map(|row| &mut row[col])
            .collect();
    }

    fn get_column(&self, col: usize) -> Vec<Tree> {
        return self.trees.iter()
            .map(|row| row[col])
            .collect();
    }

    fn get_score(&self, coords: (usize, usize)) -> usize {
        fn get_score(trees: &mut dyn Iterator<Item=&Tree>) -> usize {
            let tree = trees.next().unwrap();

            let mut len = 0;
            let mut tallest = -1i32;

            for (i, cur) in trees.enumerate() {
                len = i;
                tallest = cur.height as i32;

                if tallest >= tree.height as i32 { break; }
            }

            if tallest < 0i32 { return 0; }
            return len + 1;
        }

        let (row, col) = coords;

        let row_trees = &self.trees[row];
        let from_right = get_score(&mut row_trees[..=col].iter().rev());
        let from_left = get_score(&mut row_trees[col..].iter());

        let col_trees = self.get_column(col);
        let from_bottom = get_score(&mut col_trees[..=row].iter().rev());
        let from_top = get_score(&mut col_trees[row..].iter());

        let ret = from_right * from_left * from_bottom * from_top;
        println!("{:?}: up {} down {} left {} right {} total {}",
                 coords, from_bottom, from_top, from_right, from_left, ret);
        return ret;
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
            mark_visibles(&mut self.get_column_mut(col).into_iter());
            mark_visibles(&mut self.get_column_mut(col).into_iter().rev());
        }
    }
}

fn cartesian_product(len: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();

    for i in 0..len {
        for j in 0..len {
            ret.push((i, j));
        }
    }

    return ret;
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

    let best_spot = cartesian_product(map.trees.len())
        .iter()
        .map(|coords| map.get_score(*coords))
        .max()
        .unwrap();

    map.print_map();
    println!("there are {} visible trees.", visible_trees);
    println!("the best spot has a score of {}.", best_spot);
}
