pub type TreeStep = usize;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Tree {
    pub label: String,
    pub kids: Vec<Tree>,
}

impl Tree {
    pub fn mk(label: &str, kids: &[Tree]) -> Tree {
        Tree {
            label: label.to_string(),
            kids: kids.to_vec(),
        }
    }

    pub fn at_index(&self, index: &Index) -> &Tree {
        fn go<'a>(tree: &'a Tree, index: &Index, i: usize) -> &'a Tree {
            if i == index.len() {
                tree
            } else {
                go(&tree.kids[i], index, i + 1)
            }
        }

        go(self, index, 0)
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

    pub fn wrap_with_path_at_index(&mut self, index: &Index, path: Path) {
        fn go(tree: &Tree, index: &Index, path: Path, i: usize) -> Tree {
            if let Some(step) = index.get(i) {
                let kids_left = &tree.kids[..step];
                let kids_right = if step + 1 < tree.kids.len() {
                    &tree.kids[step + 1..]
                } else {
                    &[]
                };
                let kid_middle = go(&tree.kids[step], index, path, i + 1);
                Tree {
                    label: tree.label.clone(),
                    kids: [kids_left, &[kid_middle], kids_right].concat(),
                }
            } else {
                tree.clone().wrap_with_path(path)
            }
        }

        *self = go(self, index, path, 0);
    }

    pub fn wrap_with_path(self, path: Path) -> Tree {
        let mut tree = self;
        for tooth in path.into_iter().rev() {
            tree = tree.wrap_with_tooth(tooth)
        }
        tree
    }

    pub fn wrap_with_tooth(self, tooth: Tooth) -> Tree {
        Tree {
            label: tooth.label,
            kids: [tooth.kids_left, vec![self], tooth.kids_right].concat(),
        }
    }
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
    pub label: String,
    pub kids_left: Vec<Tree>,
    pub kids_right: Vec<Tree>,
}

impl Tooth {
    pub fn mk(label: &str, kids_left: &[Tree], kids_right: &[Tree]) -> Self {
        Tooth {
            label: label.to_string(),
            kids_left: kids_left.to_vec(),
            kids_right: kids_right.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_wrap_with_tooth_test1() {
        let tree = Tree::mk("B", &[]);

        let tree_new = tree.wrap_with_tooth(Tooth {
            label: "A".to_string(),
            kids_left: vec![Tree {
                label: "L".to_string(),
                kids: vec![],
            }],
            kids_right: vec![Tree {
                label: "R".to_string(),
                kids: vec![],
            }],
        });

        assert_eq!(
            tree_new,
            Tree::mk(
                "A",
                &[Tree::mk("L", &[]), Tree::mk("B", &[]), Tree::mk("R", &[])]
            )
        )
    }

    #[test]
    fn tree_wrap_with_path_test1() {
        let tree = Tree::mk("B", &[]);

        let tree_new = tree.wrap_with_path(vec![Tooth::mk(
            "A",
            &[Tree::mk("L", &[])],
            &[Tree::mk("R", &[])],
        )]);

        assert_eq!(
            tree_new,
            Tree::mk(
                "A",
                &[Tree::mk("L", &[]), Tree::mk("B", &[]), Tree::mk("R", &[])]
            )
        )
    }

    #[test]
    fn tree_wrap_with_path_test2() {
        let tree = Tree::mk("B", &[]);

        let tree_new = tree.wrap_with_path(vec![
            Tooth::mk("A1", &[Tree::mk("L", &[])], &[Tree::mk("R", &[])]),
            Tooth::mk("A2", &[Tree::mk("L", &[])], &[Tree::mk("R", &[])]),
            Tooth::mk("A3", &[Tree::mk("L", &[])], &[Tree::mk("R", &[])]),
        ]);

        assert_eq!(
            tree_new,
            Tree::mk(
                "A1",
                &[
                    Tree::mk("L", &[]),
                    Tree::mk(
                        "A2",
                        &[
                            Tree::mk("L", &[]),
                            Tree::mk(
                                "A3",
                                &[Tree::mk("L", &[]), Tree::mk("B", &[]), Tree::mk("R", &[])]
                            ),
                            Tree::mk("R", &[])
                        ]
                    ),
                    Tree::mk("R", &[])
                ]
            )
        )
    }

    #[test]
    fn tree_wrap_with_path_at_test1() {
        let tree = Tree::mk("B", &[]);

        let mut tree_new = tree.clone();
        tree_new.wrap_with_path_at_index(
            &Index(vec![]),
            vec![Tooth::mk("A", &[Tree::mk("L", &[])], &[Tree::mk("R", &[])])],
        );

        assert_eq!(
            tree_new,
            Tree::mk(
                "A",
                &[Tree::mk("L", &[]), Tree::mk("B", &[]), Tree::mk("R", &[])]
            )
        )
    }

    #[test]
    fn tree_wrap_with_path_at_test2() {
        let tree = Tree::mk("B1", &[Tree::mk("B2", &[])]);

        let mut tree_new = tree.clone();
        tree_new.wrap_with_path_at_index(
            &Index(vec![0]),
            vec![Tooth::mk("A", &[Tree::mk("L", &[])], &[Tree::mk("R", &[])])],
        );

        assert_eq!(
            tree_new,
            Tree::mk(
                "B1",
                &[Tree::mk(
                    "A",
                    &[Tree::mk("L", &[]), Tree::mk("B2", &[]), Tree::mk("R", &[])]
                )]
            )
        )
    }
}
