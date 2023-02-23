/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

pub(crate) mod constants {}

pub(crate) mod types {
    use tokio::sync::{broadcast, mpsc, oneshot, watch};

    /// [AsyncMutex] is a type alias for a [tokio::sync::Mutex]
    pub type AsyncMutex<T = ()> = tokio::sync::Mutex<T>;
    /// [BroadcastChannels] is a two-tuple consisting of ([broadcast::Sender], [broadcast::Receiver])
    pub type BroadcastChannels<T = ()> = (broadcast::Sender<T>, broadcast::Receiver<T>);
    /// [OneshotChannels] is a two-tuple consisting of ([oneshot::Sender], [oneshot::Receiver])
    pub type OneshotChannels<T = ()> = (oneshot::Sender<T>, oneshot::Receiver<T>);
    /// [UnboundedMPSC] is a two-tuple consisting of ([mpsc::UnboundedSender], [mpsc::UnboundedReceiver])
    pub type UnboundedMPSC<T = ()> = (mpsc::UnboundedSender<T>, mpsc::UnboundedReceiver<T>);
    /// [WatchChannels] is a two-tuple consisting of ([watch::Sender], [watch::Receiver])
    pub type WatchChannels<T = ()> = (watch::Sender<T>, watch::Receiver<T>);
}
