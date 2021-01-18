use crate::address::{Address, Addressable};
use crate::node::NodeHandle;
use crate::queue::{new_queue, AddressableQueue, QueueHandle};
use alloc::rc::Rc;
use core::cell::RefCell;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WorkerState {
    Started,
    Failed,
}

/// Worker callbacks.
pub trait Callbacks<T> {
    fn handle(&mut self, _message: T, _worker: &mut Worker<T>) -> crate::Result<bool> {
        unimplemented!()
    }

    fn starting(&mut self, _worker: &mut Worker<T>) -> crate::Result<bool> {
        Ok(true)
    }

    fn stopping(&mut self, _worker: &mut Worker<T>) -> crate::Result<bool> {
        Ok(true)
    }
}

pub type CallbackHandler<T> = Rc<RefCell<dyn Callbacks<T>>>;

/// High level Worker.
#[derive(Clone)]
pub struct Worker<T> {
    address: Address,
    pub callbacks: CallbackHandler<T>,
    pub inbox: QueueHandle<T>,
    pub node: NodeHandle,
}

impl<T> Addressable for Worker<T> {
    fn address(&self) -> Address {
        self.address.clone()
    }
}

impl<T> Callbacks<T> for Worker<T> {
    fn handle(&mut self, message: T, worker: &mut Worker<T>) -> crate::Result<bool> {
        self.callbacks.borrow_mut().handle(message, worker)
    }
}

/// Wrapper type for creating a Worker given only a closure.
type ClosureHandle<T> = Rc<RefCell<dyn FnMut(&T, &mut Worker<T>)>>;
struct ClosureCallbacks<T> {
    message_handler: Option<ClosureHandle<T>>,
}

impl<T> Callbacks<T> for ClosureCallbacks<T> {
    fn handle(&mut self, message: T, context: &mut Worker<T>) -> crate::Result<bool> {
        if let Some(handler) = self.message_handler.clone() {
            let mut h = handler.borrow_mut();
            h(&message, context);
            Ok(true)
        } else {
            Err(crate::Error::WorkerRuntime) // We should discuss public api error patterns.
        }
    }
}

impl<T> ClosureCallbacks<T> {
    fn with_closure(f: impl FnMut(&T, &mut Worker<T>) + 'static) -> ClosureCallbacks<T> {
        ClosureCallbacks {
            message_handler: Some(Rc::new(RefCell::new(f))),
        }
    }
}

pub type Mailbox<T> = Rc<RefCell<dyn AddressableQueue<T>>>;

pub struct WorkerBuilder<T> {
    node: NodeHandle,
    callbacks: Option<CallbackHandler<T>>,
    address: Option<Address>,
    inbox: Option<Mailbox<T>>,
    address_counter: usize,
}

impl<T: 'static> WorkerBuilder<T> {
    pub fn address(&mut self, address_str: &str) -> &mut Self {
        self.address = Some(Address::from(address_str));
        self
    }

    pub fn inbox(&mut self, mailbox: Mailbox<T>) -> &mut Self {
        self.inbox = Some(mailbox);
        self
    }

    pub fn build(&mut self) -> Option<Worker<T>> {
        if self.callbacks.is_none() || self.address.is_none() {
            panic!("Tried to build Context with no Worker or Address")
        }

        let mut which_address = self.address.clone();
        let mut which_inbox = self.inbox.clone();

        let default_queue = new_queue(format!(
            "{}_in",
            match self.address.clone() {
                Some(x) => x,
                None => panic!(),
            }
        ));

        if let Some(external_inbox) = which_inbox.clone() {
            which_address = Some(external_inbox.borrow().address())
        } else {
            which_inbox = Some(default_queue);
        }

        if let Some(delegate) = self.callbacks.clone() {
            if let Some(address) = which_address {
                if let Some(inbox) = which_inbox {
                    return Some(Worker {
                        node: self.node.clone(),
                        address,
                        callbacks: delegate,
                        inbox,
                    });
                }
            }
        }
        None
    }
}

/// Build a new Worker from the given implementation of Message Callbacks.
pub fn with<T>(node: NodeHandle, worker: impl Callbacks<T> + 'static) -> WorkerBuilder<T> {
    let mut builder = WorkerBuilder {
        address: None,
        inbox: None,
        address_counter: 1000,
        callbacks: None,
        node,
    };

    builder.callbacks = Some(Rc::new(RefCell::new(worker)));
    builder.address = Some(Address::new(builder.address_counter));
    builder
}

/// Build a Worker from a closure.
pub fn with_closure<T: 'static>(
    node: NodeHandle,
    handler: impl FnMut(&T, &mut Worker<T>) + 'static,
) -> WorkerBuilder<T> {
    let closure = ClosureCallbacks::with_closure(handler);
    with(node, closure)
}

#[cfg(test)]
mod test {
    use crate::address::Address;
    use crate::node::Node;
    use crate::queue::AddressedVec;
    use crate::worker::{ClosureCallbacks, Worker};
    use alloc::collections::VecDeque;
    use alloc::rc::Rc;
    use core::cell::RefCell;

    #[derive(Clone)]
    struct Thing {}

    #[test]
    fn test_worker() {
        let node = Node::new_handle();

        let work = Rc::new(RefCell::new(
            |_message: &Thing, _context: &mut Worker<Thing>| {},
        ));

        let worker = Worker {
            node,
            address: Address::from("test"),
            callbacks: Rc::new(RefCell::new(ClosureCallbacks {
                message_handler: Some(work),
            })),
            inbox: Rc::new(RefCell::new(AddressedVec {
                address: Address::from("test_inbox"),
                vec: VecDeque::new(),
            })),
        };

        let mut callbacks = worker.callbacks.borrow_mut();

        match callbacks.handle(Thing {}, &mut worker.clone()) {
            Ok(x) => x,
            Err(_) => panic!(),
        };
    }
}
