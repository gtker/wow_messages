use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};
use crate::wrath::{
    Map, SendCalendarEvent, SendCalendarHoliday, SendCalendarInstance, SendCalendarInvite, 
    SendCalendarResetTime,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_send_calendar.wowm:55`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_send_calendar.wowm#L55):
/// ```text
/// smsg SMSG_CALENDAR_SEND_CALENDAR = 0x0436 {
///     u32 amount_of_invites;
///     SendCalendarInvite[amount_of_invites] invites;
///     u32 amount_of_events;
///     SendCalendarEvent[amount_of_events] events;
///     u32 current_time;
///     DateTime zone_time;
///     u32 amount_of_instances;
///     SendCalendarInstance[amount_of_instances] instances;
///     u32 relative_time;
///     u32 amount_of_reset_times;
///     SendCalendarResetTime[amount_of_reset_times] reset_times;
///     u32 amount_of_holidays;
///     SendCalendarHoliday[amount_of_holidays] holidays;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_CALENDAR_SEND_CALENDAR {
    pub invites: Vec<SendCalendarInvite>,
    pub events: Vec<SendCalendarEvent>,
    pub current_time: u32,
    pub zone_time: DateTime,
    pub instances: Vec<SendCalendarInstance>,
    pub relative_time: u32,
    pub reset_times: Vec<SendCalendarResetTime>,
    pub holidays: Vec<SendCalendarHoliday>,
}

impl crate::private::Sealed for SMSG_CALENDAR_SEND_CALENDAR {}
impl SMSG_CALENDAR_SEND_CALENDAR {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(32..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_invites: u32
        let amount_of_invites = crate::util::read_u32_le(&mut r)?;

        // invites: SendCalendarInvite[amount_of_invites]
        let invites = {
            let mut invites = Vec::with_capacity(amount_of_invites as usize);

            let allocation_size = u64::from(amount_of_invites) * 20;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_invites {
                invites.push(SendCalendarInvite::read(&mut r)?);
            }
            invites
        };

        // amount_of_events: u32
        let amount_of_events = crate::util::read_u32_le(&mut r)?;

        // events: SendCalendarEvent[amount_of_events]
        let events = {
            let mut events = Vec::with_capacity(amount_of_events as usize);

            let allocation_size = u64::from(amount_of_events) * 26;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_events {
                events.push(SendCalendarEvent::read(&mut r)?);
            }
            events
        };

        // current_time: u32
        let current_time = crate::util::read_u32_le(&mut r)?;

        // zone_time: DateTime
        let zone_time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // amount_of_instances: u32
        let amount_of_instances = crate::util::read_u32_le(&mut r)?;

        // instances: SendCalendarInstance[amount_of_instances]
        let instances = {
            let mut instances = Vec::with_capacity(amount_of_instances as usize);

            let allocation_size = u64::from(amount_of_instances) * 20;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_instances {
                instances.push(SendCalendarInstance::read(&mut r)?);
            }
            instances
        };

        // relative_time: u32
        let relative_time = crate::util::read_u32_le(&mut r)?;

        // amount_of_reset_times: u32
        let amount_of_reset_times = crate::util::read_u32_le(&mut r)?;

        // reset_times: SendCalendarResetTime[amount_of_reset_times]
        let reset_times = {
            let mut reset_times = Vec::with_capacity(amount_of_reset_times as usize);

            let allocation_size = u64::from(amount_of_reset_times) * 12;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_reset_times {
                reset_times.push(SendCalendarResetTime::read(&mut r)?);
            }
            reset_times
        };

        // amount_of_holidays: u32
        let amount_of_holidays = crate::util::read_u32_le(&mut r)?;

        // holidays: SendCalendarHoliday[amount_of_holidays]
        let holidays = {
            let mut holidays = Vec::with_capacity(amount_of_holidays as usize);

            let allocation_size = u64::from(amount_of_holidays) * 205;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_holidays {
                holidays.push(SendCalendarHoliday::read(&mut r)?);
            }
            holidays
        };

        Ok(Self {
            invites,
            events,
            current_time,
            zone_time,
            instances,
            relative_time,
            reset_times,
            holidays,
        })
    }

}

impl crate::Message for SMSG_CALENDAR_SEND_CALENDAR {
    const OPCODE: u32 = 0x0436;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CALENDAR_SEND_CALENDAR"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CALENDAR_SEND_CALENDAR {{").unwrap();
        // Members
        writeln!(s, "    amount_of_invites = {};", self.invites.len()).unwrap();
        writeln!(s, "    invites = [").unwrap();
        for v in self.invites.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            event_id = {};", v.event_id.guid()).unwrap();
            writeln!(s, "            invite_id = {};", v.invite_id.guid()).unwrap();
            writeln!(s, "            status = {};", v.status).unwrap();
            writeln!(s, "            rank = {};", v.rank).unwrap();
            writeln!(s, "            is_guild_event = {};", if v.is_guild_event { "TRUE" } else { "FALSE" }).unwrap();
            writeln!(s, "            creator = {};", v.creator.guid()).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    amount_of_events = {};", self.events.len()).unwrap();
        writeln!(s, "    events = [").unwrap();
        for v in self.events.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            event_id = {};", v.event_id.guid()).unwrap();
            writeln!(s, "            title = \"{}\";", v.title).unwrap();
            writeln!(s, "            event_type = {};", v.event_type).unwrap();
            writeln!(s, "            event_time = {};", v.event_time.as_int()).unwrap();
            writeln!(s, "            flags = {};", v.flags).unwrap();
            writeln!(s, "            dungeon_id = {};", v.dungeon_id).unwrap();
            writeln!(s, "            creator = {};", v.creator.guid()).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    current_time = {};", self.current_time).unwrap();
        writeln!(s, "    zone_time = {};", self.zone_time.as_int()).unwrap();
        writeln!(s, "    amount_of_instances = {};", self.instances.len()).unwrap();
        writeln!(s, "    instances = [").unwrap();
        for v in self.instances.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            map = {};", v.map.as_test_case_value()).unwrap();
            writeln!(s, "            difficulty = {};", v.difficulty).unwrap();
            writeln!(s, "            reset_time = {};", v.reset_time).unwrap();
            writeln!(s, "            instance_id = {};", v.instance_id.guid()).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    relative_time = {};", self.relative_time).unwrap();
        writeln!(s, "    amount_of_reset_times = {};", self.reset_times.len()).unwrap();
        writeln!(s, "    reset_times = [").unwrap();
        for v in self.reset_times.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            map = {};", v.map.as_test_case_value()).unwrap();
            writeln!(s, "            period = {};", v.period).unwrap();
            writeln!(s, "            time_offset = {};", v.time_offset).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    amount_of_holidays = {};", self.holidays.len()).unwrap();
        writeln!(s, "    holidays = [").unwrap();
        for v in self.holidays.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            holiday_id = {};", v.holiday_id).unwrap();
            writeln!(s, "            region = {};", v.region).unwrap();
            writeln!(s, "            looping = {};", v.looping).unwrap();
            writeln!(s, "            priority = {};", v.priority).unwrap();
            writeln!(s, "            calendar_filter_type = {};", v.calendar_filter_type).unwrap();
            writeln!(s, "            holiday_days = [").unwrap();
            for v in v.holiday_days.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "            ];").unwrap();
            writeln!(s, "            durations = [").unwrap();
            for v in v.durations.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "            ];").unwrap();
            writeln!(s, "            flags = [").unwrap();
            for v in v.flags.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "            ];").unwrap();
            writeln!(s, "            texture_file_name = \"{}\";", v.texture_file_name).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1078_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_invites", "    ");
        if !self.invites.is_empty() {
            writeln!(s, "    /* invites: SendCalendarInvite[amount_of_invites] start */").unwrap();
            for (i, v) in self.invites.iter().enumerate() {
                writeln!(s, "    /* invites: SendCalendarInvite[amount_of_invites] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "event_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "invite_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "rank", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "is_guild_event", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&v.creator), "creator", "        ");
                writeln!(s, "    /* invites: SendCalendarInvite[amount_of_invites] {i} end */").unwrap();
            }
            writeln!(s, "    /* invites: SendCalendarInvite[amount_of_invites] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_events", "    ");
        if !self.events.is_empty() {
            writeln!(s, "    /* events: SendCalendarEvent[amount_of_events] start */").unwrap();
            for (i, v) in self.events.iter().enumerate() {
                writeln!(s, "    /* events: SendCalendarEvent[amount_of_events] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "event_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.title.len() + 1, "title", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "event_type", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "event_time", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&v.creator), "creator", "        ");
                writeln!(s, "    /* events: SendCalendarEvent[amount_of_events] {i} end */").unwrap();
            }
            writeln!(s, "    /* events: SendCalendarEvent[amount_of_events] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "current_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "zone_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_instances", "    ");
        if !self.instances.is_empty() {
            writeln!(s, "    /* instances: SendCalendarInstance[amount_of_instances] start */").unwrap();
            for (i, v) in self.instances.iter().enumerate() {
                writeln!(s, "    /* instances: SendCalendarInstance[amount_of_instances] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "difficulty", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "reset_time", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "instance_id", "        ");
                writeln!(s, "    /* instances: SendCalendarInstance[amount_of_instances] {i} end */").unwrap();
            }
            writeln!(s, "    /* instances: SendCalendarInstance[amount_of_instances] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "relative_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_reset_times", "    ");
        if !self.reset_times.is_empty() {
            writeln!(s, "    /* reset_times: SendCalendarResetTime[amount_of_reset_times] start */").unwrap();
            for (i, v) in self.reset_times.iter().enumerate() {
                writeln!(s, "    /* reset_times: SendCalendarResetTime[amount_of_reset_times] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "period", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "time_offset", "        ");
                writeln!(s, "    /* reset_times: SendCalendarResetTime[amount_of_reset_times] {i} end */").unwrap();
            }
            writeln!(s, "    /* reset_times: SendCalendarResetTime[amount_of_reset_times] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_holidays", "    ");
        if !self.holidays.is_empty() {
            writeln!(s, "    /* holidays: SendCalendarHoliday[amount_of_holidays] start */").unwrap();
            for (i, v) in self.holidays.iter().enumerate() {
                writeln!(s, "    /* holidays: SendCalendarHoliday[amount_of_holidays] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "holiday_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "region", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "looping", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "priority", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "calendar_filter_type", "        ");
                writeln!(s, "    /* holiday_days: u32[26] start */").unwrap();
                for (i, v) in v.holiday_days.iter().enumerate() {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("holiday_days {i}"), "        ");
                }
                writeln!(s, "    /* holiday_days: u32[26] end */").unwrap();
                writeln!(s, "    /* durations: u32[10] start */").unwrap();
                for (i, v) in v.durations.iter().enumerate() {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("durations {i}"), "        ");
                }
                writeln!(s, "    /* durations: u32[10] end */").unwrap();
                writeln!(s, "    /* flags: u32[10] start */").unwrap();
                for (i, v) in v.flags.iter().enumerate() {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("flags {i}"), "        ");
                }
                writeln!(s, "    /* flags: u32[10] end */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, v.texture_file_name.len() + 1, "texture_file_name", "        ");
                writeln!(s, "    /* holidays: SendCalendarHoliday[amount_of_holidays] {i} end */").unwrap();
            }
            writeln!(s, "    /* holidays: SendCalendarHoliday[amount_of_holidays] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_invites: u32
        w.write_all(&(self.invites.len() as u32).to_le_bytes())?;

        // invites: SendCalendarInvite[amount_of_invites]
        for i in self.invites.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_events: u32
        w.write_all(&(self.events.len() as u32).to_le_bytes())?;

        // events: SendCalendarEvent[amount_of_events]
        for i in self.events.iter() {
            i.write_into_vec(&mut w)?;
        }

        // current_time: u32
        w.write_all(&self.current_time.to_le_bytes())?;

        // zone_time: DateTime
        w.write_all(&self.zone_time.as_int().to_le_bytes())?;

        // amount_of_instances: u32
        w.write_all(&(self.instances.len() as u32).to_le_bytes())?;

        // instances: SendCalendarInstance[amount_of_instances]
        for i in self.instances.iter() {
            i.write_into_vec(&mut w)?;
        }

        // relative_time: u32
        w.write_all(&self.relative_time.to_le_bytes())?;

        // amount_of_reset_times: u32
        w.write_all(&(self.reset_times.len() as u32).to_le_bytes())?;

        // reset_times: SendCalendarResetTime[amount_of_reset_times]
        for i in self.reset_times.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_holidays: u32
        w.write_all(&(self.holidays.len() as u32).to_le_bytes())?;

        // holidays: SendCalendarHoliday[amount_of_holidays]
        for i in self.holidays.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1078, "SMSG_CALENDAR_SEND_CALENDAR", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_SEND_CALENDAR {}

impl SMSG_CALENDAR_SEND_CALENDAR {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_invites: u32
        + self.invites.iter().fold(0, |acc, x| acc + x.size()) // invites: SendCalendarInvite[amount_of_invites]
        + 4 // amount_of_events: u32
        + self.events.iter().fold(0, |acc, x| acc + x.size()) // events: SendCalendarEvent[amount_of_events]
        + 4 // current_time: u32
        + 4 // zone_time: DateTime
        + 4 // amount_of_instances: u32
        + self.instances.len() * 20 // instances: SendCalendarInstance[amount_of_instances]
        + 4 // relative_time: u32
        + 4 // amount_of_reset_times: u32
        + self.reset_times.len() * 12 // reset_times: SendCalendarResetTime[amount_of_reset_times]
        + 4 // amount_of_holidays: u32
        + self.holidays.iter().fold(0, |acc, x| acc + x.size()) // holidays: SendCalendarHoliday[amount_of_holidays]
    }
}

