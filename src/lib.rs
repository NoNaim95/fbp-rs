use std::sync::mpsc::{Receiver, Sender};

pub trait Component {
    type I;
    type O;
    fn step(&self, input: Self::I) -> Self::O;
}

pub trait Scheduler {
    fn run<T: Component>(component: T, input: Receiver<T::I>, output: Sender<T::O>) -> !;
}

pub struct SchedulerImpl {}

impl Scheduler for SchedulerImpl {
    fn run<T: Component>(component: T, input: Receiver<T::I>, output: Sender<T::O>) -> ! {
        loop {
            output
                .send(
                    component.step(
                        input
                            .recv()
                            .expect("Input Component panicked, panicking too!!!"),
                    ),
                )
                .unwrap();
        }
    }
}
