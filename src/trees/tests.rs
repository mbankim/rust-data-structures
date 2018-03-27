#[cfg(test)]
mod tests {
    use super::{BinarySearchTree, pop_leftmost};
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
        bst.remove(9);
        assert_eq!(bst.inc_order(), vec![3, 6, 7, 10]);
        bst.remove(3);
        assert_eq!(bst.inc_order(), vec![6, 7, 10]);
        bst.remove(10);
        assert_eq!(bst.inc_order(), vec![6, 7]);
    }

    #[test]
    fn test_pop_leftmost() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(4);
        bst.insert(3);
        bst.insert(2);
        bst.insert(1);
        assert_eq!(pop_leftmost(&mut bst.root).unwrap().val, 1);
        assert_eq!(pop_leftmost(&mut bst.root).unwrap().val, 2);
    }
}
