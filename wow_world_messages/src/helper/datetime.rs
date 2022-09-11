#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Copy, Clone)]
pub struct DateTime {
    inner: u32,
}

impl DateTime {
    pub const fn new(
        years_after_2000: u8,
        month: u8,
        month_day: u8,
        weekday: u8,
        hours: u8,
        minutes: u8,
    ) -> Self {
        let years_after_2000 = years_after_2000 as u32;
        let month = month as u32;
        let month_day = month_day as u32;

        let weekday = weekday as u32;
        let hours = hours as u32;
        let minutes = minutes as u32;

        let inner = years_after_2000 << 24
            | month << 20
            | month_day << 14
            | weekday << 11
            | hours << 6
            | minutes;

        Self { inner }
    }

    pub(crate) const fn from_int(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }
}
