#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp32c3_hal::{clock::ClockControl, embassy, peripherals::Peripherals, prelude::*};
use esp_backtrace as _;
use esp_println::print;

#[main]
async fn main(spawner: Spawner) {
    // println!("Hello, world!");
}
