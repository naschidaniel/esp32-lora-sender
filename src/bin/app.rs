#![no_std]
#![no_main]

extern crate sx127x_lora;
use core::fmt::Write;
use embedded_graphics::{
    mono_font::{ascii::FONT_9X15, ascii::FONT_9X15_BOLD, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Text}
};
use esp32_hal::{
    clock::ClockControl,
    gpio::IO,
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Delay, Rtc,
};
use esp_backtrace as _;
use esp_println::println;
use heapless::String;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

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
    wdt.start(10u64.secs());

    // delay
    let mut delay = Delay::new(&clocks);

    // Embedded Graphics
    let style: MonoTextStyle<BinaryColor> = MonoTextStyle::new(&FONT_9X15, BinaryColor::On);
    let text_style_big = MonoTextStyle::new(&FONT_9X15_BOLD, BinaryColor::On);
    //
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // I2C Sensor Settings
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        10u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );

    // onboard LED
    let mut led = io.pins.gpio2.into_push_pull_output();

    // Display interface
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let mut hello_msg: String<20> = String::new();

    loop {
        wdt.feed();
        led.set_high().unwrap();

        hello_msg.clear();
        write!(hello_msg, "Hello World!").unwrap();
        println!("Hello{}", hello_msg);
        Text::with_alignment(
            hello_msg.as_str(),
            display.bounding_box().center() + Point::new(0, -20),
            text_style_big,
            Alignment::Center,
        )
        .draw(&mut display)
        .unwrap();

        // Write buffer to display
        display.flush().unwrap();
        // Clear display buffer
        display.clear();

        led.set_low().unwrap();
        // Wait 5 seconds
        delay.delay_ms(5000u32);
    }
}
