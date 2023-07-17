#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum NodeValue {
    I32(i32),
    Bool(bool),
}
type Link<'a, T> = Option<Box<Node<'a, T>>>;

#[derive(Debug, Clone)]
pub struct Node<'b, T>
where
    T: Clone + 'static,
{
    value: T,
    // prev: Option<Box<Node<T>>>,
    next: &'b Link<'b, T>,
}
#[derive(Debug, Clone)]
pub struct Queue<'a, T: std::clone::Clone + 'static> {
    length: i32,
    head: Link<'a, T>,
    tail: Link<'a, T>,
}

// pub struct Iter<'a, T: 'a + std::clone::Clone> {
//     next: &'a Link<T>,
// }

// None  | Some(value)
impl<T: std::clone::Clone> Queue<'_, T> {
    pub fn new() -> Self {
        Queue {
            length: 0,
            head: None,
            tail: None,
        }
    }

    // push to the start of Q
    pub fn enqueue(&mut self, value: T) {
        let new_node = Box::new(Node {
            value: value.clone(),
            next: &None,
        });
        // if self.tail.is_none() {
        //     self.tail = Some(new_node.clone());
        //     self.head = Some(new_node);
        //     self.length += 1;
        //     return;
        // } else {
        // if there is an element update the Q
        let extracted_node: Option<Box<Node<T>>> = self.tail.take();
        match extracted_node {
            // if there exist an element, ie tail is Some()
            // create new node at tail
            // set previous tail.next to new node
            Some(mut val) => {
                println!("HIIIIIIIIIIIII");
                val.next = &Some(new_node.clone());
                self.tail = Some(new_node);
                self.length += 1;
                return;
            }
            // if Q empty set head and tail to be the same
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
                // if let Some(node) = self.head.as_mut() {
                //     node.next = &self.tail.as_ref().map(|boxed_node| Box::new(**boxed_node));
                // }
                self.length += 1;
                return;
            }
        }
        // }
    }

    // pop from the end of Q
    // return None if no value at head
    // reutnr Some(T) if value exist at head
    pub fn dequeue(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None; // Empty queue, nothing to dequeue
        }

        self.length -= 1;

        let extracted_node: Option<Box<Node<T>>> = self.head.take();
        match extracted_node {
            Some(val) => {
                self.head = val.next.as_ref().cloned();

                return Some(val.value);
            }
            None => return None,
        }
    }

    // look at head of Q and get
    pub fn peek(&mut self) -> Option<&Box<Node<T>>> {
        let var = self.head.as_ref();
        return var;
    }
}

#[cfg(test)]
mod tests {
    // use std::{collections::HashSet, error::Error};
    // use std::io;

    use super::*;

    #[test]
    fn test_init_arr() -> Result<(), &'static str> {
        const SIZE: usize = 10;
        let queue: Queue<NodeValue> = Queue::new();
        Ok(())
    }
}
