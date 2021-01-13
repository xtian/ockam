use crate::address::Address;
use crate::message::{new_message_queue, Message, MessageDelivery, MessageQueue};
use crate::node::NodeHandle;
use crate::queue::AddressableQueue;
use crate::registry::WorkerRegistry;
use alloc::rc::Rc;
use core::cell::RefCell;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WorkerState {
    Started,
    Failed,
}

/// Worker callbacks.
pub trait Callbacks<T> {
    fn handle(&mut self, _message: T, _worker: &mut Worker) -> crate::Result<bool> {
        unimplemented!()
    }

    fn starting(
        &mut self,
        _worker: &mut Worker,

        // TODO experimenting with how to provide context to the callback.
        // I don't like the explicitness of this.
        _worker_registry: &WorkerRegistry,
    ) -> crate::Result<bool> {
        Ok(true)
    }

    fn stopping(&mut self, _worker: &mut Worker) -> crate::Result<bool> {
        Ok(true)
    }
}
pub type MessageCallbacks = Rc<RefCell<dyn Callbacks<Message>>>;

/// High level Worker.
#[derive(Clone)]
pub struct Worker {
    address: Address,
    pub callbacks: MessageCallbacks,
    pub inbox: MessageQueue,
    pub outbox: MessageQueue,
    pub node: NodeHandle,
}

impl Worker {
    pub fn address(&self) -> Address {
        self.address.clone()
    }
}

impl Callbacks<Message> for Worker {
    fn handle(&mut self, message: Message, worker: &mut Worker) -> crate::Result<bool> {
        self.callbacks.borrow_mut().handle(message, worker)
    }
}

/// Invoke the message handling callback for every queued Message.
impl MessageDelivery for Worker {
    fn deliver(&mut self) {
        if self.inbox.borrow().is_empty() {
            return;
        }

        let mut mbox = self.inbox.borrow_mut();
        while !mbox.is_empty() {
            if let Some(message) = mbox.dequeue() {
                let mut callbacks = self.callbacks.borrow_mut();
                let mut worker = self.clone();
                match callbacks.handle(message, &mut worker) {
                    Ok(_) => (),
                    Err(_) => panic!(),
                };
            }
        }
    }
}

/// Wrapper type for creating a Worker given only a closure.
type ClosureHandle<T> = Rc<RefCell<dyn FnMut(&T, &mut Worker)>>;
struct ClosureCallbacks<T> {
    message_handler: Option<ClosureHandle<T>>,
}

impl<T> Callbacks<T> for ClosureCallbacks<T> {
    fn handle(&mut self, message: T, context: &mut Worker) -> crate::Result<bool> {
        if let Some(handler) = self.message_handler.clone() {
            let mut h = handler.borrow_mut();
            h(&message, context);
            Ok(true)
        } else {
            Err(crate::Error::WorkerRuntime) // We should discuss public api error patterns.
        }
    }
}

impl ClosureCallbacks<Message> {
    fn with_closure(f: impl FnMut(&Message, &mut Worker) + 'static) -> ClosureCallbacks<Message> {
        ClosureCallbacks {
            message_handler: Some(Rc::new(RefCell::new(f))),
        }
    }
}

pub type CallbackWrapper = Rc<RefCell<dyn Callbacks<Message>>>;
pub type Mailbox = Rc<RefCell<dyn AddressableQueue<Message>>>;

pub struct WorkerBuilder {
    node: NodeHandle,
    callbacks: Option<CallbackWrapper>,
    address: Option<Address>,
    inbox: Option<Mailbox>,
    address_counter: usize,
}

impl WorkerBuilder {
    pub fn address(&mut self, address_str: &str) -> &mut Self {
        self.address = Some(Address::from(address_str));
        self
    }

    pub fn inbox(&mut self, mailbox: Mailbox) -> &mut Self {
        self.inbox = Some(mailbox);
        self
    }

    pub fn build(&mut self) -> Option<Worker> {
        if self.callbacks.is_none() || self.address.is_none() {
            panic!("Tried to build Context with no Worker or Address")
        }

        let mut which_address = self.address.clone();
        let mut which_inbox = self.inbox.clone();

        if let Some(external_inbox) = which_inbox.clone() {
            which_address = Some(external_inbox.borrow().address())
        } else {
            which_inbox = Some(new_message_queue(
                format!(
                    "{}_in",
                    match self.address.clone() {
                        Some(x) => x,
                        None => panic!(),
                    }
                )
                .into(),
            ));
        }

        if let Some(delegate) = self.callbacks.clone() {
            if let Some(address) = which_address {
                if let Some(inbox) = which_inbox {
                    let outbox = new_message_queue(format!("{}_out", address).into());

                    return Some(Worker {
                        node: self.node.clone(),
                        address,
                        callbacks: delegate,
                        inbox,
                        outbox,
                    });
                }
            }
        }
        None
    }
}

/// Build a new Worker from the given implementation of Message Callbacks.
pub fn with(node: NodeHandle, worker: impl Callbacks<Message> + 'static) -> WorkerBuilder {
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
pub fn with_closure(
    node: NodeHandle,
    handler: impl FnMut(&Message, &mut Worker) + 'static,
) -> WorkerBuilder {
    let closure = ClosureCallbacks::with_closure(handler);
    with(node, closure)
}

#[cfg(test)]
mod test {
    use crate::address::Address;
    use crate::message::Message;
    use crate::node::Host;
    use crate::queue::AddressedVec;
    use crate::worker::{ClosureCallbacks, Worker};
    use alloc::collections::VecDeque;
    use alloc::rc::Rc;
    use core::cell::RefCell;

    #[test]
    fn test_worker() {
        let host = Host::new();

        let node = host.node.unwrap();

        let work = Rc::new(RefCell::new(|_message: &Message, _context: &mut Worker| {}));

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
            outbox: Rc::new(RefCell::new(AddressedVec {
                address: Address::from("test_outbox"),
                vec: VecDeque::new(),
            })),
        };

        let mut callbacks = worker.callbacks.borrow_mut();

        match callbacks.handle(Message::empty(), &mut worker.clone()) {
            Ok(x) => x,
            Err(_) => panic!(),
        };
    }
}
