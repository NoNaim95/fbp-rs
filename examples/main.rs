use crossbeam_channel::bounded;
use std::time::Duration;

use channels::receivers::*;
use channels::senders::*;
use process_factory::*;

use components::*;

use fbp_rs::*;

pub struct Printer {
    pub date: &'static str,
}

impl ProcessorComponent for Printer {
    type I = (String, i32);
    type O = ();
    fn process(&self, input: Self::I) -> Self::O {
        println!("name: {}, age: {}", input.0, input.1);
    }
}

pub struct StringFactory {}
impl ProcessorComponent for StringFactory {
    type I = ();
    type O = String;

    fn process(&self, _input: Self::I) -> Self::O {
        String::from("Made in String Factory")
    }
}

pub struct AgeFactory {}
impl ProcessorComponent for AgeFactory {
    type I = ();
    type O = i32;

    fn process(&self, _input: Self::I) -> Self::O {
        std::thread::sleep(Duration::from_secs(1));
        5
    }
}

pub struct Networker {}
impl IocComponent for Networker {
    type O = String;

    fn run(&self, mut event_handler: EventHandler<Self::O>) -> ! {
        loop {
            event_handler.call(String::from("NICEIECNIENEIC"));
        }
    }

    fn create_event_handler<T: 'static + Sender<Self::O>>(
        sender: T,
        f: impl Fn(&mut T, Self::O) + 'static,
    ) -> EventHandler<Self::O> {
        todo!()
    }
}

fn main() {
    let (age_pipe_begin, age_pipe_end) = bounded(4);
    let age_process =
        ProcessFactoryImpl::create_process(AgeFactory {}, EmptyReceiver {}, age_pipe_begin);
    let age_handle = std::thread::spawn(age_process);

    let (string_pipe_begin, string_pipe_end) = bounded(4);
    let string_factory_process =
        ProcessFactoryImpl::create_process(StringFactory {}, EmptyReceiver {}, string_pipe_begin);
    let string_factory_handle = std::thread::spawn(string_factory_process);

    let printer_process = ProcessFactoryImpl::create_process(
        Printer { date: "Monday" },
        (string_pipe_end, age_pipe_end),
        EmptySender {},
    );
    std::thread::sleep(Duration::from_secs(5));
    let printer_handle = std::thread::spawn(printer_process);

    printer_handle.join().unwrap();
    string_factory_handle.join().unwrap();
    age_handle.join().unwrap();
}
