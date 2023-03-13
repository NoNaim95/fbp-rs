use crossbeam_channel::unbounded;

use fbp_rs::components::Component;
use udp_proxy::packet::Packet;
use udp_proxy::ProxyServer;

use fbp_rs::*;
use process_factory::*;

pub struct IocNetworker {
    proxy: ProxyServer,
}

impl Component for IocNetworker {
    type I = ();
    type O = Packet;
}

impl IocGeneratorComponent for IocNetworker {
    fn run(self, event_handler: Box<dyn Fn(Self::O)>) -> ! {
        self.proxy.run(event_handler);
    }
}

pub struct Logger {}

impl Component for Logger {
    type I = Packet;
    type O = ();
}

impl ProcessorComponent for Logger {
    fn process(&self, input: Self::I) -> Self::O {
        dbg!(input.data);
    }
}

fn main() {
    let (network_pipe_begin, network_pipe_end) = unbounded();
    let (socket, client) = ProxyServer::get_client(4444);
    let network_component = IocNetworker {
        proxy: ProxyServer::new(socket, "103.172.92.234:28763".parse().unwrap(), client),
    };
    let network_process =
        ProcessFactoryImpl::create_from_ioc_generator(network_component, network_pipe_begin);
    let networker_handle = std::thread::spawn(network_process);

    let logger_component = Logger {};
    let logger_process = ProcessFactoryImpl::create_process(logger_component, network_pipe_end, ());
    let logger_handle = std::thread::spawn(logger_process);

    networker_handle.join().unwrap();
    logger_handle.join().unwrap();
}
