use crossbeam_channel::bounded;
use std::time::Duration;

use channels::receivers::*;
use channels::senders::*;
use process_factory::*;

use fbp_rs::*;

struct NetworkingComponent {}

impl NetworkingComponent {
    pub fn run(f: impl Fn(Vec<u8>)) {
        f(vec![1, 2, 3]);
        send(r.recv());
    }
}

fn main() {}
