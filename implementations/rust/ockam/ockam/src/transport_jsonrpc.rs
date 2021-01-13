/// Example implementation of the prototype Transport, using jsonrpc crates.
use crate::address::{Address, Addressable};
use crate::message::Message;
use crate::worker::{Callbacks, Worker};

use serde_json::Value;
use std::sync::{Arc, Mutex};

use crate::registry::WorkerRegistry;
use jsonrpc_client_http::{HttpHandle, HttpTransport};
use jsonrpc_http_server::jsonrpc_core::{futures, IoHandler, Params};
use jsonrpc_http_server::*;

jsonrpc_client!(pub struct JsonClient {
    pub fn send_message(&mut self, message: Message) -> RpcRequest<String>;
});

#[derive(Clone)]
pub struct JsonConnector {
    local_address: Address,
    remote_address: Address,
    transport_handle: Option<HttpHandle>,
}

impl Addressable for JsonConnector {
    fn address(&self) -> Address {
        self.local_address.clone()
    }
}

impl JsonConnector {
    pub fn new(local_address: Address, remote_address: Address) -> Self {
        JsonConnector {
            local_address,
            remote_address,
            transport_handle: None,
        }
    }
}

impl Callbacks<Message> for JsonConnector {
    fn handle(&mut self, message: Message, _context: &mut Worker) -> crate::Result<bool> {
        if let Some(transport_handle) = &self.transport_handle {
            let mut client = JsonClient::new(transport_handle.clone());
            let res = client.send_message(message).call();
            match res {
                Ok(response) => println!("Success: {}", response),
                Err(e) => println!("Error: {}", e),
            };
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn starting(
        &mut self,
        _context: &mut Worker,
        _worker_registry: &WorkerRegistry,
    ) -> crate::Result<bool> {
        let transport = HttpTransport::new().standalone().unwrap();
        let remote = format!("http://{}", self.remote_address.to_string());

        println!("JsonConnector: Connecting to {}", remote);

        match transport.handle(remote.as_str()) {
            Ok(transport_handle) => {
                self.transport_handle = Some(transport_handle);
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }
}

#[derive(Clone)]
pub struct JsonListener {}

impl Callbacks<Message> for JsonListener {
    fn handle(&mut self, _message: Message, _context: &mut Worker) -> crate::Result<bool> {
        println!("{:#?}", _message);
        Ok(true)
    }

    fn starting(
        &mut self,
        worker: &mut Worker,
        _worker_registry: &WorkerRegistry,
    ) -> crate::Result<bool> {
        let mut listener = self.clone();
        let addr = worker.address();
        let _handle = Arc::new(Mutex::new(Some(std::thread::spawn(move || {
            listener.listen(addr);
        }))));
        Ok(true)
    }

    fn stopping(&mut self, _context: &mut Worker) -> crate::Result<bool> {
        Ok(true)
    }
}

impl JsonListener {
    fn listen(&mut self, address: Address) {
        let mut io = IoHandler::new();

        io.add_method("send_message", |p: Params| {
            println!("JsonListener got message: {:#?}", p);
            futures::future::ok(Value::String("OK".into()))
        });

        let address_string: String = address.into();
        let server = ServerBuilder::new(io)
            .start_http(&address_string.parse().unwrap())
            .expect("Unable to start RPC server");

        println!("Listening for JSON");
        server.wait();
    }
}
