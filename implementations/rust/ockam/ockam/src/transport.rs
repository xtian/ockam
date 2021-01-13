/// Prototype of a Transport
use crate::address::Addressable;

pub trait Connector<T> {
    fn send_message(&mut self, message: T);
}

pub trait Listener<T>: Addressable {
    fn listen(&mut self);
}
