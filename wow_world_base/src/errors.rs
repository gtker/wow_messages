use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error enum for when an integer is out of range.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EnumError {
    pub name: &'static str,
    pub value: i128,
}

impl EnumError {
    pub const fn new(name: &'static str, value: i128) -> Self {
        Self { name, value }
    }
}

impl Display for EnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Enum '{}' can not have value: '{}'",
            self.name, self.value
        ))
    }
}

impl std::error::Error for EnumError {}

/// Error enum for when an integer is out of range.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DateTimeError {
    EnumError(EnumError),
    InvalidMinute(u8),
    InvalidHour(u8),
    InvalidMonthDay {
        month: u8,
        day: u8,
    },
    InvalidDate {
        year_after_2000: u8,
        month: u8,
        month_day: u8,
        weekday: u8,
        predicted_weekday: u8,
    },
}

impl Display for DateTimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DateTimeError::EnumError(e) => e.fmt(f),
            DateTimeError::InvalidMinute(v) => write!(f, "Minute {v} higher than allowed 59"),
            DateTimeError::InvalidHour(v) => write!(f, "Hour {v} is higher than allowed 23"),
            DateTimeError::InvalidMonthDay { month, day } => {
                write!(f, "Invalid day '{day}' for month {month:?}")
            }
            DateTimeError::InvalidDate {
                year_after_2000,
                month,
                month_day,
                weekday,
                predicted_weekday,
            } => {
                let month = month + 1;

                let get_weekday = |weekday: u8| match weekday {
                    0 => "Sunday",
                    1 => "Monday",
                    2 => "Tuesday",
                    3 => "Wednesday",
                    4 => "Thursday",
                    5 => "Friday",
                    6 => "Saturday",
                    _ => "Unknown weekday",
                };

                let month_day = month_day + 1;
                let year = *year_after_2000 as u32 + 2000;

                let weekday = get_weekday(*weekday);
                let expected_weekday = get_weekday(*predicted_weekday);

                write!(f, "{year}-{month}-{month_day} is not a '{weekday}', but instead a '{expected_weekday}'")
            }
        }
    }
}

impl std::error::Error for DateTimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DateTimeError::EnumError(e) => Some(e),
            DateTimeError::InvalidMinute(_)
            | DateTimeError::InvalidHour(_)
            | DateTimeError::InvalidMonthDay { .. }
            | DateTimeError::InvalidDate { .. } => None,
        }
    }
}

impl From<EnumError> for DateTimeError {
    fn from(value: EnumError) -> Self {
        Self::EnumError(value)
    }
}
