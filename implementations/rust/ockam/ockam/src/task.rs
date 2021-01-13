use crate::queue::QueueHandle;
use crate::worker::{Callbacks, Worker};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub type TaskHandle<I, O> = Rc<RefCell<dyn Task<I, O>>>;

/// A handler (possibly/probably a Worker?) that takes input and produces output.
pub trait Task<I, O> {
    fn run(&mut self, input: I) -> crate::Result<O>;

    /// Experimental, allows peeking at state if the Task is long running.
    fn state(&self) -> Option<O> {
        None
    }
}

pub trait Poll {
    /// Polling doesn't actually deal with any I/O - that is the realm of Tasks
    fn poll(&mut self) -> crate::Result<()>;

    /// More like ignoring_poll. Should be phased out with proper error handling.
    fn safe_poll(&mut self) {
        if self.poll().is_err() {
            // Ignored
        };
    }
}

/// An input queue and Task to execute for the queue.
#[derive(Clone)]
pub struct TaskQueue<I, O> {
    queue: QueueHandle<I>,
    task: TaskHandle<I, O>,
}

/// Generic delegating functions
impl<I: 'static, O: 'static> TaskQueue<I, O> {
    pub fn new(task: TaskHandle<I, O>) -> Self {
        TaskQueue {
            queue: Rc::new(RefCell::new(VecDeque::new())),
            task,
        }
    }

    pub fn enqueue(&self, element: I) {
        match self.queue.borrow_mut().enqueue(element) {
            Ok(_) => {}
            Err(_) => panic!(),
        };
    }

    fn dequeue(&self) -> Option<I> {
        self.queue.borrow_mut().dequeue()
    }
}

/// Polling a TaskQueue causes it to invoke the `Task` once for each `Message` in the `Queue`. 
impl<I: 'static, O: 'static> Poll for TaskQueue<I, O> {
    fn poll(&mut self) -> crate::Result<()> {
        let task = self.task.clone();
        while let Some(element) = self.dequeue() {
            // TODO check output values from `run` or remove output if unneeded

            match task.borrow_mut().run(element) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }
}

/// Worker `Callbacks` for a `TaskQueue`. Enqueues messages sent to it.
impl<I: 'static, O: 'static> Callbacks<I> for TaskQueue<I, O> {
    fn handle(&mut self, data: I, _context: &mut Worker) -> crate::Result<bool> {
        self.enqueue(data);
        Ok(true)
    }
}

#[test]
fn test_task_queue() {
    struct Adder {
        state: Rc<RefCell<i32>>,
    }

    impl Task<i32, i32> for Adder {
        fn run(&mut self, data: i32) -> crate::Result<i32> {
            *self.state.borrow_mut() += data;
            Ok(*self.state.borrow())
        }
    }

    let state: Rc<RefCell<i32>> = Rc::new(RefCell::new(0 as i32));

    let adder = Adder {
        state: state.clone(),
    };

    let a: TaskHandle<i32, i32> = Rc::new(RefCell::new(adder));

    let mut task_queue = TaskQueue::new(a.clone());

    task_queue.enqueue(1);
    task_queue.enqueue(2);
    task_queue.safe_poll();

    assert_eq!(3, *state.borrow());
}
