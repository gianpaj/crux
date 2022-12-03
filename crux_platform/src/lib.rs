//! TODO mod docs

use crux_core::{channels::Sender, Capability, Command};
use serde::{Deserialize, Serialize};

// TODO revisit this
#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformResponse(pub String);

pub struct Platform<Ev> {
    sender: Sender<Command<(), Ev>>,
}

impl<Ev> Platform<Ev>
where
    Ev: 'static,
{
    pub fn new(sender: Sender<Command<(), Ev>>) -> Self {
        Self { sender }
    }

    pub fn get<F>(&self, callback: F)
    where
        F: Fn(PlatformResponse) -> Ev + Send + Sync + 'static,
    {
        self.sender.send(Command::new((), callback))
    }
}

impl<Ef> Capability<Ef> for Platform<Ef> {
    type MappedSelf<MappedEv> = Platform<MappedEv>;

    fn map_event<F, NewEvent>(&self, f: F) -> Self::MappedSelf<NewEvent>
    where
        F: Fn(NewEvent) -> Ef + Send + Sync + Copy + 'static,
        Ef: 'static,
        NewEvent: 'static,
    {
        Platform::new(self.sender.map_event(f))
    }
}
