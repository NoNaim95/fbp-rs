use crate::channels::{receivers::*, senders::*};
use crate::ProcessorComponent;

use crate::components::IocComponent;

fn create_callback<T>(sender: impl Sender<T> + 'static) -> Box<dyn Fn(T)> {
    Box::new(move |event: T| {
        sender.send(event).unwrap();
    })
}

pub trait ProcessFactory {
    fn create_process<T, I, O>(
        component: T,
        input_channel: I,
        output_channel: O,
    ) -> Box<dyn FnOnce() + Send>
    where
        T: ProcessorComponent + 'static + Send,
        I: Receiver<T::I> + 'static + Send,
        O: Sender<T::O> + 'static + Send;

    fn create_from_ioc<T, O>(component: T, output_channel: O) -> Box<dyn FnOnce() + Send>
    where
        T: IocComponent + 'static + Send,
        O: Sender<T::O> + 'static + Send;
}
pub struct ProcessFactoryImpl {}

impl ProcessFactory for ProcessFactoryImpl {
    fn create_process<T, I, O>(
        component: T,
        input_channel: I,
        output_channel: O,
    ) -> Box<dyn FnOnce() + Send>
    where
        T: ProcessorComponent + 'static + Send,
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

    fn create_from_ioc<T, O>(component: T, output_channel: O) -> Box<dyn FnOnce() + Send>
    where
        T: IocComponent + 'static + Send,
        O: Sender<T::O> + 'static + Send,
    {
        Box::new(move || {
            component.run(create_callback(output_channel));
        })
    }
}
