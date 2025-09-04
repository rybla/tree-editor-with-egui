pub type TreeStep = usize;

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Index(pub Vec<TreeStep>);

impl Index {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, i: TreeStep) {
        self.0.push(i);
    }

    pub fn get(&self, i: usize) -> Option<TreeStep> {
        if i < self.len() {
            Some(self.0[i])
        } else {
            None
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.0.iter()
    }

    pub fn move_up(&mut self) {
        self.0.pop();
    }

    pub fn move_down(&mut self, i: TreeStep) {
        self.0.push(i);
    }

    pub fn move_left_sibling(&mut self) {
        if let Some(i) = self.0.pop() {
            self.0.push(if i > 0 { i - 1 } else { i });
        }
    }

    pub fn move_right_sibling(&mut self) {
        if let Some(parent) = self.0.pop() {
            self.0.push(parent + 1);
        }
    }

    pub fn shift(&mut self) -> Option<TreeStep> {
        if let Some(i) = self.get(0) {
            self.0.remove(0);
            Some(i)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Tree {
    pub label: String,
    pub kids: Vec<Tree>,
}

impl Tree {
    pub fn at_path(&self, path: &Index, i: Option<usize>) -> &Tree {
        match i {
            None => todo!(),
            Some(i) => {
                if i == path.len() {
                    self
                } else {
                    self.kids[i].at_path(path, Some(i + 1))
                }
            }
        }
    }

    pub fn index_in_bounds(&self, index: &Index) -> bool {
        let mut tree = self;
        for i in index.iter() {
            if !(*i < tree.kids.len()) {
                return false;
            }
            tree = &tree.kids[*i]
        }
        return true;
    }

    // pub fn insert_at(&mut self, index: &Index, path: Path) {
    //     let mut tree = self.clone();
    //     let mut path_outer: Path = Path::new();
    //     for i in path.iter() {
    //         let (kids_left, kids_middle_and_right) = tree.kids.split_at(i);
    //         // let kids_middle_and_right = kids_middle_and_right.to_vec();
    //         let kid_middle = kids_middle_and_right[0];
    //         let kids_right = &kids_middle_and_right[1..kids_middle_and_right.len()];
    //         path_outer.push(Tooth {
    //             label: tree.label.clone(),
    //             kids_left: todo!(),
    //             kids_right: todo!(),
    //         });
    //     }
    // }
}

pub fn big_tree(width: u32, height: u32) -> Tree {
    fn go(current_depth: u32, width: u32, height: u32) -> Tree {
        if current_depth == height {
            Tree {
                label: format!("D{current_depth}"),
                kids: vec![],
            }
        } else {
            let mut kids = Vec::with_capacity(width as usize);
            for _ in 0..width {
                kids.push(go(current_depth + 1, width, height));
            }
            Tree {
                label: format!("N{current_depth}"),
                kids,
            }
        }
    }

    // Start the recursion from depth 0
    go(0, width, height)
}

pub type Path = Vec<Tooth>;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Tooth {
    label: String,
    kids_left: Vec<Tree>,
    kids_right: Vec<Tree>,
}
