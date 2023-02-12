use esp_idf_hal::prelude::*;
//use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys::esp;
use esp_idf_sys::{self, EspError};
use log::*;
//use std::mem::MaybeUninit;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::reset::WakeupReason;
use std::time::Duration;

include!(env!("EMBUILD_GENERATED_SYMBOLS_FILE"));
const ULP: &[u8] = include_bytes!(env!("EMBUILD_GENERATED_BIN_FILE"));

// This is a temporary solution as the enums are not defined
const ESP_PD_OPTION_ON: u32 = 1;
const ESP_PD_OPTION_AUTO: u32 = 2;
const ESP_PD_DOMAIN_RTC_PERIPH: u32 = 0;
//#[link_section = ".rtc.force_fast"]
//static mut RUN_TIME: MaybeUninit<Duration> = MaybeUninit::uninit();

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let wakeup_reason = WakeupReason::get();
    info!("Wakeup reason: {:?}", wakeup_reason);

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let mut led = PinDriver::output_od(pins.gpio13)?;
    let mut status_led = BlinkDriver::new(pins.gpio10)?;

    match wakeup_reason {
        WakeupReason::Unknown => {
            status_led.blink(1, 1000)?;
            init_upl_program(peripherals.ulp)?;
        }

        WakeupReason::ULP => {
            status_led.blink(2, 300)?;
        }

        WakeupReason::CoCpu => {
            status_led.blink(3, 500)?;
        }

        WakeupReason::CoCpuTrapTrig => {
            status_led.blink(4, 500)?;
        }

        _ => {
            status_led.blink(5, 500)?;
        }
    };

    for _ in 0..4 {
        led.set_high()?;
        std::thread::sleep(core::time::Duration::from_millis(500));
        led.set_low()?;
        std::thread::sleep(core::time::Duration::from_millis(500));
    }

    drop(led);

    unsafe {
        esp!(esp_idf_sys::esp_sleep_pd_config(
            ESP_PD_DOMAIN_RTC_PERIPH,
            ESP_PD_OPTION_ON
        ))?;

        esp!(esp_idf_sys::esp_sleep_enable_ulp_wakeup())?;
        esp_idf_sys::esp_deep_sleep_start();
    }
}

fn init_upl_program(ulp: esp_idf_hal::ulp::ULP) -> anyhow::Result<()> {
    let mut ulp_driver = esp_idf_hal::ulp::UlpDriver::new(ulp)?;

    unsafe {
        ulp_driver.load(ULP)?;

        info!("RiscV ULP binary loaded successfully");

        // suppress boot messages
        esp_idf_sys::esp_deep_sleep_disable_rom_logging();
        esp!(esp_idf_sys::ulp_set_wakeup_period(0, 500_000))?;

        ulp_driver.start();
        info!("RiscV ULP started");
    }
    Ok(())
}

pub struct BlinkDriver<P>
where
    P: Pin,
{
    pin: PinDriver<'static, P, Output>,
}

impl<P: OutputPin> BlinkDriver<P> {
    pub fn new(pin: impl Peripheral<P = P> + 'static) -> Result<BlinkDriver<P>, EspError> {
        Ok(BlinkDriver {
            pin: PinDriver::output(pin)?,
        })
    }

    fn blink(&mut self, times: u8, delay_time: u64) -> Result<(), EspError> {
        for _ in 0..times {
            self.pin.set_high().unwrap();
            std::thread::sleep(core::time::Duration::from_millis(delay_time));
            self.pin.set_low().unwrap();
            std::thread::sleep(core::time::Duration::from_millis(delay_time));
        }
        Ok(())
    }
}
