use crate::js;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;
use wasm_bindgen::JsCast;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(u64);

impl Instant {
    pub fn now() -> Self {
        let global = js_sys::global();
        let global_scope = global.unchecked_ref::<js::GlobalScope>();
        let perf = global_scope.performance();
        let now = ((perf.time_origin() + perf.now()) * 1000.0) as u64;
        Self(now)
    }

    pub fn from_raw_us(us: u64) -> Self {
        Self(us)
    }

    pub fn raw_us(&self) -> u64 {
        self.0
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        self.0.checked_sub(earlier.0).map(Duration::from_micros)
    }

    pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }

    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
        self.0
            .checked_add(duration.as_micros().try_into().unwrap())
            .map(Instant)
    }

    pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
        self.0
            .checked_sub(duration.as_micros().try_into().unwrap())
            .map(Instant)
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, other: Duration) -> Self::Output {
        self.checked_add(other)
            .expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, other: Duration) -> Self::Output {
        self.checked_sub(other)
            .expect("overflow when substraction duration from instant")
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Self::Output {
        self.duration_since(other)
    }
}

pub const UNIX_EPOCH: SystemTime = SystemTime(0);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SystemTime(u64);

impl SystemTime {
    pub const UNIX_EPOCH: SystemTime = UNIX_EPOCH;

    pub fn now() -> SystemTime {
        let us = (js_sys::Date::now() * 1000f64) as u64;
        SystemTime(us)
    }

    pub fn duration_since(&self, earlier: SystemTime) -> Result<Duration, SystemTimeError> {
        if self.0 < earlier.0 {
            return Err(SystemTimeError(Duration::from_micros(earlier.0 - self.0)));
        }

        Ok(Duration::from_micros(self.0 - earlier.0))
    }

    pub fn elapsed(&self) -> Result<Duration, SystemTimeError> {
        SystemTime::now().duration_since(*self)
    }

    pub fn checked_add(&self, duration: Duration) -> Option<SystemTime> {
        self.0
            .checked_add(duration.as_micros().try_into().unwrap())
            .map(SystemTime)
    }

    pub fn checked_sub(&self, duration: Duration) -> Option<SystemTime> {
        self.0
            .checked_sub(duration.as_micros().try_into().unwrap())
            .map(SystemTime)
    }
}

impl Add<Duration> for SystemTime {
    type Output = SystemTime;

    fn add(self, other: Duration) -> Self::Output {
        self.checked_add(other)
            .expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl Sub<Duration> for SystemTime {
    type Output = SystemTime;

    fn sub(self, other: Duration) -> Self::Output {
        self.checked_sub(other)
            .expect("overflow when substraction duration from instant")
    }
}

impl SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

#[derive(Debug, Clone)]
pub struct SystemTimeError(Duration);

impl SystemTimeError {
    pub fn duration(&self) -> Duration {
        self.0
    }
}

impl std::fmt::Display for SystemTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "second time provided was later than self")
    }
}
