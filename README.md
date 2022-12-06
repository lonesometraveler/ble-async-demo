# 🦀 async BLE Demo

This is a BLE peripheral example app built with [Embassy](https://github.com/embassy-rs/embassy) and SoftDevice.

### Features:

* BLE services
	* LED service: LED on/off control
	* Button service: Button state notification
	* UART service: BLE <-> UART interface
* Concurrent connections
* Messaging between async tasks

## Requirements

### probe-rs

Install probe-run

```
cargo install probe-run
```

### Board

This app works with these boards:

* [nRF52DK](https://www.nordicsemi.com/Products/Development-hardware/nrf52-dk)
* [micro:bit v2](https://microbit.org/new-microbit/)
* [nRF52840](https://www.nordicsemi.com/Products/Development-hardware/nrf52840-dk)


### SoftDevice

SoftDevices are pre-compiled BLE stack libraries from Nordic. This app requires SoftDevice s132 for nRF52DK and s140 for micro:bit v2 and nRF52840DK.

Download SoftDevice from Nordic.

* [s132](https://www.nordicsemi.com/Products/Development-software/s132/download)
* [s140](https://www.nordicsemi.com/Products/Development-software/s140/download)

### nrf-softdevice

The app depends on [nrf-softdevice](https://github.com/embassy-rs/nrf-softdevice), Rust bindings for SoftDevices. 



## How to run the app

### 1. Install SoftDevice

SoftDevice needs to be flashed first. You only need to do this once. (If you do a full erase, flash it again.)

#### 1.1 Erase the chip

* nRF52DK: `probe-rs-cli erase --chip nRF52832_xxAA`

* micro:bit v2: `probe-rs-cli erase --chip nRF52833_xxAA`

* nRF52840DK: `probe-rs-cli erase --chip nRF52840_xxAA`

#### 1.2 Flash softdevice

* nRF52DK: `probe-rs-cli download --chip nRF52832_xxAA --format hex s132_nrf52_7.3.0_softdevice.hex`

* micro:bit v2: `probe-rs-cli download --chip nRF52833_xxAA --format hex s140_nrf52_7.3.0_softdevice.hex`

* nRF52840DK: `probe-rs-cli download --chip nRF52840_xxAA --format hex s140_nrf52_7.3.0_softdevice.hex`

### 2. Adjust config.toml

Choose a runner for your board in `.cargo/config.toml`

```
# runner = "probe-run --chip nRF52832_xxAA"
# runner = "probe-run --chip nRF52833_xxAA"
runner = "probe-run --chip nRF52840_xxAA"
```

### 3. Define the memory region information

SoftDevice takes up flash and RAM. Adjust the origins accordingly in `memory.x` file.

* Definition for nRF52DK with s132 Softdevice

```
MEMORY
{
  /* s132 */
  FLASH (rx): ORIGIN = 0x26000,    LENGTH = 0x80000 - 0x26000
  RAM (rwx):  ORIGIN = 0x2000DA40, LENGTH = 0x20010000 - 0x2000DA40
}

```

* Definition for micro:bit v2 and nRF52840DK with s140 Softdevice

```
MEMORY
{
  /* s140 */
  FLASH : ORIGIN = 0x00027000, LENGTH = 0x100000 - 0x27000
  RAM : ORIGIN = 0x2000DA50, LENGTH = 0x20010000 - 0x2000DA50
}
```

### 4. Select a feature

Specify `default` feature for your target board in `Cargo.toml` file. Here is an example setting for nRF52840DK.

```
[features]
default = ["nrf52840dk"]
nrf52dk = ["embassy-nrf/nrf52832", "nrf-softdevice/nrf52832", "nrf-softdevice/s132"]
nrf52840dk = ["embassy-nrf/nrf52840", "nrf-softdevice/nrf52840", "nrf-softdevice/s140", "embassy-time/tick-hz-32_768"]
microbit-v2 = ["embassy-nrf/nrf52833", "nrf-softdevice/nrf52833", "nrf-softdevice/s140", "embassy-time/tick-hz-32_768"]
```

### 5. Run

`cargo run --bin app`

The device advertises its BLE name as "HelloRust". Use a BLE app (LightBlue, nRF Connect, etc.) and connect to the device. 

Here are the services you can interact with:

* LED service: Write 0/1 or false/true to control the output pin connected to a LED.
* Button service: Read the state of the input pin. Or subscribe to the characteristic notification.
* UART service: UART goes through the USB serial port. Open the port with your serial monitor (baud rate = 115200). The data written to the UART service get relayed to the monitor. The chars written to the monitor are sent to the UART service when 0x0D (carriage return) is written or the RX buffer gets full.

### 6. Experiment

Modify the code and play around with it. If you only change the Rust source codes, you don't need to reflash SoftDevice.

If you change SoftDevice settings (like increasing the number of concurrent connections), you may need to update RAM info in `memory.x`. If you modify `memory.x`, make sure you run `cargo clean` before `cargo run` as `cargo build/run` doesn't track changes in `memory.x`.
