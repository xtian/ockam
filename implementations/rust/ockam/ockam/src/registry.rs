/// Worker Registration and Storage
use crate::address::Address;
use crate::message::MessageDelivery;
use crate::task::{Poll, Task, TaskQueue};
use crate::worker::Worker;
use hashbrown::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

/// Worker context storage type, associated by Address
pub type Workers = Rc<RefCell<HashMap<Address, Worker>>>;

/// High level Worker storage.
#[derive(Default, Clone)]
pub struct WorkerRegistry {
    workers: Workers,
}

/// Standard wrapper for a WorkerRegistry
pub type WorkerRegistryHandle = Rc<RefCell<WorkerRegistry>>;

/// A `TaskQueue` which takes in `Worker` "messages" and registers them into the `Workers` registry
pub type WorkQueue = TaskQueue<Worker, Workers>;

/// High level registration and retrieval of Workers
impl WorkerRegistry {
    pub fn new() -> WorkerRegistryHandle {
        Rc::new(RefCell::new(WorkerRegistry::default()))
    }

    pub fn register(&self, worker: Worker) {
        let address = worker.address();
        self.workers.borrow_mut().insert(address.clone(), worker);
        println!("Worker added at {}", address)
    }

    /// Currently this callback provides a simple reference to the stored `Worker`. Since everything
    /// in the `Worker` has members with interior mutability, there is not a need for us to provide
    /// a mutable reference. I guess a more general solution would allow mutability. It does work
    /// right now, but I have been trying to reduce mutability wherever possible.
    pub fn with_worker(&self, address: &Address, mut handler: impl FnMut(Option<&Worker>)) {
        handler(self.workers.borrow_mut().get(address));
    }
}

/// Deliver messages from Worker inboxes to the workers. Not exactly Registry related, but it has the
/// closest access to the workers to do the operation. Not sold on it though.
impl Poll for WorkerRegistry {
    fn poll(&mut self) -> crate::Result<()> {
        let mut workers = self.workers.borrow_mut();
        let worker_map = workers.values_mut();
        for worker in worker_map {
            worker.deliver();
        }
        Ok(())
    }
}

/// Register a Worker and then call its `starting` callback.
impl Task<Worker, Workers> for WorkerRegistry {
    fn run(&mut self, data: Worker) -> crate::Result<Workers> {
        self.register(data.clone());
        match data
            .callbacks
            .borrow_mut()
            .starting(&mut data.clone(), &self)
        {
            Ok(_) => Ok(self.workers.clone()),
            Err(e) => Err(e),
        }
    }

    /// Expose the internal Task state, for Worker lookups in this case. Not set on the pattern.
    fn state(&self) -> Option<Workers> {
        Some(self.workers.clone())
    }
}
