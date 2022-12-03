//! Uart RX task
use crate::message::{AppEvent, AppPublisher};
use embassy_nrf::{peripherals::UARTE0, uarte::UarteRx};

// Notify on UART RX event.
#[embassy_executor::task]
pub async fn run(mut rx: UarteRx<'static, UARTE0>, sender: AppPublisher) {
    let mut tx_buf = heapless::Vec::new();
    loop {
        let mut rx_buf = [0u8; 1];
        if (rx.read(&mut rx_buf).await).is_ok() {
            let val = rx_buf[0];
            tx_buf.push(val).unwrap();
            if val == 0x0D || tx_buf.is_full() {
                sender
                    .publish(AppEvent::UartRxWritten(tx_buf.clone()))
                    .await;
                tx_buf.clear();
            }
        }
    }
}
