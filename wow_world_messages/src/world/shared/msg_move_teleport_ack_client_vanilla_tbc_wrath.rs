use std::io::{Read, Write};

use crate::Guid;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Response to [`MSG_MOVE_TELEPORT_ACK_Server`](crate::vanilla::MSG_MOVE_TELEPORT_ACK_Server), at which point [`MSG_MOVE_TELEPORT_ACK_Server`](crate::vanilla::MSG_MOVE_TELEPORT_ACK_Server) should be sent to observing players.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm#L1):
/// ```text
/// cmsg MSG_MOVE_TELEPORT_ACK_Client = 0x00C7 {
///     PackedGuid guid;
///     u32 movement_counter;
///     Milliseconds time;
/// }
/// ```
pub struct MSG_MOVE_TELEPORT_ACK_Client {
    pub guid: Guid,
    pub movement_counter: u32,
    pub time: Duration,
}

#[cfg(feature = "print-testcase")]
impl MSG_MOVE_TELEPORT_ACK_Client {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_MOVE_TELEPORT_ACK_Client {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    movement_counter = {};", self.movement_counter).unwrap();
        writeln!(s, "    time = {};", self.time.as_millis()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 199_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.guid), "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "movement_counter", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "time", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for MSG_MOVE_TELEPORT_ACK_Client {}
impl crate::Message for MSG_MOVE_TELEPORT_ACK_Client {
    const OPCODE: u32 = 0x00c7;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        MSG_MOVE_TELEPORT_ACK_Client::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // time: Milliseconds
        w.write_all((self.time.as_millis() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(10..=17).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00C7, size: body_size });
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        // time: Milliseconds
        let time = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            guid,
            movement_counter,
            time,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_MOVE_TELEPORT_ACK_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_MOVE_TELEPORT_ACK_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_MOVE_TELEPORT_ACK_Client {}

impl MSG_MOVE_TELEPORT_ACK_Client {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // movement_counter: u32
        + 4 // time: Milliseconds
    }
}

