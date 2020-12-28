extern crate alloc;

use alloc::rc::Rc;
use core::cell::RefCell;

use super::Worker;

pub struct Builder {
    address: String,
    worker: Rc<RefCell<dyn Worker>>,
}

impl Builder {
    pub fn with_address<A: ToString>(&mut self, address: A) -> &mut Builder {
        self.address = address.to_string();
        self
    }

    pub fn start(&mut self) -> String {
        let w = self.worker.borrow();
        w.starting();

        return self.address.clone();
    }
}

pub fn from(worker: impl Worker + 'static) -> Builder {
    Builder {
        address: String::from("default"),
        worker: Rc::new(RefCell::new(worker)),
    }
}
