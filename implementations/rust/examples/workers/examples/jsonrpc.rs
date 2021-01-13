use ockam::address::Address;
use ockam::message::{MessageBuilder, OnwardTo};
use ockam::node::Host;
use ockam::task::Poll;
use ockam::transport_jsonrpc::JsonListener;
use std::time::Duration;

fn send_messages(host: &mut Host) {
    let local_worker_address = "local_worker";
    let transport_address = "127.0.0.1:9000";
    let remote_worker_address = "remote_worker";

    let message = MessageBuilder::message()
        .payload(vec![1, 2, 3])
        .onward_to(transport_address)
        .onward_to(remote_worker_address)
        .return_to(transport_address)
        .return_to(local_worker_address)
        .build();

    let connector = ockam::transport_jsonrpc::JsonConnector::new(
        Address::from(local_worker_address),
        Address::from(transport_address),
    );

    let node = host.clone().node.unwrap();

    let maybe_worker = ockam::worker::with(node.clone(), connector)
        .address(local_worker_address)
        .build();

    if let Some(worker) = maybe_worker {
        {
            let mut n = node.borrow_mut();
            n.register(worker);
        }

        host.safe_poll();

        node.borrow().route(message);

        host.safe_poll();
    }
}

#[ockam::node]
async fn main() {
    let mut host = Host::new();
    let node = host.clone().node.unwrap();

    let maybe_listener = ockam::worker::with(node.clone(), JsonListener {})
        .address("127.0.0.1:9000")
        .build();

    if let Some(listener) = maybe_listener {
        {
            let mut n = node.borrow_mut();
            n.register(listener);
        }
        host.safe_poll();
    }

    std::thread::sleep(Duration::from_millis(100));

    send_messages(&mut host);

    std::thread::sleep(Duration::from_millis(1000));
}
