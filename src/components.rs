use crate::channels::{receivers::*, senders::*};

pub trait ProcessorComponent {
    type I;
    type O;
    fn process(&self, input: Self::I) -> Self::O;
}

pub trait IocComponent {
    type I;
    type O;
    fn run(&self, input_channel: impl Receiver<Self::I>, output_channel: impl Sender<Self::O>);
}
