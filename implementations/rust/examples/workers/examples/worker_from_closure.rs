use ockam::message::{MessageBuilder, OnwardTo};
use ockam::node::Host;
use ockam::task::Poll;

#[ockam::node]
pub async fn main() {
    let mut host = Host::new();
    let node = host.clone().node.unwrap();

    if let Some(worker) = ockam::worker::with_closure(node.clone(), move |message, context| {
        println!("Address: {}\tMessage: {:#?}", context.address(), message)
    })
    .build()
    {
        let address = worker.address();

        {
            let mut n = node.borrow_mut();
            n.register(worker);
        }

        host.safe_poll();

        let n = node.borrow();
        n.route(MessageBuilder::message().empty().onward_to(address).build());
        host.safe_poll();
    }
}
