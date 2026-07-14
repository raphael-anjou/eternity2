#![forbid(unsafe_code)]
#![no_std]

// Monotonic-microsecond clock that works on native and wasm32 browsers.
// On native: std::time::Instant. On wasm32: Date::now() in milliseconds
// scaled to microseconds (ms resolution is fine for the educational mode
// that never runs >100 ms anyway).

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

#[cfg(not(target_arch = "wasm32"))]
pub struct Clock(std::time::Instant);

#[cfg(not(target_arch = "wasm32"))]
impl Clock {
    #[must_use]
    pub fn now() -> Self {
        Self(std::time::Instant::now())
    }
    #[must_use]
    pub fn elapsed_us(&self) -> u64 {
        u64::try_from(self.0.elapsed().as_micros()).unwrap_or(u64::MAX)
    }
}

#[cfg(target_arch = "wasm32")]
pub struct Clock(f64);

#[cfg(target_arch = "wasm32")]
impl Clock {
    #[must_use]
    pub fn now() -> Self {
        Self(js_sys::Date::now())
    }
    #[must_use]
    pub fn elapsed_us(&self) -> u64 {
        let now = js_sys::Date::now();
        ((now - self.0).max(0.0) * 1000.0) as u64
    }
}
