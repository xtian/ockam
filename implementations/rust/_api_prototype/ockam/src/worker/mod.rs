mod builder;

pub use builder::from;

pub trait Worker {
    fn starting(&self) {}
}
