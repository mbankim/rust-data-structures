use std::cmp::Ordering;

type Tree = Option<Box<Node>>;

pub struct BST {
    root: Tree,
}

impl BST {
    pub fn new() -> Self {
        BST{ root : None }
    }

    pub fn insert(&mut self, val: i32) {
        insert_node(&mut self.root, val);
    }

    pub fn delete(&mut self, val: i32) {
    }

    pub fn inc_order(&self) -> Vec<i32> {
        let mut result = vec![];
        dfs(&self.root, &mut result);
        result
    }
}

struct Node {
    val: i32,
    left: Tree,
    right: Tree,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

fn insert_node(bst: &mut Tree, val: i32) {
    match *bst {
        Some(ref mut tree) => {
            match val.cmp(&tree.val) {
                Ordering::Less => insert_node(&mut tree.left, val),
                Ordering::Greater => insert_node(&mut tree.right, val),
                Ordering::Equal => panic!("Node with value {} already exists!", val),
            }
        },
        None => *bst = Some(Box::new(Node::new(val))),
    }
}

fn dfs(bst: &Tree, result: &mut Vec<i32>) {
    match *bst {
        Some(ref tree) => {
            dfs(&tree.left, result);
            result.push(tree.val);
            dfs(&tree.right, result);
        },
        None => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let mut bst = BST::new();
        bst.insert(3);
        bst.insert(5);
        bst.insert(1);

        assert_eq!(bst.inc_order(), vec![1, 3, 5])
    }
}
