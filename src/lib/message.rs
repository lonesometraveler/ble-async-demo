//! Types and functions to pass data between threads.
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    pubsub::{PubSubChannel, Publisher, Subscriber},
};

/// MESSAGE_BUS queue size
pub const QUEUE_SIZE: usize = 3;
/// Max number of subsribers in MESSAGE_BUS
pub const SUBS_SIZE: usize = 5;
/// Max number of publishers in MESSAGE_BUS
pub const PUBS_SIZE: usize = 5;

/// Create the message bus. It has a queue of QUEUE_SIZE, supports SUBS_SIZE subscribers and PUBS_SIZE publisher
pub static MESSAGE_BUS: PubSubChannel<
    ThreadModeRawMutex,
    AppEvent,
    QUEUE_SIZE,
    SUBS_SIZE,
    PUBS_SIZE,
> = PubSubChannel::new();

/// Global Subscriber
pub type AppSubscriber =
    Subscriber<'static, ThreadModeRawMutex, AppEvent, QUEUE_SIZE, SUBS_SIZE, PUBS_SIZE>;
/// Global Publisher
pub type AppPublisher =
    Publisher<'static, ThreadModeRawMutex, AppEvent, QUEUE_SIZE, SUBS_SIZE, PUBS_SIZE>;

/// Event types and associated values used in this app
#[derive(Clone, Debug, defmt::Format)]
pub enum AppEvent {
    Led(PinState),
    Button(PinState),
    BleBytesWritten(heapless::Vec<u8, 32>),
    UartRxWritten(heapless::Vec<u8, 32>),
}

/// GPIO pin state
#[derive(Clone, Debug, PartialEq, defmt::Format)]
pub enum PinState {
    Low,
    High,
}

impl From<PinState> for bool {
    fn from(val: PinState) -> Self {
        match val {
            PinState::High => true,
            PinState::Low => false,
        }
    }
}

impl From<bool> for PinState {
    fn from(value: bool) -> Self {
        if value {
            PinState::High
        } else {
            PinState::Low
        }
    }
}
