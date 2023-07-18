#[derive(Debug, Clone)]
pub struct SQueue<T> {
    stack: Vec<T>,
    length: usize,
}

const UMAX: usize = 9223372036854775807;


#[allow(dead_code)]
impl<T> SQueue<T> {
    pub fn new() -> Self {
        SQueue {
            stack: Vec::new(),
            length: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn pop(&mut self) -> Option<T> {
        let res = self.stack.pop();
        match res {
            None => return None,
            Some(val) => {
                self.length -= 1;
                return Some(val);
            }
        }
    }

    pub fn push(&mut self, item: T) {
        if self.length >= UMAX {
            return;
        }
        self.length += 1;
        self.stack.push(item)
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
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
        let mut queue: SQueue<i32> = SQueue::new();
        assert_eq!(queue.length(), 0);

        assert_eq!(queue.peek(), None);
        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.pop(), None);

        Ok(())
    }

    #[test]
    fn test_push_q() -> Result<(), &'static str> {
        const SIZE: usize = 10;
        let mut queue: SQueue<i32> = SQueue::new();
        assert_eq!(queue.length(), 0);

        assert_eq!(queue.peek(), None);


        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.pop(), None);
        queue.push(1);
        assert_eq!(queue.peek(), Some(&1));

        assert_eq!(queue.length(), 1);

        assert_eq!(queue.is_empty(), false);

        queue.push(2);

        assert_eq!(queue.length(), 2);

        assert_eq!(queue.peek(), Some(&2)); // filo stack so should be 2

        assert_eq!(queue.pop(), Some(2));

        assert_eq!(queue.pop(), Some(1));
        Ok(())
    }
}
