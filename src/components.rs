//use crate::channels::ip::Ip;

pub trait Component {
    type I;
    type O;
    fn process(&self, input: Self::I) -> Self::O;
}

pub trait SourceComponent {}
pub trait SinkComponent {}
