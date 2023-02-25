pub trait ProcessorComponent {
    type I;
    type O;
    fn process(&self, input: Self::I) -> Self::O;
}

pub trait IocComponent {
    type O;
    fn run(self, event_handler: Box<dyn Fn(Self::O)>) -> !;
}

pub trait StaticIocComponent {
    type O;
    fn run(event_handler: Box<dyn Fn(Self::O)>) -> !;
}
