use std::{sync::mpsc::channel, time::Duration};

use fbp_rs::*;

pub struct Printer {
    pub date: &'static str,
}

impl Component for Printer {
    type I = String;
    type O = ();
    fn step(&self, input: Self::I) -> Self::O {
        println!("Today is {}, {}", self.date, input);
    }
}

pub struct StringFactory {}

impl Component for StringFactory {
    type I = ();

    type O = String;

    fn step(&self, input: Self::I) -> Self::O {
        String::from("Made in String Factory")
    }
}

fn main() {
    let (string_pipe_input, string_pipe_output) = channel();
    let (printer_output_taker, rx) = channel();

    let handle1 = std::thread::spawn(move || {
        SchedulerImpl::run(Printer { date: "Monday" }, string_pipe_output, printer_output_taker);
    });

    string_pipe_input.send(String::from("This is a message")).unwrap();
    std::thread::sleep(Duration::from_secs(1));
    string_pipe_input.send(String::from("Hallo Lenz")).unwrap();
    std::thread::sleep(Duration::from_secs(1));
    string_pipe_input.send(String::from("Hallo Deni")).unwrap();
    std::thread::sleep(Duration::from_secs(1));
    rx.recv();

    handle1.join().unwrap();
}
