use crossbeam_channel::bounded;
use crossbeam_channel::unbounded;
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
    let (s, r) = unbounded();

    let ioc_comp = Networker{};

    ioc_comp.run(EventHandler::new())

    std::thread::sleep(Duration::from_secs(5));
    let printer_handle = std::thread::spawn(||());

    printer_handle.join().unwrap();
}
