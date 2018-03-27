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

