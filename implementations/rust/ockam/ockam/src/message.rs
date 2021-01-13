use crate::address::{Address, Addressable};
use crate::queue::{AddressableQueue, Queue};
use crate::route::Route;
use crate::worker::Worker;
use alloc::collections::VecDeque;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

#[cfg(feature = "json")]
use serde::*;

pub type Payload = Vec<u8>;

#[derive(Debug, Copy, Clone)]
#[cfg(feature = "json")]
#[derive(Serialize)]
pub enum MessageType {
    Payload,
}

#[derive(Debug, Clone)]
#[cfg(feature = "json")]
#[derive(Serialize)]
pub struct Message {
    pub message_type: MessageType,
    pub onward_route: Route,
    pub return_route: Route,
    pub payload: Payload,
}

impl From<Payload> for Message {
    fn from(payload: Payload) -> Self {
        Message {
            message_type: MessageType::Payload,
            onward_route: Route::default(),
            return_route: Route::default(),
            payload,
        }
    }
}

impl Into<Message> for &str {
    fn into(self) -> Message {
        Message::from(self.as_bytes().to_vec())
    }
}

impl Into<Message> for i32 {
    fn into(self) -> Message {
        Message::from(self.to_le_bytes().to_vec())
    }
}

pub trait OnwardTo<T> {
    fn onward_to(&mut self, onward: T) -> &mut Self;
}

impl Message {
    pub fn empty() -> Self {
        Message::from(vec![])
    }

    pub fn onward_add(&mut self, address: Address) {
        self.onward_route.append(address.into());
    }

    pub fn return_add(&mut self, address: Address) {
        self.onward_route.append(address.into());
    }
}

pub struct MessageBuilder {
    message_type: Option<MessageType>,
    payload: Option<Payload>,
    onward_route: Route,
    return_route: Route,
}

impl MessageBuilder {
    pub fn message() -> Self {
        MessageBuilder {
            message_type: None,
            payload: None,
            onward_route: Route::default(),
            return_route: Route::default(),
        }
    }

    pub fn message_type(&mut self, message_type: MessageType) -> &mut Self {
        self.message_type = Some(message_type);
        self
    }

    pub fn payload(&mut self, payload: Payload) -> &mut Self {
        self.payload = Some(payload);
        self
    }

    pub fn empty(&mut self) -> &mut Self {
        self.payload = Some(vec![]);
        self
    }

    pub fn onward_route(&mut self, onward_route: Route) -> &mut Self {
        self.onward_route = onward_route;
        self
    }

    pub fn return_route(&mut self, return_route: Route) -> &mut Self {
        self.return_route = return_route;
        self
    }

    pub fn return_to(&mut self, ret: &str) -> &mut Self {
        self.return_route.append(ret.into());
        self
    }

    pub fn build(&self) -> Message {
        let message_type = if let Some(t) = self.message_type {
            t
        } else {
            MessageType::Payload
        };

        let payload = if let Some(p) = &self.payload {
            p.to_vec()
        } else {
            vec![]
        };

        Message {
            message_type,
            onward_route: self.onward_route.clone(),
            return_route: self.return_route.clone(),
            payload,
        }
    }
}

impl OnwardTo<&str> for MessageBuilder {
    fn onward_to(&mut self, onward: &str) -> &mut Self {
        self.onward_route.append(onward.into());
        self
    }
}

impl OnwardTo<String> for MessageBuilder {
    fn onward_to(&mut self, onward: String) -> &mut Self {
        self.onward_route.append(onward.as_str().into());
        self
    }
}

impl OnwardTo<Address> for MessageBuilder {
    fn onward_to(&mut self, onward: Address) -> &mut Self {
        self.onward_route.append(onward.into());
        self
    }
}

struct AddressedMessageQueue {
    address: Address,
    inner: VecDeque<Message>,
}

impl AddressedMessageQueue {
    fn new(address: Address) -> Self {
        AddressedMessageQueue {
            address,
            inner: VecDeque::new(),
        }
    }
}

impl Queue<Message> for AddressedMessageQueue {
    fn enqueue(&mut self, element: Message) -> crate::Result<bool> {
        self.inner.enqueue(element)
    }

    fn dequeue(&mut self) -> Option<Message> {
        self.inner.dequeue()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Addressable for AddressedMessageQueue {
    fn address(&self) -> Address {
        self.address.clone()
    }
}

impl AddressableQueue<Message> for AddressedMessageQueue {}

pub type MessageQueue = Rc<RefCell<dyn AddressableQueue<Message>>>;

pub fn new_message_queue(address: Address) -> MessageQueue {
    Rc::new(RefCell::new(AddressedMessageQueue::new(address)))
}

pub trait MessageDelivery {
    fn deliver(&mut self);
}

pub struct MessageSender {}

impl MessageSender {
    pub fn send(worker: &mut Worker, message: Message) {
        let mut mailbox = worker.inbox.borrow_mut();
        if let Err(e) = mailbox.enqueue(message) {
            panic!("Couldn't enqueue message: {:?}", e)
        }
    }
}
