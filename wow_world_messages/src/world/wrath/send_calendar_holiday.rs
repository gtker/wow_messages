use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_send_calendar.wowm:41`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_send_calendar.wowm#L41):
/// ```text
/// struct SendCalendarHoliday {
///     u32 holiday_id;
///     u32 region;
///     u32 looping;
///     u32 priority;
///     u32 calendar_filter_type;
///     u32[26] holiday_days;
///     u32[10] durations;
///     u32[10] flags;
///     CString texture_file_name;
/// }
/// ```
pub struct SendCalendarHoliday {
    pub holiday_id: u32,
    pub region: u32,
    pub looping: u32,
    pub priority: u32,
    pub calendar_filter_type: u32,
    pub holiday_days: [u32; 26],
    pub durations: [u32; 10],
    pub flags: [u32; 10],
    pub texture_file_name: String,
}

impl SendCalendarHoliday {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // holiday_id: u32
        w.write_all(&self.holiday_id.to_le_bytes())?;

        // region: u32
        w.write_all(&self.region.to_le_bytes())?;

        // looping: u32
        w.write_all(&self.looping.to_le_bytes())?;

        // priority: u32
        w.write_all(&self.priority.to_le_bytes())?;

        // calendar_filter_type: u32
        w.write_all(&self.calendar_filter_type.to_le_bytes())?;

        // holiday_days: u32[26]
        for i in self.holiday_days.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // durations: u32[10]
        for i in self.durations.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // flags: u32[10]
        for i in self.flags.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // texture_file_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.texture_file_name.as_bytes().iter().next_back(), Some(&0_u8), "String `texture_file_name` must not be null-terminated.");
        w.write_all(self.texture_file_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl SendCalendarHoliday {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // holiday_id: u32
        let holiday_id = crate::util::read_u32_le(&mut r)?;

        // region: u32
        let region = crate::util::read_u32_le(&mut r)?;

        // looping: u32
        let looping = crate::util::read_u32_le(&mut r)?;

        // priority: u32
        let priority = crate::util::read_u32_le(&mut r)?;

        // calendar_filter_type: u32
        let calendar_filter_type = crate::util::read_u32_le(&mut r)?;

        // holiday_days: u32[26]
        let holiday_days = {
            let mut holiday_days = [u32::default(); 26];
            for i in holiday_days.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            holiday_days
        };

        // durations: u32[10]
        let durations = {
            let mut durations = [u32::default(); 10];
            for i in durations.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            durations
        };

        // flags: u32[10]
        let flags = {
            let mut flags = [u32::default(); 10];
            for i in flags.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            flags
        };

        // texture_file_name: CString
        let texture_file_name = {
            let texture_file_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(texture_file_name)?
        };

        Ok(Self {
            holiday_id,
            region,
            looping,
            priority,
            calendar_filter_type,
            holiday_days,
            durations,
            flags,
            texture_file_name,
        })
    }

}

impl SendCalendarHoliday {
    pub(crate) fn size(&self) -> usize {
        4 // holiday_id: u32
        + 4 // region: u32
        + 4 // looping: u32
        + 4 // priority: u32
        + 4 // calendar_filter_type: u32
        + 104 // holiday_days: u32[26]
        + 40 // durations: u32[10]
        + 40 // flags: u32[10]
        + self.texture_file_name.len() + 1 // texture_file_name: CString
    }
}

