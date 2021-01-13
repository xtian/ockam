use crate::address::{Address, Addressable};
use alloc::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;

/// Yet another Queue!
pub trait Queue<T> {
    fn enqueue(&mut self, element: T) -> crate::Result<bool>;
    fn dequeue(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
}

impl<T> Queue<T> for VecDeque<T> {
    fn enqueue(&mut self, element: T) -> crate::Result<bool> {
        self.push_back(element);
        Ok(true)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// High level queue allocator
pub fn new_queue<T: 'static>() -> impl Queue<T> {
    VecDeque::<T>::new()
}

/// A Queue which also has an Address
pub trait AddressableQueue<T>: Queue<T> + Addressable {}

/// The workhorse in-memory queue
#[derive(Debug)]
pub struct AddressedVec<T> {
    pub address: Address,
    pub vec: VecDeque<T>,
}

impl<T> Queue<T> for AddressedVec<T> {
    fn enqueue(&mut self, element: T) -> crate::Result<bool> {
        self.vec.enqueue(element)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.vec.dequeue()
    }

    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}

impl<T> Addressable for AddressedVec<T> {
    fn address(&self) -> Address {
        self.address.clone()
    }
}

impl<T> AddressableQueue<T> for AddressedVec<T> {}

pub type QueueHandle<T> = Rc<RefCell<dyn Queue<T>>>;

/// Run a callback for each element in the queue, removing the element.
pub trait Drain<T> {
    fn drain(&mut self, f: impl FnMut(T));
}

impl<T> Drain<T> for dyn AddressableQueue<T> {
    fn drain(&mut self, mut f: impl FnMut(T)) {
        while let Some(element) = self.dequeue() {
            f(element);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::queue::{new_queue, Queue};

    #[test]
    fn test_queue() {
        struct Item;

        let mut queue = new_queue();

        match queue.enqueue(Item {}) {
            Ok(_) => {}
            Err(_) => panic!(),
        };
        match queue.dequeue() {
            Some(_) => {}
            None => panic!(),
        };
    }
}
