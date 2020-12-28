use ockam::worker::{self, Worker};

struct Bob {}

impl Worker for Bob {
    fn starting(&self) {
        println!("starting bob!!");
    }
}

#[ockam::node]
fn main() {
    let address = worker::from(Bob {}).with_address("bob").start();
    println!("{}", address);
}
