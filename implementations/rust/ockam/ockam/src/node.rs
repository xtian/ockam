#[cfg(feature = "ockam_node_no_std")]
pub use ockam_node_no_std::block_on;

#[cfg(feature = "ockam_node_std")]
pub use ockam_node_std::block_on;

use crate::address::Address;
use crate::worker::{Worker, WorkerState};
use alloc::rc::Rc;
use core::cell::RefCell;

#[derive(Clone)]
struct Message {}

pub struct Node {
    // worker_registry: RefCell<WorkerRegistry>,
}

pub type NodeHandle = Rc<RefCell<Node>>;

impl Node {
    pub fn new() -> Self {
        Node {
           // worker_registry: RefCell::new(WorkerRegistry::new()),
        }
    }

    pub fn new_handle() -> NodeHandle {
        Rc::new(RefCell::new(Node {}))
    }

    pub fn register<T>(&mut self, _worker: Worker<T>) {
        //      self.worker_registry.borrow_mut().insert(worker);
    }

    pub fn start(&mut self, _address: &Address) -> WorkerState {
        /*   let mut registry = self.worker_registry.borrow_mut();
        if let Some(context) = registry.get_mut(address) {
            if let Ok(started) = context.delegate.borrow_mut().starting(&mut context.clone()) {
                if started {
                    return WorkerState::Started;
                }
            }
        };*/
        WorkerState::Started
    }
}

#[cfg(test)]
mod test {
    use crate::node::Node;

    #[test]
    fn test_node() {
        let _node = Node::new_handle();
    }
}
