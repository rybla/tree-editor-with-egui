#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct TreeIndex(pub Vec<usize>);

impl TreeIndex {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, i: usize) {
        self.0.push(i);
    }

    pub fn get(&self, i: usize) -> Option<usize> {
        if i < self.len() {
            Some(self.0[i])
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
    pub fn at_path(&self, path: &TreeIndex, i: Option<usize>) -> &Tree {
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
}

pub fn big_tree(width: u32, height: u32) -> Tree {
    // Define a nested function for recursive tree generation.
    // Nested functions in Rust do not capture outer scope variables implicitly
    // like closures do, so `width` and `height` must be passed as arguments.
    fn go_recursive(current_depth: u32, width: u32, height: u32) -> Tree {
        if current_depth == height {
            Tree {
                label: format!("N{current_depth}"),
                kids: vec![],
            }
        } else {
            let mut kids = Vec::with_capacity(width as usize);
            for _ in 0..width {
                kids.push(go_recursive(current_depth + 1, width, height));
            }
            Tree {
                label: format!("N{current_depth}"),
                kids,
            }
        }
    }

    // Start the recursion from depth 0
    go_recursive(0, width, height)
}
