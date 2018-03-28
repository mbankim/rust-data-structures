use std::cmp::Ordering;
use rand::random;

type Tree = Option<Box<Node>>;

pub struct Treap {
    root: Tree,
}

impl Treap {
    pub fn new() -> Self {
        Treap { root : None }
    }

    pub fn insert(&mut self, val: u32) {
        insert_node(&mut self.root, val);
    }

    pub fn remove(&mut self, val: u32) -> u32 {
        return val;
    }

    /*
    pub fn split(&mut self, val: u32) -> Treap {
    }
    */

    pub fn merge(&mut self, other: &mut Treap) {
    }

    pub fn inc_order(&self) -> Vec<u32> {
        return get_inorder(&self.root);
    }

    pub fn get_depths(&self) -> Vec<u32> {
        return count_depth(&self.root, 0u32);
    }
}

struct Node {
    val: u32,
    priority: u32,
    left: Tree,
    right: Tree,
}

impl Node {
    fn new(val: u32) -> Self {
        Node {
            val,
            priority: random::<u32>(),
            left: None,
            right: None,
        }
    }
}

fn insert_node(tree: &mut Tree, val: u32) {
    match *tree {
        Some(ref mut node) => {
            match val.cmp(&node.val) {
                Ordering::Less => insert_node(&mut node.left, val),
                Ordering::Greater => insert_node(&mut node.right, val),
                Ordering::Equal => return,
            }
        },
        None => *tree = Some(Box::new(Node::new(val))),
    }
}

fn get_inorder(tree: &Tree) -> Vec<u32> {
    match *tree {
        Some(ref node) => {
            let mut vec = get_inorder(&node.left);
            vec.push(node.val);
            vec.append(&mut get_inorder(&node.right));
            vec
        },
        None => Vec::new(),
    }
}

fn count_depth(tree: &Tree, depth: u32) -> Vec<u32> {
    match *tree {
        Some(ref node) => {
            let mut vec = count_depth(&node.left, depth + 1);
            vec.append(&mut count_depth(&node.right, depth + 1));
            if vec.is_empty() {
                vec.push(depth);
            }
            vec
        },
        None => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{Treap};

    #[test]
    fn test_treap_insert() {
        let mut treap = Treap::new();
        for i in 0u32..100u32 {
            treap.insert(i);
        }
        assert_eq!(treap.inc_order(), (0u32..100u32).collect::<Vec<u32>>());
    }
}
