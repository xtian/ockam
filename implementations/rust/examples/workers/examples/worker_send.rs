use ockam::message::{Message, MessageBuilder, OnwardTo};
use ockam::node::Host;
use ockam::task::Poll;
use ockam::worker::{Callbacks, Worker};
use ockam::Result;

struct PrintWorker {}

impl Callbacks<Message> for PrintWorker {
    fn handle(&mut self, message: Message, _worker: &mut Worker) -> Result<bool> {
        println!("{:#?}", message);
        Ok(true)
    }
}

#[ockam::node]
async fn main() {
    let mut host = Host::new();
    let node = host.clone().node.unwrap();

    if let Some(worker) = ockam::worker::with(node.clone(), PrintWorker {})
        .address("printer")
        .build()
    {
        let address = worker.address();

        println!("Address: {}", address);

        {
            let mut n = node.borrow_mut();
            n.register(worker);
        }

        host.safe_poll();

        node.borrow().route(
            MessageBuilder::message()
                .payload("hello".as_bytes().to_vec())
                .onward_to(address)
                .build(),
        );

        host.safe_poll();
    }
}
