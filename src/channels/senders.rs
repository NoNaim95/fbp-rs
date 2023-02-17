use anyhow::{anyhow, Result};
use crossbeam_channel as crossbeam;

pub trait Sender<T> {
    fn send(&self, t: T) -> Result<()>;
}

impl<T> Sender<T> for crossbeam::Sender<T> {
    fn send(&self, t: T) -> Result<()> {
        let x = self.send(t).map_err(|_| anyhow!("Receiver is gone"))?;
        Ok(x)
    }
}

impl<T, U, ST: Sender<T>, SU: Sender<U>> Sender<(T, U)> for (ST, SU) {
    fn send(&self, t: (T, U)) -> Result<()> {
        self.0.send(t.0).map_err(|_| anyhow!("Receiver is gone"))?;
        self.1.send(t.1).map_err(|_| anyhow!("Receiver is gone"))?;
        Ok(())
    }
}

pub struct EmptySender {}
impl Sender<()> for EmptySender {
    fn send(&self, _: ()) -> Result<()> {
        Ok(())
    }
}
