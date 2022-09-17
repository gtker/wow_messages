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

#[cfg(feature = "chrono")]
impl<T: chrono::prelude::TimeZone> std::convert::TryFrom<chrono::prelude::DateTime<T>>
    for DateTime
{
    type Error = &'static str;

    fn try_from(dt: chrono::prelude::DateTime<T>) -> Result<Self, Self::Error> {
        use chrono::prelude::*;
        use std::convert::TryInto;
        let date_time = Self::new(
            (dt.year() - 2000)
                .try_into()
                .map_err(|_| "Year does not fit in byte")?,
            dt.month0()
                .try_into()
                .map_err(|_| "Month does not fit in byte")?,
            dt.day0()
                .try_into()
                .map_err(|_| "Day does not fit in byte")?,
            dt.weekday()
                .num_days_from_monday()
                .try_into()
                .map_err(|_| "Day of week does not fit in byte")?,
            dt.hour()
                .try_into()
                .map_err(|_| "Hour does not fit in byte")?,
            dt.minute()
                .try_into()
                .map_err(|_| "Minute does fit in byte")?,
        );
        Ok(date_time)
    }
}
