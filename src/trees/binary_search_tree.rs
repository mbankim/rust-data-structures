use std::cmp::Ordering;
use std::mem;

type Tree = Option<Box<Node>>;

pub struct BinarySearchTree {
    root: Tree,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree { root : None }
    }

    pub fn insert(&mut self, val: i32) {
        insert_node(&mut self.root, Box::new(Node::new(val)));
    }

    pub fn remove(&mut self, val: i32) {
        remove_node(&mut self.root, val);
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

fn insert_node(bst: &mut Tree, node: Box<Node>) {
    match *bst {
        Some(ref mut tree) => {
            match node.val.cmp(&tree.val) {
                Ordering::Less => insert_node(&mut tree.left, node),
                Ordering::Greater => insert_node(&mut tree.right, node),
                Ordering::Equal => return,
            }
        },
        None => *bst = Some(node),
    }
}

fn pop_leftmost(tree: &mut Tree) -> Tree {
    return match *tree {
        Some(ref mut node) => pop_leftmost(&mut node.left),
        None => None,
    }.or_else(|| tree.take())
}

fn remove_node(bst: &mut Tree, val: i32) {
    let node_to_promote = match *bst {
        Some(ref mut node) => {
            match val.cmp(&node.val) {
                Ordering::Less => return remove_node(&mut node.left, val),
                Ordering::Greater => return remove_node(&mut node.right, val),
                Ordering::Equal => {
                    if node.right.is_some() {
                        let mut leftmost = pop_leftmost(&mut node.right);
                        if let Some(ref mut leftmost_node) = leftmost {
                            leftmost_node.right = node.right.take();
                        }
                        leftmost
                    } else { node.left.take() }
                },
            }
        },
        None => return,
    };
    mem::replace(bst, node_to_promote);
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
    use super::{BinarySearchTree, pop_leftmost};

    #[test]
    fn test_bst_insert() {
        let mut bst = BinarySearchTree::new();
        bst.insert(3);
        bst.insert(5);
        bst.insert(1);
        assert_eq!(bst.inc_order(), vec![1, 3, 5])
    }

    #[test]
    fn test_bst_delete() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10);
        bst.insert(3);
        bst.insert(7);
        bst.insert(9);
        bst.insert(6);
        assert_eq!(bst.inc_order(), vec![3, 6, 7, 9, 10]);
        bst.remove(9);
        assert_eq!(bst.inc_order(), vec![3, 6, 7, 10]);
        bst.remove(3);
        assert_eq!(bst.inc_order(), vec![6, 7, 10]);
        bst.remove(10);
        assert_eq!(bst.inc_order(), vec![6, 7]);
    }

    #[test]
    fn test_bst_pop_leftmost() {
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(4);
        bst.insert(3);
        bst.insert(2);
        bst.insert(1);
        assert_eq!(pop_leftmost(&mut bst.root).unwrap().val, 1);
        assert_eq!(pop_leftmost(&mut bst.root).unwrap().val, 2);
    }
}
