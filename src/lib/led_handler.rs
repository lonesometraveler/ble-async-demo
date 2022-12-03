//! LED indicator task
use embassy_nrf::gpio::{AnyPin, Output};
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn run(mut led: Output<'static, AnyPin>) {
    loop {
        Timer::after(Duration::from_millis(100)).await;
        led.set_high();
        Timer::after(Duration::from_millis(1_000)).await;
        led.set_low();
    }
}
