use crate::channels::{receivers::*, senders::*};
use crate::Component;

pub trait ProcessFactory {
    fn create_process<T, I, O>(component: T, input_channel: I, output_channel: O) -> Box<dyn FnOnce() + Send>
    where
        T: Component + 'static + Send,
        I: Receiver<T::I> + 'static + Send,
        O: Sender<T::O> + 'static + Send;
}

pub struct ProcessFactoryImpl {}

impl ProcessFactory for ProcessFactoryImpl {
    fn create_process<T, I, O>(component: T, input_channel: I, output_channel: O) -> Box<dyn FnOnce() + Send>
    where
        T: Component + 'static + Send,
        I: Receiver<T::I> + 'static + Send,
        O: Sender<T::O> + 'static + Send,
    {
        Box::new(move || loop {
            let arg = input_channel
                .recv()
                .expect("Input Component panicked, panicking too!!!");
            let o = component.process(arg);
            output_channel.send(o).unwrap();
        })
    }
}

