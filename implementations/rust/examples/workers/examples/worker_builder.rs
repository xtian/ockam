use ockam::message::Message;
use ockam::worker::{Callbacks, Worker};

use ockam::node::Node;
use ockam::registry::WorkerRegistry;
use ockam::Result;

struct BuiltWorker {}

impl Callbacks<Message> for BuiltWorker {
    fn starting(&mut self, worker: &mut Worker, _worker_registry: &WorkerRegistry) -> Result<bool> {
        println!("Started on address {}", worker.address());
        Ok(true)
    }
}

#[ockam::node]
pub async fn main() {
    let node = Node::new_handle();

    let maybe_worker = ockam::worker::with(node.clone(), BuiltWorker {})
        .address("worker123")
        .build();

    match maybe_worker {
        Some(w) => {
            let mut n = node.borrow_mut();
            n.register(w)
        }
        None => panic!("Failed to start"),
    }
}
