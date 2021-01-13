#[cfg(feature = "ockam_node_no_std")]
pub use ockam_node_no_std::block_on;

#[cfg(feature = "ockam_node_std")]
pub use ockam_node_std::block_on;

use crate::address::{Address, Addressable};
use crate::message::Message;
use crate::queue::Drain;
use crate::registry::{WorkQueue, WorkerRegistry, WorkerRegistryHandle};
use crate::router::{Router, RouterHandle};
use crate::task::{Poll, Task, TaskQueue};
use crate::worker::Worker;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use std::time::Duration;

/// The Host currently exists in order to hold resources outside of the Node, which need mutable
/// access to the "Node". Node here means high level routing logic and other APIs that worker callbacks
/// need to have mutable/borrow-able access to.
#[derive(Clone)]
pub struct Host {
    worker_registry: WorkerRegistryHandle,
    worker_registry_queue: WorkQueue,
    pub node: Option<NodeHandle>,
}

#[derive(Clone)]
pub struct Node {
    // TODO this probably needs to be broken out like Registration. In that case what is Node?
    // Anything? Is Host actually Node? An API Facade?
    _router: RouterHandle,
    pub host: Option<Host>,
}

impl Poll for Host {
    fn poll(&mut self) -> crate::Result<()> {
        // Process pending Registrations.
        self.worker_registry_queue.safe_poll();

        self.worker_registry.borrow_mut().safe_poll();

        // TODO WIP: this is hacky/broken outbox delivery, sending outbound queued Messages from a Worker to Node.
        // Probably should be in WorkerRegistry and likely a Task
        if let Some(node_handle) = &self.node {
            let node = node_handle.borrow();
            let registry = self.worker_registry.borrow_mut();
            if let Some(workers) = registry.state() {
                for worker in workers.borrow_mut().values_mut() {
                    worker.outbox.borrow_mut().drain(|message: Message| {
                        node.route(message);
                    });
                }
            } else {
                println!("No workers");
            }
        }

        Ok(())
    }
}

/// The idea behind Runner is to encapsulate execution of the entire system and step it, using
/// whichever mechanism. TODO Should be broken into pure trait and std implementation which can run
/// on a background thread, or can run this polling logic below.s
pub trait Runner {
    fn delay(&self) {
        std::thread::sleep(Duration::from_millis(20))
    }

    fn run_for(&mut self, cycles: usize);

    fn run_forever(&mut self) -> ! {
        loop {
            self.run_for(usize::max_value())
        }
    }
}

impl Runner for Host {
    fn run_for(&mut self, cycles: usize) {
        let mut c = cycles;
        while self.poll().is_ok() {
            match std::io::stdout().flush() {
                Ok(_) => {}
                Err(_) => panic!(),
            };

            self.delay();
            if c == 0 {
                break;
            }
            c -= 1;
        }
    }
}

/// Highest level data structure
impl Host {
    pub fn new() -> Host {
        let worker_registry = WorkerRegistry::new();
        let worker_registry_queue = TaskQueue::new(worker_registry.clone());
        let node = Node::new_handle();

        let host = Host {
            worker_registry,
            worker_registry_queue,
            node: Some(node),
        };

        // Give the Node a reference to the Host. This is circular, but also a singleton. Kinda hacky.
        let node = host.clone().node.unwrap();
        let mut node_mut = node.borrow_mut();
        node_mut.host = Some(host.clone());

        host
    }
}

impl Default for Host {
    fn default() -> Self {
        Host::new()
    }
}

impl Node {
    pub fn new() -> Node {
        let _router = Router::new();
        Node {
            _router,
            host: None,
        }
    }

    pub fn new_handle() -> NodeHandle {
        Rc::new(RefCell::new(Node::new()))
    }

    pub fn route(&self, mut message: Message) {
        if message.onward_route.is_empty() {
            println!("No route for Message: {:#?}", message);
            return;
        }

        let route_entry = message.onward_route.take_front().unwrap();

        self.with_worker(&route_entry.address(), |maybe_worker: Option<&Worker>| {
            if let Some(worker) = maybe_worker {
                if let Err(e) = worker.inbox.borrow_mut().enqueue(message.clone()) {
                    panic!(e)
                };
            } else {
                println!("route: No Worker at Address {}", route_entry.address())
            }
        })
    }

    pub fn register(&mut self, worker: Worker) {
        if let Some(host) = &self.host {
            host.worker_registry_queue.enqueue(worker);
        }
    }

    pub fn with_worker(&self, address: &Address, worker: impl FnMut(Option<&Worker>)) {
        if let Some(host) = &self.host {
            let reg = host.worker_registry.borrow();
            (*reg).with_worker(address, worker);
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node::new()
    }
}

pub type NodeHandle = Rc<RefCell<Node>>;

#[cfg(test)]
mod tests {
    use crate::address::Address;
    use crate::message::Message;
    use crate::node::{Host, Runner};
    use crate::registry::WorkerRegistry;
    use crate::worker::{Callbacks, Worker};

    #[test]
    fn test_node() {
        let mut host = Host::new();
        let node_handle = host.clone().node.unwrap();

        struct TestWorker {}

        impl Callbacks<Message> for TestWorker {
            fn handle(&mut self, message: Message, _worker: &mut Worker) -> crate::Result<bool> {
                println!("test worker message: {:#?}", message);
                Ok(true)
            }

            fn starting(
                &mut self,
                _worker: &mut Worker,
                _worker_registry: &WorkerRegistry,
            ) -> crate::Result<bool> {
                println!("created");

                Ok(true)
            }

            fn stopping(&mut self, _worker: &mut Worker) -> crate::Result<bool> {
                println!("destroyed");
                Ok(true)
            }
        }

        let worker_address = Address::from("worker");

        let worker = match crate::worker::with(node_handle.clone(), TestWorker {})
            .address("worker")
            .build()
        {
            Some(worker) => worker,
            None => panic!(),
        };

        let mut node = node_handle.borrow_mut();

        node.register(worker);

        host.run_for(1);

        node.with_worker(&worker_address, |maybe_worker: Option<&Worker>| {
            if let Some(worker) = maybe_worker {
                println!("Got worker at address {:?}", worker.address());
            } else {
                panic!("No worker at {}", worker_address)
            }
        });
    }
}
