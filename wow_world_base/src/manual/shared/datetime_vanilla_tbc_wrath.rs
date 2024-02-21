use crate::{DateTimeError, EnumError};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
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

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let weekday = self.weekday();
        let year = self.years_after_2000() as u32 + 2000;
        let month = self.month().iso8601();
        let day = self.month_day() + 1;
        let hour = self.hours();
        let minute = self.minutes();

        write!(f, "{weekday} {year}-{month}-{day}T{hour}:{minute}")
    }
}

impl TryFrom<u32> for DateTime {
    type Error = DateTimeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let minutes = minutes(value) as u8;
        if minutes > 59 {
            return Err(DateTimeError::InvalidMinute(minutes));
        }

        let hours = hours(value) as u8;
        if hours > 23 {
            return Err(DateTimeError::InvalidHour(hours));
        }

        let weekday = weekday(value);
        let weekday = Weekday::try_from(weekday)?;

        let month = month(value);
        let month = Month::try_from(month)?;

        let years_after_2000 = years_after_2000(value) as u8;

        let month_day = month_day(value) as u8;
        if month_day > month.maximum_days(years_after_2000) {
            return Err(DateTimeError::InvalidMonthDay {
                month: month.as_int() as u8,
                day: month_day,
            });
        }

        let predicted_weekday = predicted_weekday(month_day, month, years_after_2000);

        if weekday != predicted_weekday {
            return Err(DateTimeError::InvalidDate {
                year_after_2000: years_after_2000,
                month: month.as_int() as u8,
                month_day,
                weekday: weekday.as_int() as u8,
                predicted_weekday: predicted_weekday.as_int() as u8,
            });
        }

        Ok(Self::new(
            years_after_2000,
            month,
            month_day,
            weekday,
            hours,
            minutes,
        ))
    }
}

// Rata Die algorithm using Jan 1st 2000 as the base
fn predicted_weekday(month_day: u8, month: Month, years_after_2000: u8) -> Weekday {
    const DAYS_IN_WEEK: u32 = 7;
    const SATURDAY: u32 = 5;

    let january_first_2000_weekday = SATURDAY;

    let number_of_days_since_january_first_2000 = {
        let mut leap_year_days = years_after_2000 / 4 - years_after_2000 / 100;
        if years_after_2000 > 0 {
            leap_year_days += 1; // 2000 was a leap year
        }
        if leap_year(years_after_2000 as u32) && leap_year_days > 0 {
            leap_year_days -= 1; // Leap day is added in months
        }

        let days_from_years = years_after_2000 as u32 * 365 + leap_year_days as u32;

        let days_from_month = month.days_from_previous_months(years_after_2000);
        days_from_years + days_from_month + month_day as u32
    };

    let day = (january_first_2000_weekday + number_of_days_since_january_first_2000) % DAYS_IN_WEEK;

    match day {
        0 => Weekday::Monday,
        1 => Weekday::Tuesday,
        2 => Weekday::Wednesday,
        3 => Weekday::Thursday,
        4 => Weekday::Friday,
        5 => Weekday::Saturday,
        6 => Weekday::Sunday,
        DAYS_IN_WEEK..=u32::MAX => {
            unreachable!("value greater than modulus DAYS_IN_WEEK not possible")
        }
    }
}

#[test]
fn predicted_weekdays() {
    const DATA: &[((u8, Month, u8), Weekday)] = &[
        ((0, Month::January, 0), Weekday::Saturday), // Jan 1st 2000
        ((1, Month::January, 0), Weekday::Sunday),   // Jan 2nd 2000
        ((2, Month::January, 0), Weekday::Monday),   // Jan 3rd 2000
        ((0, Month::February, 0), Weekday::Tuesday), // Feb 1st 2000
        ((28, Month::February, 0), Weekday::Tuesday), // Feb 29th 2000
        ((0, Month::March, 0), Weekday::Wednesday),  // Mar 1st 2000
        ((14, Month::March, 0), Weekday::Wednesday), // Mar 15th 2000
        ((30, Month::March, 0), Weekday::Friday),    // Mar 31st 2000
        ((23, Month::June, 0), Weekday::Saturday),   // Jun 24th 2000
        ((29, Month::November, 0), Weekday::Thursday), // Nov 30th 2000
        ((0, Month::December, 0), Weekday::Friday),  // Dec 1st 2000
        ((30, Month::December, 0), Weekday::Sunday), // Dec 31st 2000
        ((0, Month::January, 1), Weekday::Monday),   // Jan 1st 2001
        ((1, Month::January, 1), Weekday::Tuesday),  // Jan 2nd 2001
        ((2, Month::January, 1), Weekday::Wednesday), // Jan 3rd 2001
        ((0, Month::February, 1), Weekday::Thursday), // Feb 1st 2001
        ((11, Month::October, 22), Weekday::Wednesday),
        ((30, Month::March, 3), Weekday::Monday),
        ((30, Month::December, 3), Weekday::Wednesday),
        ((0, Month::January, 4), Weekday::Thursday),
        ((23, Month::June, 4), Weekday::Thursday),
        ((4, Month::December, 8), Weekday::Friday),
        ((19, Month::September, 12), Weekday::Thursday),
        ((21, Month::February, 35), Weekday::Thursday),
        ((8, Month::July, 36), Weekday::Wednesday),
        ((25, Month::July, 39), Weekday::Tuesday),
        ((8, Month::July, 59), Weekday::Wednesday),
        ((24, Month::February, 72), Weekday::Thursday),
        ((17, Month::December, 87), Weekday::Thursday),
        ((1, Month::March, 90), Weekday::Thursday),
        ((1, Month::July, 132), Weekday::Wednesday),
        ((10, Month::August, 152), Weekday::Friday),
        ((6, Month::June, 157), Weekday::Tuesday),
        ((24, Month::November, 184), Weekday::Thursday),
        ((19, Month::May, 190), Weekday::Thursday),
        ((19, Month::March, 199), Weekday::Wednesday),
        ((1, Month::September, 212), Weekday::Wednesday),
        ((15, Month::October, 212), Weekday::Friday),
        ((8, Month::June, 218), Weekday::Tuesday),
        ((13, Month::June, 222), Weekday::Friday),
        ((1, Month::November, 224), Weekday::Tuesday),
        ((13, Month::June, 233), Weekday::Friday),
        ((15, Month::December, 247), Weekday::Thursday),
        ((14, Month::April, 250), Weekday::Monday),
        ((13, Month::October, 26), Weekday::Wednesday),
        ((17, Month::September, 35), Weekday::Tuesday),
        ((3, Month::August, 44), Weekday::Thursday),
        ((28, Month::May, 62), Weekday::Monday),
        ((24, Month::July, 69), Weekday::Thursday),
        ((12, Month::March, 82), Weekday::Friday),
        ((17, Month::August, 107), Weekday::Thursday),
        ((12, Month::September, 118), Weekday::Tuesday),
        ((7, Month::May, 125), Weekday::Tuesday),
        ((6, Month::May, 126), Weekday::Tuesday),
        ((28, Month::June, 135), Weekday::Wednesday),
        ((7, Month::June, 141), Weekday::Thursday),
        ((23, Month::October, 146), Weekday::Monday),
        ((17, Month::August, 158), Weekday::Friday),
        ((13, Month::July, 160), Weekday::Monday),
        ((24, Month::April, 171), Weekday::Thursday),
        ((25, Month::September, 177), Weekday::Friday),
        ((8, Month::May, 183), Weekday::Friday),
        ((19, Month::November, 183), Weekday::Thursday),
        ((2, Month::July, 197), Weekday::Monday),
        ((15, Month::August, 211), Weekday::Friday),
        ((23, Month::January, 223), Weekday::Friday),
        ((26, Month::April, 242), Weekday::Wednesday),
        ((21, Month::July, 246), Weekday::Wednesday),
        ((17, Month::March, 247), Weekday::Thursday),
        ((12, Month::May, 33), Weekday::Friday),
        ((27, Month::March, 35), Weekday::Wednesday),
        ((12, Month::January, 38), Weekday::Wednesday),
        ((18, Month::February, 42), Weekday::Wednesday),
        ((5, Month::September, 45), Weekday::Wednesday),
        ((13, Month::September, 55), Weekday::Tuesday),
        ((18, Month::September, 56), Weekday::Tuesday),
        ((3, Month::December, 68), Weekday::Tuesday),
        ((10, Month::May, 90), Weekday::Thursday),
        ((15, Month::December, 106), Weekday::Thursday),
        ((22, Month::August, 112), Weekday::Tuesday),
        ((22, Month::July, 115), Weekday::Tuesday),
        ((5, Month::September, 135), Weekday::Tuesday),
        ((14, Month::April, 149), Weekday::Tuesday),
        ((19, Month::August, 150), Weekday::Thursday),
        ((21, Month::May, 164), Weekday::Tuesday),
        ((27, Month::December, 170), Weekday::Friday),
        ((30, Month::January, 180), Weekday::Monday),
        ((14, Month::September, 183), Weekday::Monday),
        ((11, Month::March, 184), Weekday::Friday),
        ((1, Month::June, 202), Weekday::Wednesday),
        ((0, Month::May, 215), Weekday::Monday),
        ((15, Month::April, 232), Weekday::Monday),
        ((21, Month::December, 248), Weekday::Friday),
        ((14, Month::March, 255), Weekday::Thursday),
        ((27, Month::August, 6), Weekday::Monday),
        ((6, Month::May, 9), Weekday::Thursday),
        ((7, Month::February, 11), Weekday::Tuesday),
        ((7, Month::February, 16), Weekday::Monday),
        ((29, Month::May, 18), Weekday::Wednesday),
        ((6, Month::February, 39), Weekday::Monday),
        ((14, Month::October, 42), Weekday::Wednesday),
        ((9, Month::November, 44), Weekday::Thursday),
        ((16, Month::March, 59), Weekday::Monday),
        ((2, Month::November, 59), Weekday::Monday),
        ((23, Month::January, 62), Weekday::Tuesday),
        ((15, Month::July, 64), Weekday::Wednesday),
        ((25, Month::February, 65), Weekday::Thursday),
        ((24, Month::April, 75), Weekday::Thursday),
        ((19, Month::October, 100), Weekday::Wednesday),
        ((5, Month::June, 126), Weekday::Thursday),
        ((0, Month::January, 161), Weekday::Thursday),
        ((0, Month::January, 173), Weekday::Friday),
        ((20, Month::July, 195), Weekday::Tuesday),
        ((11, Month::August, 208), Weekday::Friday),
        ((5, Month::March, 216), Weekday::Wednesday),
        ((16, Month::April, 221), Weekday::Tuesday),
        ((4, Month::September, 223), Weekday::Friday),
        ((23, Month::January, 226), Weekday::Tuesday),
        ((14, Month::January, 250), Weekday::Tuesday),
    ];

    for (i, ((month_day, month, years_after_2000), expected)) in DATA.iter().enumerate() {
        assert_eq!(
            *expected,
            predicted_weekday(*month_day, *month, *years_after_2000),
            "index {i}: {month_day}+1, {month:?}, year 2000+{years_after_2000}"
        );
    }
}

const fn leap_year(years_after_2000: u32) -> bool {
    let year = years_after_2000 + 2000;
    if year % 4 == 0 && year % 100 != 0 {
        true
    } else {
        year % 400 == 0
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

impl std::fmt::Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Saturday => "Saturday",
            Weekday::Sunday => "Sunday",
        })
    }
}

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

    pub const fn iso8601(&self) -> u32 {
        self.as_int() + 1
    }

    const fn maximum_days(&self, years_after_2000: u8) -> u8 {
        match self {
            Month::January => 31,
            Month::February => {
                if leap_year(years_after_2000 as u32) {
                    29
                } else {
                    28
                }
            }
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }

    const fn days_from_previous_months(&self, years_after_2000: u8) -> u32 {
        match self {
            Month::January => 0,
            Month::February => Self::January.maximum_days(years_after_2000) as u32,
            Month::March => {
                Self::January.maximum_days(years_after_2000) as u32
                    + Self::February.maximum_days(years_after_2000) as u32
            }
            Month::April => {
                Self::January.maximum_days(years_after_2000) as u32
                    + Self::February.maximum_days(years_after_2000) as u32
                    + Self::March.maximum_days(years_after_2000) as u32
            }
            Month::May => {
                Self::January.maximum_days(years_after_2000) as u32
                    + Self::February.maximum_days(years_after_2000) as u32
                    + Self::March.maximum_days(years_after_2000) as u32
                    + Self::April.maximum_days(years_after_2000) as u32
            }
            Month::June => {
                Self::January.maximum_days(years_after_2000) as u32
                    + Self::February.maximum_days(years_after_2000) as u32
                    + Self::March.maximum_days(years_after_2000) as u32
                    + Self::April.maximum_days(years_after_2000) as u32
                    + Self::May.maximum_days(years_after_2000) as u32
            }
            Month::July => {
                Self::January.maximum_days(years_after_2000) as u32
                    + Self::February.maximum_days(years_after_2000) as u32
                    + Self::March.maximum_days(years_after_2000) as u32
                    + Self::April.maximum_days(years_after_2000) as u32
                    + Self::May.maximum_days(years_after_2000) as u32
                    + Self::June.maximum_days(years_after_2000) as u32
            }
            Month::August => {
                Self::January.maximum_days(years_after_2000) as u32
                    + Self::February.maximum_days(years_after_2000) as u32
                    + Self::March.maximum_days(years_after_2000) as u32
                    + Self::April.maximum_days(years_after_2000) as u32
                    + Self::May.maximum_days(years_after_2000) as u32
                    + Self::June.maximum_days(years_after_2000) as u32
                    + Self::July.maximum_days(years_after_2000) as u32
            }
            Month::September => {
                Self::January.maximum_days(years_after_2000) as u32
                    + Self::February.maximum_days(years_after_2000) as u32
                    + Self::March.maximum_days(years_after_2000) as u32
                    + Self::April.maximum_days(years_after_2000) as u32
                    + Self::May.maximum_days(years_after_2000) as u32
                    + Self::June.maximum_days(years_after_2000) as u32
                    + Self::July.maximum_days(years_after_2000) as u32
                    + Self::August.maximum_days(years_after_2000) as u32
            }
            Month::October => {
                Self::January.maximum_days(years_after_2000) as u32
                    + Self::February.maximum_days(years_after_2000) as u32
                    + Self::March.maximum_days(years_after_2000) as u32
                    + Self::April.maximum_days(years_after_2000) as u32
                    + Self::May.maximum_days(years_after_2000) as u32
                    + Self::June.maximum_days(years_after_2000) as u32
                    + Self::July.maximum_days(years_after_2000) as u32
                    + Self::August.maximum_days(years_after_2000) as u32
                    + Self::September.maximum_days(years_after_2000) as u32
            }
            Month::November => {
                Self::January.maximum_days(years_after_2000) as u32
                    + Self::February.maximum_days(years_after_2000) as u32
                    + Self::March.maximum_days(years_after_2000) as u32
                    + Self::April.maximum_days(years_after_2000) as u32
                    + Self::May.maximum_days(years_after_2000) as u32
                    + Self::June.maximum_days(years_after_2000) as u32
                    + Self::July.maximum_days(years_after_2000) as u32
                    + Self::August.maximum_days(years_after_2000) as u32
                    + Self::September.maximum_days(years_after_2000) as u32
                    + Self::October.maximum_days(years_after_2000) as u32
            }
            Month::December => {
                Self::January.maximum_days(years_after_2000) as u32
                    + Self::February.maximum_days(years_after_2000) as u32
                    + Self::March.maximum_days(years_after_2000) as u32
                    + Self::April.maximum_days(years_after_2000) as u32
                    + Self::May.maximum_days(years_after_2000) as u32
                    + Self::June.maximum_days(years_after_2000) as u32
                    + Self::July.maximum_days(years_after_2000) as u32
                    + Self::August.maximum_days(years_after_2000) as u32
                    + Self::September.maximum_days(years_after_2000) as u32
                    + Self::October.maximum_days(years_after_2000) as u32
                    + Self::November.maximum_days(years_after_2000) as u32
            }
        }
    }
}

const MONTH: &str = "Month";

impl Default for Month {
    fn default() -> Self {
        Self::January
    }
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        })
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
