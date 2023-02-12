#![no_std]
#![no_main]
#![feature(core_intrinsics, start)]

use esp_idf_hal::prelude::*;
use esp_idf_hal::{gpio::*, riscv_ulp_hal::sys::*};

extern crate panic_halt;

#[no_mangle]
static mut CYCLES: u32 = 40;

#[no_mangle]
static mut LED_SWITCH: bool = true;

#[no_mangle]

fn main() {
    unsafe {
        gpio_set_direction(11, gpio_mode_t_GPIO_MODE_OUTPUT_OD);
    }
    if toogle_led_switch() {
        unsafe {
            gpio_set_level(11, 1);
        }
    } else {
        unsafe {
            gpio_set_level(11, 0);
        }
    }

    if get_cycles() == 1 {
        set_cycles(20);
        wakeup_main_processor();
    } else {
        decr_cycles();
    }
}
/*
fn main() {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut led = PinDriver::output_od(pins.gpio11).unwrap();

    if toogle_led_switch() {
        led.set_high().unwrap();
    } else {
        led.set_low().unwrap();
    }

    if get_cycles() == 1 {
        set_cycles(20);
        wakeup_main_processor();
    } else {
        decr_cycles();
    }
}
*/
fn toogle_led_switch() -> bool {
    unsafe {
        let led_switch = core::ptr::read_volatile(&LED_SWITCH);
        core::ptr::write_volatile(&mut LED_SWITCH, !led_switch);
        led_switch
    }
}

fn get_cycles() -> u32 {
    unsafe { core::ptr::read_volatile(&CYCLES) }
}

fn set_cycles(new_cycles: u32) {
    unsafe {
        core::ptr::write_volatile(&mut CYCLES, new_cycles);
    }
}

fn decr_cycles() {
    unsafe {
        let cycles = core::ptr::read_volatile(&CYCLES);

        if cycles > 0 {
            core::ptr::write_volatile(&mut CYCLES, cycles - 1);
        }
    }
}
