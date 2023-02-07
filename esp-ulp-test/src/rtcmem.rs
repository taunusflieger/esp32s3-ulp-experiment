/// Persist simple types and structs across deep sleep on esp32 MCUs.
/// These are only initialized when the reset reason is not DeepSleep.
///
/// SAFETY: Do NOT store anything with pointers here as these would
///         escape the confines of RTC memory. `RTCMemPersist` MUST
///         be created before the first deep sleep or it would not
///         be initialized and that will cause undefined behavior.
///
/// # Example use:
/// ```
/// use std::mem::MaybeUninit;
/// use esp_idf_hal::reset::ResetReason;
///
/// main() {
///     let counter = unsafe {
///         // NOTE: the `mut static` is here so its not available to anyone but
///         // RTCMemPersist
///         #[link_section = ".rtc.force_fast"]
///         static mut COUNTER: MaybeUninit<u32> = MaybeUninit::uninit();
///         RTCMemPersist::new(&mut COUNTER, 0)
///     };
///     println!("deep sleep count: {}", counter.get());
///     count.set(count.get()+1);
///     unsafe {
///         esp!(esp_idf_sys::esp_sleep_enable_timer_wakeup(
///             60 as u64 * 1000000 // one minute
///         )).unwrap();
///         esp_idf_sys::esp_deep_sleep_start();
///     }
/// }
/// ```
use std::mem::MaybeUninit;

use esp_idf_hal::reset::ResetReason;

pub struct RTCMemPersist<'a, T> {
    object: &'a mut MaybeUninit<T>,
}

impl<'a, T> RTCMemPersist<'a, T> {
    pub unsafe fn new(object: &'a mut MaybeUninit<T>, def: T) -> Self {
        if ResetReason::get() != ResetReason::DeepSleep {
            object.write(def);
        }
        Self { object }
    }
    pub fn get(&self) -> T {
        unsafe { self.object.assume_init_read() }
    }
    pub fn set(&mut self, value: T) {
        let x = unsafe { self.object.assume_init_mut() };
        *x = value;
    }
}
