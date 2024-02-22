#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use ble_async_demo::{
    self as _,
    ble::{sd, server},
    device::Board,
};
use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::interrupt::Priority;
use embassy_time::{Duration, Timer};
use ens160::{AirQualityIndex, Ens160};

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

    // Enable SoftDevice
    let sd = nrf_softdevice::Softdevice::enable(&sd::softdevice_config());

    // Create BLE GATT server
    let server = unwrap!(server::Server::new(sd));

    // Run SoftDevice task
    unwrap!(spawner.spawn(sd::softdevice_task(sd)));

    // Run BLE server task
    unwrap!(spawner.spawn(server::ble_server_task(spawner, server, sd)));

    let twim = board.twim;
    // Set up TWI slave
    let mut device = Ens160::new(twim, 0x53);

    loop {
        if let Ok(status) = device.status() {
            if status.data_is_ready() {
                let tvoc = device.tvoc().unwrap();
                info!("TVOC: {}", tvoc);

                let eco2 = device.eco2().unwrap();
                let air_quality_index = AirQualityIndex::try_from(eco2).unwrap();
                match air_quality_index {
                    AirQualityIndex::Excellent => info!("Air quality index: Excellent"),
                    AirQualityIndex::Good => info!("Air quality index: Good"),
                    AirQualityIndex::Moderate => info!("Air quality index: Moderate"),
                    AirQualityIndex::Poor => info!("Air quality index: Poor"),
                    AirQualityIndex::Unhealthy => info!("Air quality index: Unhealthy"),
                }
            }
        }

        Timer::after(Duration::from_millis(1_000)).await;
    }
}
