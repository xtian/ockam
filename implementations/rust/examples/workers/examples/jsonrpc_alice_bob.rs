/// This is a WIP. There is a panic in the outbox handling that I'm working on.
use ockam::address::Address;
use ockam::message::{Message, MessageBuilder, OnwardTo};
use ockam::node::{Host, NodeHandle, Runner};
use ockam::registry::WorkerRegistry;
use ockam::transport_jsonrpc::{JsonConnector, JsonListener};
use ockam::worker::{Callbacks, Worker};
use ockam::Result;
use std::time::{Duration, Instant};

struct PingPongWorker {
    name: String,
    last_ping: Option<Instant>,
}

impl PingPongWorker {
    fn to_hub_address(&self) -> Address {
        Address::from(format!("{}_to_hub", self.name))
    }

    fn start_game(&self, message: Message, worker: &mut Worker) {
        let p = String::from_utf8(message.payload).unwrap();
        let reply = format!("{} PING {}", self.name, p);

        let message = MessageBuilder::message()
            .payload(reply.into_bytes())
            .onward_to(self.to_hub_address())
            .onward_to(HUB_JSON_LISTENER)
            .build();

        // Panics in delivery runtime, WIP
        worker.outbox.borrow_mut().enqueue(message).unwrap();
        println!("{} start", self.name);
    }

    fn play(&mut self, message: Message, worker: &mut Worker) {
        if self.last_ping.is_none() {
            self.start_game(message, worker);
        }

        self.last_ping = Some(Instant::now());
    }
}

impl Callbacks<Message> for PingPongWorker {
    fn handle(&mut self, message: Message, worker: &mut Worker) -> Result<bool> {
        println!("PING PONG: {:#?}", message);
        self.play(message, worker);
        Ok(true)
    }

    fn starting(
        &mut self,
        worker: &mut Worker,
        worker_registry: &WorkerRegistry,
    ) -> ockam::Result<bool> {
        println!("PingPong worker {} started", worker.address());

        let hub = Address::from(HUB_JSON_LISTENER);
        let to_hub = Address::from(format!("{}_to_hub", worker.address()));
        let connector = JsonConnector::new(to_hub.clone(), hub);

        worker_registry.register(
            ockam::worker::with(worker.node.clone(), connector)
                .address(to_hub.to_string().as_str())
                .build()
                .unwrap(),
        );
        Ok(true)
    }

    fn stopping(&mut self, worker: &mut Worker) -> ockam::Result<bool> {
        println!("PingPong worker {} stopping", worker.address());
        Ok(true)
    }
}

const HUB_JSON_LISTENER: &str = "127.0.0.1:9000";

fn create_hub(node: NodeHandle) -> Worker {
    ockam::worker::with(node, JsonListener {})
        .address(HUB_JSON_LISTENER)
        .build()
        .unwrap()
}

fn create_ping_pong(address: &str, node: NodeHandle) -> Worker {
    ockam::worker::with(
        node,
        PingPongWorker {
            name: String::from(address),
            last_ping: None,
        },
    )
    .address(address)
    .build()
    .unwrap()
}

fn main() {
    let mut host = Host::new();
    let node_handle = host.clone().node.unwrap();
    {
        let node_clone = node_handle.clone();
        let mut node = node_clone.borrow_mut();

        let hub = create_hub(node_handle.clone());

        node.register(hub);

        std::thread::sleep(Duration::from_millis(100));

        let alice = create_ping_pong("alice", node_handle.clone());

        node.register(alice);

        let bob = create_ping_pong("bob", node_handle.clone());

        node.register(bob);
    }

    // Process all worker registrations.
    host.run_for(1);

    {
        let start = MessageBuilder::message()
            .payload("start".into())
            .onward_to("alice")
            .build();

        let node = node_handle.borrow_mut();
        node.route(start);
    }

    // TODO same as above: async + timer
    host.run_forever();
}
