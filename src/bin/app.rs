#![no_std]
#![no_main]

use core::fmt::Write;
use esp32_hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    uart::{
        config::{Config, DataBits, Parity, StopBits},
        TxRxPins,
    },
    Delay, Rtc, Uart,
};
use esp_backtrace as _;
use esp_println::println;
use heapless::String;

use nb::block;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);

    // init Watchdog and RTC
    let mut wdt = timer_group0.wdt;
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    rtc.rwdt.disable();
    wdt.start(30u64.secs());

    // delay
    let mut delay = Delay::new(&clocks);
    //
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let config = Config {
        baudrate: 9600,
        data_bits: DataBits::DataBits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::STOP1,
    };

    let pins = TxRxPins::new_tx_rx(
        io.pins.gpio17.into_push_pull_output(),
        io.pins.gpio16.into_floating_input(),
    );

    let mut serial1 = Uart::new_with_config(
        peripherals.UART1,
        Some(config),
        Some(pins),
        &clocks
    );

    // onboard LED
    let mut led = io.pins.gpio2.into_push_pull_output();

    let mut hello_msg: String<20> = String::new();
    write!(hello_msg, "Hello World!").unwrap();

    loop {
        wdt.feed();
        led.set_high().unwrap();
        println!("Hello");

        serial1.write_str("Hallo\n").unwrap();
        delay.delay_ms(5000u32);

        // serial1.write(0x42).ok();

        // Wait 5 seconds
        led.set_low().unwrap();
        delay.delay_ms(5000u32);
    }
}
