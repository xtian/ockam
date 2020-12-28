use ockam::worker::{self, Worker};

struct Bob {}
impl Worker for Bob {}

#[ockam::node]
fn main() {
    let address = worker::from(Bob {}).start();
    println!("{}", address);
}
