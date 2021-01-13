use ockam::address::Address;
use ockam::message::Message;
use ockam::node::Node;
use ockam::worker::Worker;

#[ockam::node]
pub async fn main() {
    let inbox = ockam::message::new_message_queue(Address::from("worker_inbox"));

    let handler = |message: &Message, worker: &mut Worker| {
        println!("Address: {}, Message: {:#?}", worker.address(), message);
    };

    let node_handle = Node::new_handle();

    if let Some(worker) = ockam::worker::with_closure(node_handle.clone(), handler)
        .inbox(inbox)
        .build()
    {
        let mut node = node_handle.borrow_mut();
        node.register(worker);
        node.route("hello".into())
    }
}
