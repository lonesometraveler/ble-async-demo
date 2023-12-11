//! Board definition for Microbit v2
use embassy_nrf::{
    bind_interrupts,
    gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull},
    interrupt::{self, InterruptExt},
    peripherals::{TWISPI0, UARTE0},
    twim::{self, Twim},
    uarte::{self, Uarte},
};

bind_interrupts!(struct Irqs {
    UARTE0_UART0 => uarte::InterruptHandler<UARTE0>;
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<TWISPI0>;
});

pub struct Board {
    /// Onboard LED Column 1
    pub led1: Output<'static, AnyPin>,
    /// Onboard LED Column 2
    pub led2: Output<'static, AnyPin>,
    /// Onboard LED Row 1
    pub led3: Output<'static, AnyPin>,
    /// Onboard LED Row 2
    pub led4: Output<'static, AnyPin>,
    /// Onboard Button A
    pub button1: Input<'static, AnyPin>,
    /// Onboard Button B
    pub button2: Input<'static, AnyPin>,
    /// Additional Input P0_10
    pub button3: Input<'static, AnyPin>,
    /// Additional Input P0_17
    pub button4: Input<'static, AnyPin>,
    /// TWI
    pub twim: Twim<'static, TWISPI0>,
    /// UART: Serial - USB
    pub uart: Uarte<'static, UARTE0>,
}

impl Board {
    /// Returns Board with concrete peripherals
    pub fn init(p: embassy_nrf::Peripherals) -> Board {
        let led1 = Output::new(p.P0_28.degrade(), Level::Low, OutputDrive::Standard);
        let led2 = Output::new(p.P0_11.degrade(), Level::Low, OutputDrive::Standard);
        let led3 = Output::new(p.P0_21.degrade(), Level::Low, OutputDrive::Standard);
        let led4 = Output::new(p.P0_22.degrade(), Level::Low, OutputDrive::Standard);

        let button1 = Input::new(p.P0_14.degrade(), Pull::Up);
        let button2 = Input::new(p.P0_23.degrade(), Pull::Up);
        let button3 = Input::new(p.P0_10.degrade(), Pull::Up);
        let button4 = Input::new(p.P0_17.degrade(), Pull::Up);

        // configure twi
        let twim_config = embassy_nrf::twim::Config::default();
        interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0.set_priority(interrupt::Priority::P3);
        let twim = Twim::new(p.TWISPI0, Irqs, p.P0_16, p.P0_08, twim_config);

        // configure uart
        let mut uart_config = uarte::Config::default();
        uart_config.parity = uarte::Parity::EXCLUDED;
        uart_config.baudrate = uarte::Baudrate::BAUD115200;
        interrupt::UARTE0_UART0.set_priority(interrupt::Priority::P3);
        let uart = uarte::Uarte::new(p.UARTE0, Irqs, p.P1_08, p.P0_06, uart_config);

        Board {
            led1,
            led2,
            led3,
            led4,
            button1,
            button2,
            button3,
            button4,
            twim,
            uart,
        }
    }
}
