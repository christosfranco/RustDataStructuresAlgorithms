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
            current.left.to_owned().unwrap().borrow_mut().inorder(result);
        }
        result.push(current.value);
        if !current.right.is_none() {
            current.right.to_owned().unwrap().borrow_mut().inorder(result);
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
            current.left.to_owned().unwrap().borrow_mut().preorder(result);
        }
        if !current.right.is_none() {
            current.right.to_owned().unwrap().borrow_mut().preorder(result);
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
            current.left.to_owned().unwrap().borrow_mut().postorder(result);
        }
        if !current.right.is_none() {
            current.right.to_owned().unwrap().borrow_mut().postorder(result);
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
            if current.left.to_owned().unwrap().borrow_mut().breath_first_search(needle) {
                return true;
            };
        }
        if !current.right.is_none() {
            if current.right.to_owned().unwrap().borrow_mut().breath_first_search(needle) {
                return true;
            };
        }
        return false;
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
        
        let values_insert = vec![5,15,2,5,22,1];
        for i in &values_insert {
            BST::insert(&mut bst, *i);
        }

        let inorder_vector = BST::inorder_traverse(&mut bst);
        assert_eq!(vec![1,2,5,5,10,15,22], inorder_vector);
        
        let preorder_vector = BST::preorder_traverse(&mut bst);
        assert_eq!(vec![10,5,2,1,5,15,22], preorder_vector);

        let postorder_vector = BST::postorder_traverse(&mut bst);
        assert_eq!(vec![1,2,5,5,22,15,10], postorder_vector);

        
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
      
        let values_insert = vec![5,15,2,5,22,1];
        for i in &values_insert {
            BST::insert(&mut bst, *i);
        }
        for i in &values_insert {
            assert_eq!(true, BST::breath_first_search(&mut bst, *i));

        }
        Ok(())
    }
}
