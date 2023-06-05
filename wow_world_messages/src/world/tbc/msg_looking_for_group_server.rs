use std::io::{Read, Write};

use crate::tbc::{
    LfgPlayer, LfgType,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm:50`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm#L50):
/// ```text
/// smsg MSG_LOOKING_FOR_GROUP_Server = 0x01FF {
///     (u32)LfgType lfg_type;
///     u32 entry;
///     u32 amount_of_players_displayed;
///     u32 amount_of_players_found;
///     LfgPlayer[amount_of_players_displayed] players_displayed;
/// }
/// ```
pub struct MSG_LOOKING_FOR_GROUP_Server {
    pub lfg_type: LfgType,
    /// entry from LfgDunggeons.dbc
    ///
    pub entry: u32,
    pub amount_of_players_found: u32,
    pub players_displayed: Vec<LfgPlayer>,
}

#[cfg(feature = "print-testcase")]
impl MSG_LOOKING_FOR_GROUP_Server {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_LOOKING_FOR_GROUP_Server {{").unwrap();
        // Members
        writeln!(s, "    lfg_type = {};", self.lfg_type.as_test_case_value()).unwrap();
        writeln!(s, "    entry = {};", self.entry).unwrap();
        writeln!(s, "    amount_of_players_displayed = {};", self.players_displayed.len()).unwrap();
        writeln!(s, "    amount_of_players_found = {};", self.amount_of_players_found).unwrap();
        write!(s, "    players_displayed = [").unwrap();
        for v in self.players_displayed.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    guid = {};", v.guid.guid()).unwrap();
            writeln!(s, "    level = {};", v.level.as_int()).unwrap();
            writeln!(s, "    area = {};", v.area.as_test_case_value()).unwrap();
            writeln!(s, "    lfg_mode = {};", v.lfg_mode.as_test_case_value()).unwrap();
            write!(s, "    lfg_slots = [").unwrap();
            for v in v.lfg_slots.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "];").unwrap();
            writeln!(s, "    comment = \"{}\";", v.comment).unwrap();
            writeln!(s, "    amount_of_members = {};", v.members.len()).unwrap();
            write!(s, "    members = [").unwrap();
            for v in v.members.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "    guid = {};", v.guid.guid()).unwrap();
                writeln!(s, "    level = {};", v.level.as_int()).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 511_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "lfg_type");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for MSG_LOOKING_FOR_GROUP_Server {}
impl crate::Message for MSG_LOOKING_FOR_GROUP_Server {
    const OPCODE: u32 = 0x01ff;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // lfg_type: LfgType
        w.write_all(&u32::from(self.lfg_type.as_int()).to_le_bytes())?;

        // entry: u32
        w.write_all(&self.entry.to_le_bytes())?;

        // amount_of_players_displayed: u32
        w.write_all(&(self.players_displayed.len() as u32).to_le_bytes())?;

        // amount_of_players_found: u32
        w.write_all(&self.amount_of_players_found.to_le_bytes())?;

        // players_displayed: LfgPlayer[amount_of_players_displayed]
        for i in self.players_displayed.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(16..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01FF, size: body_size });
        }

        // lfg_type: LfgType
        let lfg_type = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // entry: u32
        let entry = crate::util::read_u32_le(&mut r)?;

        // amount_of_players_displayed: u32
        let amount_of_players_displayed = crate::util::read_u32_le(&mut r)?;

        // amount_of_players_found: u32
        let amount_of_players_found = crate::util::read_u32_le(&mut r)?;

        // players_displayed: LfgPlayer[amount_of_players_displayed]
        let players_displayed = {
            let mut players_displayed = Vec::with_capacity(amount_of_players_displayed as usize);
            for _ in 0..amount_of_players_displayed {
                players_displayed.push(LfgPlayer::read(&mut r)?);
            }
            players_displayed
        };

        Ok(Self {
            lfg_type,
            entry,
            amount_of_players_found,
            players_displayed,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_LOOKING_FOR_GROUP_Server {}

impl MSG_LOOKING_FOR_GROUP_Server {
    pub(crate) fn size(&self) -> usize {
        4 // lfg_type: LfgType
        + 4 // entry: u32
        + 4 // amount_of_players_displayed: u32
        + 4 // amount_of_players_found: u32
        + self.players_displayed.iter().fold(0, |acc, x| acc + x.size()) // players_displayed: LfgPlayer[amount_of_players_displayed]
    }
}

