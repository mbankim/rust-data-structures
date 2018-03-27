use std::cmp::Ordering;
use std::mem;

type Tree = Option<Box<Node>>;

pub struct BST {
    root: Tree,
}

impl BST {
    pub fn new() -> Self {
        BST{ root : None }
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

fn remove_node(bst: &mut Tree, val: i32) {
    match *bst {
        Some(ref mut tree) => {
            match val.cmp(&tree.val) {
                Ordering::Less => {
                    remove_node(&mut tree.left, val);
                    return
                }
                Ordering::Greater => {
                    remove_node(&mut tree.right, val);
                    return
                }
                Ordering::Equal => (),
            }
        },
        None => return,
    }

    let mut node = bst.take().unwrap();
    if let Some(right) = node.right.take() {
        insert_node(&mut node.left, right);
    }
    mem::replace(bst, node.left);
}


fn dfs(bst: &Tree, result: &mut Vec<i32>) {
    match *bst {
        Some(ref tree) => {
            println!("visiting node with value {}", tree.val);
            dfs(&tree.left, result);
            result.push(tree.val);
            dfs(&tree.right, result);
        },
        None => (),
    }
}

#[cfg(test)]
mod tests {
    use super::BST;
    #[test]
    fn test_insert() {
        let mut bst = BST::new();
        bst.insert(3);
        bst.insert(5);
        bst.insert(1);

        assert_eq!(bst.inc_order(), vec![1, 3, 5])
    }

    #[test]
    fn test_delete() {
        let mut bst = BST::new();
        bst.insert(10);
        bst.insert(3);
        bst.insert(7);
        bst.insert(9);
        bst.insert(6);
        assert_eq!(bst.inc_order(), vec![3, 6, 7, 9, 10]);

        bst.remove(3);
        bst.remove(9);
        assert_eq!(bst.inc_order(), vec![6, 7, 10]);
        bst.remove(10);
        assert_eq!(bst.inc_order(), vec![6, 7]);
    }
}
