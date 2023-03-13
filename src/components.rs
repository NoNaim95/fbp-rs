pub trait Component {
    type I;
    type O;
}

pub trait ProcessorComponent: Component {
    fn process(&self, input: Self::I) -> Self::O;
}

pub trait IocGeneratorComponent: Component<I = ()> {
    fn run(self, event_handler: Box<dyn Fn(Self::O)>) -> !;
}
