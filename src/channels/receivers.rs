use anyhow::Result;
use crossbeam_channel as crossbeam;

pub trait Receiver<T> {
    fn recv(&self) -> Result<T>;
}

impl<T> Receiver<T> for crossbeam::Receiver<T> {
    fn recv(&self) -> Result<T> {
        let msg = self.recv()?;
        Ok(msg)
    }
}

impl<T, U, RT: Receiver<T>, RU: Receiver<U>> Receiver<(T, U)> for (RT, RU) {
    fn recv(&self) -> Result<(T, U)> {
        Ok((self.0.recv()?, self.1.recv()?))
    }
}

impl Receiver<()> for () {
    fn recv(&self) -> Result<()> {
        Ok(())
    }
}
