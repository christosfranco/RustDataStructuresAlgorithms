use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct VQueue<T> {
    queue: VecDeque<T>,
}

#[allow(dead_code)]
impl<T> VQueue<T> {
    pub fn new_V(value: usize) -> Self {
        let mut queue = VecDeque::with_capacity(value);
        queue.make_contiguous();
        VQueue { queue }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push_back(item)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        // VecDeque::pop_front(self.queue);
        // self.queue.remove(0);
        return self.queue.pop_front();
    }

    pub fn length(&self) -> usize {
        self.queue.len()
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    pub fn peek(&self) -> Option<&T> {
        self.queue.get(0)
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
        let mut queue: VQueue<i32> = VQueue::new_V(SIZE);
        assert_eq!(queue.length(), 0);

        assert_eq!(queue.peek(), None);
        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.dequeue(), None);

        Ok(())
    }

    #[test]
    fn test_push_q() -> Result<(), &'static str> {
        const SIZE: usize = 10;
        let mut queue: VQueue<i32> = VQueue::new_V(SIZE);
        assert_eq!(queue.length(), 0);

        assert_eq!(queue.peek(), None);
        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.dequeue(), None);
        queue.enqueue(1);
        assert_eq!(queue.peek(),Some(&1));

        assert_eq!(queue.length(), 1);

        assert_eq!(queue.is_empty(), false);

        
        queue.enqueue(2);
        
        assert_eq!(queue.length(), 2);
        
        assert_eq!(queue.peek(),Some(&1)); // should still peek at 1 because fifo so 1 should pop first

        assert_eq!(queue.dequeue(), Some(1));
        
        assert_eq!(queue.dequeue(), Some(2));
        Ok(())
    }
}
