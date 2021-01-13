use crate::message::Message;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Router {}

pub type RouterHandle = Rc<RefCell<Router>>;

impl Router {
    pub fn new() -> RouterHandle {
        Rc::new(RefCell::new(Router {}))
    }
    pub fn route(&mut self, _message: Message) {}
}
