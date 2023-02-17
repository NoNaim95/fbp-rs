use crate::channels::{receivers::*, senders::*};
use crate::Component;

pub trait Scheduler {
    fn create_process<T, I, O>(component: T, input: I, output: O) -> Box<dyn FnOnce() + Send>
    where
        T: Component + 'static + Send,
        I: Receiver<T::I> + 'static + Send,
        O: Sender<T::O> + 'static + Send;
}

pub struct SchedulerImpl {}

impl Scheduler for SchedulerImpl {
    fn create_process<T, I, O>(component: T, input: I, output: O) -> Box<dyn FnOnce() + Send>
    where
        T: Component + 'static + Send,
        I: Receiver<T::I> + 'static + Send,
        O: Sender<T::O> + 'static + Send,
    {
        Box::new(move || loop {
            let arg = input
                .recv()
                .expect("Input Component panicked, panicking too!!!");
            let o = component.process(arg);
            output.send(o).unwrap();
        })
    }
}
