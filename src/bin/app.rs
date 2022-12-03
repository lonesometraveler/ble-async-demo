#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use ble_async_demo::{
    self as _,
    ble::{sd, server},
    button_handler,
    device::Board,
    led_handler,
    message::{self, AppEvent, PinState},
    uart_rx_handler,
};
use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::interrupt::Priority;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Async + SoftDevice Demo");

    // Configure peripherals
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    // Initialize board with peripherals
    let board = Board::init(p);

    // Run LED indicator task
    unwrap!(spawner.spawn(led_handler::run(board.led3)));

    // BLE controllable LED
    let mut ble_led = board.led4;

    // Messaging: Create Publishers and Subscribers
    let mut subscriber = unwrap!(message::MESSAGE_BUS.subscriber());
    let publisher_1 = unwrap!(message::MESSAGE_BUS.publisher());
    let publisher_2 = unwrap!(message::MESSAGE_BUS.publisher());

    // Enable SoftDevice
    let sd = nrf_softdevice::Softdevice::enable(&sd::softdevice_config());

    // Create BLE GATT server
    let server = unwrap!(server::Server::new(sd));

    // Run SoftDevice task
    unwrap!(spawner.spawn(sd::softdevice_task(sd)));

    // Run BLE server task
    unwrap!(spawner.spawn(server::ble_server_task(spawner, server, sd)));

    // Run Button task
    unwrap!(spawner.spawn(button_handler::run(board.button1, publisher_1)));

    // UART device: split to tx and rx
    let (mut tx, rx) = board.uart.split();

    // Run UART RX task
    unwrap!(spawner.spawn(uart_rx_handler::run(rx, publisher_2)));

    // Wait for a message...
    loop {
        match subscriber.next_message_pure().await {
            AppEvent::Led(pin_state) => match pin_state {
                PinState::High => ble_led.set_high(),
                PinState::Low => ble_led.set_low(),
            },
            AppEvent::BleBytesWritten(data) => {
                // Send the received data through UART TX
                if let Err(e) = tx.write(&data).await {
                    error!("{:?}", e);
                }
            }
            _ => (),
        }
    }
}
