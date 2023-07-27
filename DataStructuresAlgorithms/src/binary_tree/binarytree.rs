use std::cell::RefCell;
use std::rc::Rc;
use std::thread::current;

type Node = Rc<RefCell<BST>>;

pub struct BST {
    pub root: NODE,
}

pub struct NODE {
    pub value: i32,
    pub left: Option<Node>,
    pub right: Option<Node>,
}

impl NODE {
    pub fn new(value: i32) -> NODE {
        NODE {
            value: value,
            left: None,
            right: None,
        }
    }
}

impl BST {
    pub fn new(value: i32) -> Self {
        BST {
            root: NODE::new(value),
        }
    }

    pub fn insert(&mut self, value: i32) {
        let mut current = &mut self.root;
        if value < current.value {
            if current.left.is_none() {
                current.left = Some(Rc::new(RefCell::new(BST::new(value))));
            } else {
                current.left.to_owned().unwrap().borrow_mut().insert(value);
            }
        } else {
            if current.right.is_none() {
                current.right = Some(Rc::new(RefCell::new(BST::new(value))));
            } else {
                current.right.to_owned().unwrap().borrow_mut().insert(value);
            }
        }
    }

    pub fn print(&mut self) {
        // Inorder Traverse to Print all Values
        let current = &mut self.root;
        if !current.left.is_none() {
            current.left.to_owned().unwrap().borrow_mut().print();
        }
        println!("{}", current.value);
        if !current.right.is_none() {
            current.right.to_owned().unwrap().borrow_mut().print();
        }
    }

    pub fn inorder_traverse(&mut self) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        self.inorder(&mut result);
        result
    }

    fn inorder(&mut self, result: &mut Vec<i32>) -> () {
        let current = &mut self.root;
        if !current.left.is_none() {
            current
                .left
                .to_owned()
                .unwrap()
                .borrow_mut()
                .inorder(result);
        }
        result.push(current.value);
        if !current.right.is_none() {
            current
                .right
                .to_owned()
                .unwrap()
                .borrow_mut()
                .inorder(result);
        }
    }

    pub fn preorder_traverse(&mut self) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        self.preorder(&mut result);
        result
    }

    fn preorder(&mut self, result: &mut Vec<i32>) -> () {
        let current = &mut self.root;
        result.push(current.value);
        if !current.left.is_none() {
            current
                .left
                .to_owned()
                .unwrap()
                .borrow_mut()
                .preorder(result);
        }
        if !current.right.is_none() {
            current
                .right
                .to_owned()
                .unwrap()
                .borrow_mut()
                .preorder(result);
        }
    }

    pub fn postorder_traverse(&mut self) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        self.postorder(&mut result);
        result
    }

    fn postorder(&mut self, result: &mut Vec<i32>) -> () {
        let current = &mut self.root;
        if !current.left.is_none() {
            current
                .left
                .to_owned()
                .unwrap()
                .borrow_mut()
                .postorder(result);
        }
        if !current.right.is_none() {
            current
                .right
                .to_owned()
                .unwrap()
                .borrow_mut()
                .postorder(result);
        }
        result.push(current.value);
    }

    pub fn breath_first_search(&mut self, needle: i32) -> bool {
        let current = &mut self.root;
        // println!("{}", current.value);
        if current.value == needle {
            // println!("FOUND THE NEEDLE");
            return true;
        }
        if !current.left.is_none() {
            if current
                .left
                .to_owned()
                .unwrap()
                .borrow_mut()
                .breath_first_search(needle)
            {
                return true;
            };
        }
        if !current.right.is_none() {
            if current
                .right
                .to_owned()
                .unwrap()
                .borrow_mut()
                .breath_first_search(needle)
            {
                return true;
            };
        }
        return false;
    }

    // compare two BST to check equiality
    pub fn compare(bst1: &mut BST, bst2: &mut BST) -> bool {
        let node1 = &mut bst1.root;
        let node2 = &mut bst2.root;

        // XOR -> NOT , returns true if both (false or true)
        let rightnodes_states = !(node1.right.is_none() ^ node2.right.is_none());
        let leftnodes_states = !(node1.left.is_none() ^ node2.left.is_none());
        let current_state = node1.value == node2.value;

        println!("node1.value: {}, node2.value: {}", node1.value, node2.value);
        // if both left state and right state are the same return true (this check None or Some(), will not check the individual value)
        // this reduces the number of operations becuase 1 less recursion has to be done.
        // also checks that the current value is the same.
        let final_state = rightnodes_states && leftnodes_states && current_state;
        println!("FINAL STATE {}", final_state);
        if !final_state {
            return false;
        }

        let mut recurse_right = rightnodes_states;
        if !(node1.right.is_none() || node2.right.is_none()) {
            recurse_right = Self::compare(
                &mut node1.right.to_owned().unwrap().borrow_mut(),
                &mut node2.right.to_owned().unwrap().borrow_mut(),
            );
        }

        let mut recurse_left = leftnodes_states;
        if !(node1.left.is_none() || node2.left.is_none()) {
            recurse_left = Self::compare(
                &mut node1.left.to_owned().unwrap().borrow_mut(),
                &mut node2.left.to_owned().unwrap().borrow_mut(),
            );
        }
        return recurse_left && recurse_right;
    }
}

#[cfg(test)]
mod tests {
    // use std::{collections::HashSet, error::Error};
    // use std::io;

    use std::vec;

    use super::*;

    #[test]
    fn test_traverses() -> Result<(), &'static str> {
        let mut bst = BST::new(10); // Root Node

        let values_insert = vec![5, 15, 2, 5, 22, 1];
        for i in &values_insert {
            BST::insert(&mut bst, *i);
        }

        let inorder_vector = BST::inorder_traverse(&mut bst);
        assert_eq!(vec![1, 2, 5, 5, 10, 15, 22], inorder_vector);

        let preorder_vector = BST::preorder_traverse(&mut bst);
        assert_eq!(vec![10, 5, 2, 1, 5, 15, 22], preorder_vector);

        let postorder_vector = BST::postorder_traverse(&mut bst);
        assert_eq!(vec![1, 2, 5, 5, 22, 15, 10], postorder_vector);

        //          10
        //        /    \
        //       5      15
        //     /   \      \
        //    2     5      22
        //   /
        //  1
        Ok(())
    }

    #[test]
    fn test_push_q() -> Result<(), &'static str> {
        let mut bst = BST::new(10); // Root Node

        let values_insert = vec![5, 15, 2, 5, 22, 1];
        for i in &values_insert {
            BST::insert(&mut bst, *i);
        }
        for i in &values_insert {
            assert_eq!(true, BST::breath_first_search(&mut bst, *i));
        }
        Ok(())
    }

    #[test]
    fn test_compare_trees_equal() -> Result<(), &'static str> {
        let values_insert = vec![5, 15, 2, 5, 22, 1];

        let mut bst = BST::new(10); // Root Node
        let mut bst2 = BST::new(10); // Root Node

        // insert bst
        for i in &values_insert {
            BST::insert(&mut bst, *i);
        }

        // insert bst2
        for i in &values_insert {
            BST::insert(&mut bst2, *i);
        }

        assert_eq!(true, BST::compare(&mut bst, &mut bst2));

        Ok(())
    }

    #[test]
    fn test_compare_trees_unequal() -> Result<(), &'static str> {
        let values_insert = vec![5, 15, 2, 5, 22, 1];

        let mut bst = BST::new(10); // Root Node
        let mut bst2 = BST::new(9); // Root Node

        // insert bst
        for i in &values_insert {
            BST::insert(&mut bst, *i);
        }

        // insert bst2
        for i in &values_insert {
            BST::insert(&mut bst2, *i);
        }

        assert_eq!(false, BST::compare(&mut bst, &mut bst2));

        Ok(())
    }


    
    #[test]
    fn test_compare_trees_unequal_depth() -> Result<(), &'static str> {
        let values_insert = vec![5, 15, 2, 5, 22, 1];
        let values_insert2 = vec![5, 15, 2, 5, 25, 1];


        let mut bst = BST::new(10); // Root Node
        let mut bst2 = BST::new(10); // Root Node

        // insert bst
        for i in &values_insert {
            BST::insert(&mut bst, *i);
        }

        // insert bst2
        for i in &values_insert2 {
            BST::insert(&mut bst2, *i);
        }

        assert_eq!(false, BST::compare(&mut bst, &mut bst2));

        Ok(())
    }

}
