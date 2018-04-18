use std::cmp::Ordering;
use std::mem;
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
        self.root = Some(insert_node(&mut self.root, val));
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

fn insert_node(tree: &mut Tree, val: u32) -> Box<Node> {
    let mut cur_tree = tree.take();
    cur_tree.map_or(
        Box::new(Node::new(val)),
        |mut node| {
            match val.cmp(&node.val) {
                Ordering::Less => {
                    let mut new_node = insert_node(&mut node.left, val);
                    match new_node.priority.cmp(&node.priority) {
                        Ordering::Less => {
                            // rotate
                            let mut right_child = new_node.right.take();
                            mem::replace(&mut node.left, right_child);
                            mem::replace(&mut new_node.right, Some(node));
                            new_node
                        },
                        Ordering::Greater | Ordering::Equal => {
                            node.left = Some(new_node);
                            node
                        },
                    }
                }
                Ordering::Greater | Ordering::Equal => {
                    let mut new_node = insert_node(&mut node.right, val);
                    match new_node.priority.cmp(&node.priority) {
                        Ordering::Less => {
                            // rotate
                            let mut left_child = new_node.left.take();
                            mem::replace(&mut node.right, left_child);
                            mem::replace(&mut new_node.left, Some(node));
                            new_node
                        },
                        Ordering::Greater | Ordering::Equal => {
                            node.right = Some(new_node);
                            node
                        },
                    }
                }
            }
        }
    )
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
    use super::{Treap, Node};

    fn assert_heap_property(tree: &Option<Box<Node>>, parent_priority: u32) {
        if let Some(ref node) = *tree {
            assert!(parent_priority <= node.priority);
            assert_heap_property(&node.right, node.priority);
            assert_heap_property(&node.left, node.priority);
        }
    }

    #[test]
    fn test_treap_insert() {
        let mut treap = Treap::new();
        for i in 0u32..100u32 {
            treap.insert(i);
        }
        assert_eq!(treap.inc_order(), (0u32..100u32).collect::<Vec<u32>>());
        assert_heap_property(&treap.root, 0);
    }
}
