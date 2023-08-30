use crate::EnumError;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Copy, Clone)]
pub struct DateTime {
    inner: u32,
}

impl DateTime {
    // TODO: Make fallible?
    pub const fn new(
        years_after_2000: u8,
        month: Month,
        month_day: u8,
        weekday: Weekday,
        hours: u8,
        minutes: u8,
    ) -> Self {
        let years_after_2000 = years_after_2000 as u32;
        let month = month.as_int();
        let month_day = month_day as u32;

        let weekday = weekday.as_int();
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

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

    pub const fn minutes(&self) -> u8 {
        minutes(self.inner) as u8
    }

    pub const fn hours(&self) -> u8 {
        hours(self.inner) as u8
    }

    pub const fn weekday(&self) -> Weekday {
        match weekday(self.inner) {
            0 => Weekday::Sunday,
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            6 => Weekday::Saturday,
            _ => unreachable!(),
        }
    }

    pub const fn month_day(&self) -> u8 {
        month_day(self.inner) as u8
    }

    pub const fn month(&self) -> Month {
        match month(self.inner) {
            0 => Month::January,
            1 => Month::February,
            2 => Month::March,
            3 => Month::April,
            4 => Month::May,
            5 => Month::June,
            6 => Month::July,
            7 => Month::August,
            8 => Month::September,
            9 => Month::October,
            10 => Month::November,
            11 => Month::December,
            _ => unreachable!(),
        }
    }

    pub const fn years_after_2000(&self) -> u8 {
        years_after_2000(self.inner) as u8
    }
}

const fn minutes(v: u32) -> u32 {
    v & 0b111111
}

const fn hours(v: u32) -> u32 {
    (v >> 6) & 0b11111
}

const fn weekday(v: u32) -> u32 {
    (v >> 11) & 0b111
}

const fn month_day(v: u32) -> u32 {
    (v >> 14) & 0b111111
}

const fn month(v: u32) -> u32 {
    (v >> 20) & 0b1111
}

const fn years_after_2000(v: u32) -> u32 {
    (v >> 24) & 0b11111111
}

impl TryFrom<u32> for DateTime {
    type Error = EnumError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let minutes = minutes(value);

        let hours = hours(value);

        let weekday = weekday(value);
        let weekday = Weekday::try_from(weekday)?;

        let month_day = month_day(value);

        let month = month(value);
        let month = Month::try_from(month)?;

        let years_after_2000 = years_after_2000(value);

        Ok(Self::new(
            years_after_2000 as u8,
            month,
            month_day as u8,
            weekday,
            hours as u8,
            minutes as u8,
        ))
    }
}

#[cfg(feature = "chrono")]
impl<T: chrono::prelude::TimeZone> TryFrom<chrono::prelude::DateTime<T>> for DateTime {
    type Error = &'static str;

    fn try_from(dt: chrono::prelude::DateTime<T>) -> Result<Self, Self::Error> {
        use chrono::prelude::*;
        let date_time = Self::new(
            (dt.year() - 2000)
                .try_into()
                .map_err(|_| "Year does not fit in byte")?,
            dt.month0().try_into().unwrap(),
            dt.day0()
                .try_into()
                .map_err(|_| "Day does not fit in byte")?,
            dt.weekday().num_days_from_sunday().try_into().unwrap(),
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    const fn as_int(&self) -> u32 {
        match self {
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
            Weekday::Sunday => 0,
        }
    }
}

const WEEKDAY: &str = "Weekday";

impl Default for Weekday {
    fn default() -> Self {
        Self::Monday
    }
}

impl TryFrom<u32> for Weekday {
    type Error = EnumError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Sunday,
            1 => Self::Monday,
            2 => Self::Tuesday,
            3 => Self::Wednesday,
            4 => Self::Thursday,
            5 => Self::Friday,
            6 => Self::Saturday,
            v => return Err(EnumError::new("Weekday", v.into())),
        })
    }
}

impl TryFrom<u8> for Weekday {
    type Error = EnumError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(Into::<u32>::into(value))
    }
}

impl TryFrom<u16> for Weekday {
    type Error = EnumError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(Into::<u32>::into(value))
    }
}

impl TryFrom<u64> for Weekday {
    type Error = EnumError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a =
            TryInto::<u32>::try_into(value).map_err(|_| EnumError::new(WEEKDAY, value.into()))?;
        a.try_into()
    }
}

impl TryFrom<i8> for Weekday {
    type Error = EnumError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let a =
            TryInto::<u32>::try_into(value).map_err(|_| EnumError::new(WEEKDAY, value.into()))?;
        a.try_into()
    }
}

impl TryFrom<i16> for Weekday {
    type Error = EnumError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let a =
            TryInto::<u32>::try_into(value).map_err(|_| EnumError::new(WEEKDAY, value.into()))?;
        a.try_into()
    }
}

impl TryFrom<i32> for Weekday {
    type Error = EnumError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let a =
            TryInto::<u32>::try_into(value).map_err(|_| EnumError::new(WEEKDAY, value.into()))?;
        a.try_into()
    }
}

impl TryFrom<i64> for Weekday {
    type Error = EnumError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let a =
            TryInto::<u32>::try_into(value).map_err(|_| EnumError::new(WEEKDAY, value.into()))?;
        a.try_into()
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    const fn as_int(&self) -> u32 {
        match self {
            Month::January => 0,
            Month::February => 1,
            Month::March => 2,
            Month::April => 3,
            Month::May => 4,
            Month::June => 5,
            Month::July => 6,
            Month::August => 7,
            Month::September => 8,
            Month::October => 9,
            Month::November => 10,
            Month::December => 11,
        }
    }
}

const MONTH: &str = "Month";

impl Default for Month {
    fn default() -> Self {
        Self::January
    }
}

impl TryFrom<u32> for Month {
    type Error = EnumError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Month::January,
            1 => Month::February,
            2 => Month::March,
            3 => Month::April,
            4 => Month::May,
            5 => Month::June,
            6 => Month::July,
            7 => Month::August,
            8 => Month::September,
            9 => Month::October,
            10 => Month::November,
            11 => Month::December,
            v => return Err(EnumError::new("Month", v.into())),
        })
    }
}

impl TryFrom<u8> for Month {
    type Error = EnumError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(Into::<u32>::into(value))
    }
}

impl TryFrom<u16> for Month {
    type Error = EnumError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(Into::<u32>::into(value))
    }
}

impl TryFrom<u64> for Month {
    type Error = EnumError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).map_err(|_| EnumError::new(MONTH, value.into()))?;
        a.try_into()
    }
}

impl TryFrom<i8> for Month {
    type Error = EnumError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).map_err(|_| EnumError::new(MONTH, value.into()))?;
        a.try_into()
    }
}

impl TryFrom<i16> for Month {
    type Error = EnumError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).map_err(|_| EnumError::new(MONTH, value.into()))?;
        a.try_into()
    }
}

impl TryFrom<i32> for Month {
    type Error = EnumError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).map_err(|_| EnumError::new(MONTH, value.into()))?;
        a.try_into()
    }
}

impl TryFrom<i64> for Month {
    type Error = EnumError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).map_err(|_| EnumError::new(MONTH, value.into()))?;
        a.try_into()
    }
}
