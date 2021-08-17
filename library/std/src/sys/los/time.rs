use crate::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemTime;

pub const UNIX_EPOCH: SystemTime = SystemTime;

impl Instant {
    pub fn now() -> Instant {
        Instant
    }

    pub const fn zero() -> Instant {
        Instant
    }

    pub fn actually_monotonic() -> bool {
        true
    }

    pub fn checked_sub_instant(&self, _other: &Instant) -> Option<Duration> {
        None
    }

    pub fn checked_add_duration(&self, _other: &Duration) -> Option<Instant> {
        None
    }

    pub fn checked_sub_duration(&self, _other: &Duration) -> Option<Instant> {
        None
    }
}

impl SystemTime {
    pub fn now() -> SystemTime {
        SystemTime
    }

    pub fn sub_time(&self, _other: &SystemTime) -> Result<Duration, Duration> {
        Ok(Duration::new(0, 0))
    }

    pub fn checked_add_duration(&self, _other: &Duration) -> Option<SystemTime> {
        None
    }

    pub fn checked_sub_duration(&self, _other: &Duration) -> Option<SystemTime> {
        None
    }
}
