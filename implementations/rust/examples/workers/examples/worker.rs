use ockam::message::Message;
use ockam::node::Host;
use ockam::registry::WorkerRegistry;
use ockam::task::Poll;
use ockam::worker::{Callbacks, Worker};
use ockam::Result;

struct MyWorker {}

impl Callbacks<Message> for MyWorker {
    fn starting(&mut self, worker: &mut Worker, _worker_registry: &WorkerRegistry) -> Result<bool> {
        println!("Started on address {}", worker.address());
        Ok(true)
    }

    fn stopping(&mut self, _worker: &mut Worker) -> Result<bool> {
        println!("Stopping!");
        Ok(true)
    }
}

#[ockam::node]
pub async fn main() {
    let mut host = Host::new();
    let node = host.clone().node.unwrap();

    if let Some(worker) = ockam::worker::with(node.clone(), MyWorker {}).build() {
        println!("{:?}", worker.address());

        let mut n = node.borrow_mut();
        n.register(worker);
    } else {
        panic!("Couldn't create Worker");
    }
    host.safe_poll();
}
