use std::sync::mpsc;
use std::thread;

use crate::config::{Config, Input};
use crate::node::Node;

use ockam_common::commands::ockam_commands::*;
use ockam_message::message::{
    AddressType, Message as OckamMessage, MessageType, Route, RouterAddress,
};

pub fn run(mut config: Config) {
    // configure a node
    let node_config = config.clone();
    let (node, router_tx) = Node::new(&node_config);

    // read from stdin, pass each line to the router within the node
    thread::spawn(move || {
        let input = std::io::stdin();
        let mut buf = String::new();

        // create a stub worker to handle the kex messages
        let kex_worker_addr = RouterAddress::worker_router_address_from_str("01242020")
            .expect("failed to create worker address for kex");
        let (kex_worker_tx, kex_worker_rx) = mpsc::channel();
        router_tx
            .send(OckamCommand::Router(RouterCommand::Register(
                AddressType::Worker,
                kex_worker_tx,
            )))
            .expect("failed to register kex worker with router");

        // create the secure channel
        let mut onward_route = config.onward_route().expect("misconfigured onward route");
        onward_route.addresses.insert(
            0,
            RouterAddress::channel_router_address_from_str("00000000")
                .expect("failed to create zero channel address"),
        );
        let return_route = Route {
            addresses: vec![kex_worker_addr],
        };
        let kex_msg = OckamMessage {
            message_type: MessageType::None,
            message_body: vec![],
            onward_route,
            return_route,
        };
        router_tx
            .send(OckamCommand::Router(RouterCommand::SendMessage(kex_msg)))
            .expect("failed to send kex request message to router");

        // await key exchange finalization
        match kex_worker_rx.recv() {
            Ok(cmd) => match cmd {
                OckamCommand::Worker(WorkerCommand::ReceiveMessage(msg)) => {
                    match msg.message_type {
                        MessageType::None => {
                            onward_route = msg.return_route;
                        }
                        _ => unimplemented!(),
                    }
                }
                _ => unimplemented!(),
            },
            Err(e) => panic!("failed to handle kex response: {:?}", e),
        }

        if matches!(config.input_kind(), Input::Stdin) {
            let mps_router_tx = router_tx.clone();
            let mps_onward_route = onward_route.clone();
            thread::spawn(move || loop {
                mps_router_tx
                    .send(OckamCommand::Router(RouterCommand::SendMessage(
                        OckamMessage {
                            onward_route: mps_onward_route.clone(),
                            return_route: Route { addresses: vec![] },
                            message_body: b"Hello from mps thread...\n".to_vec(),
                            message_type: MessageType::Payload,
                        },
                    )))
                    .expect("failed to send input data to node");
            });
            loop {
                if input.read_line(&mut buf).is_ok() {
                    router_tx
                        .send(OckamCommand::Router(RouterCommand::SendMessage(
                            OckamMessage {
                                onward_route: onward_route.clone(),
                                return_route: Route { addresses: vec![] },
                                message_body: buf.as_bytes().to_vec(),
                                message_type: MessageType::Payload,
                            },
                        )))
                        .expect("failed to send input data to node");
                    buf.clear();
                } else {
                    eprintln!("fatal error: failed to read from input");
                    std::process::exit(1);
                }
            }
        }
    });

    // run the node to poll its various internal components
    node.run();
}