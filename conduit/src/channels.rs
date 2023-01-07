/*
    Appellation: channels <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{Context, State};
use scsys::prelude::Locked;
use tokio::sync::{mpsc, oneshot};

pub type OneshotChannels<T> = (oneshot::Sender<T>, oneshot::Receiver<T>);

pub type UnboundedMPSC<T> = (mpsc::UnboundedSender<T>, mpsc::UnboundedReceiver<T>);

#[derive(Debug)]
pub struct AppChannels {
    pub context: OneshotChannels<Context>,
    pub state: UnboundedMPSC<Locked<State>>,
}

impl AppChannels {
    pub fn new() -> Self {
        let context = oneshot::channel();
        let state = mpsc::unbounded_channel();
        Self { context, state }
    }
    pub fn state_channels(&self) -> &UnboundedMPSC<Locked<State>> {
        &self.state
    }
}

impl Default for AppChannels {
    fn default() -> Self {
        Self::new()
    }
}