//! Button tasks
use crate::message::{
    AppEvent::Button,
    AppPublisher,
    PinState::{High, Low},
};
use embassy_nrf::gpio::{AnyPin, Input};

// Notify BLE task if button state changes
#[embassy_executor::task]
pub async fn run(mut button: Input<'static, AnyPin>, sender: AppPublisher) {
    loop {
        button.wait_for_low().await;
        sender.publish(Button(Low)).await;
        button.wait_for_high().await;
        sender.publish(Button(High)).await;
    }
}
