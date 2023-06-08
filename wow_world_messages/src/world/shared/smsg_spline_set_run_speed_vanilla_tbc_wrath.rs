use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Informs the client that the run speed of a unit has changed.
/// Mangos sends this to third parties that aren't having their speed changed and [`SMSG_FORCE_RUN_SPEED_CHANGE`](crate::vanilla::SMSG_FORCE_RUN_SPEED_CHANGE) to the client that has their run speed changed.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_SET_RUN_SPEED = 0x02FE {
///     PackedGuid guid;
///     f32 speed;
/// }
/// ```
pub struct SMSG_SPLINE_SET_RUN_SPEED {
    pub guid: Guid,
    pub speed: f32,
}

impl crate::private::Sealed for SMSG_SPLINE_SET_RUN_SPEED {}
impl crate::Message for SMSG_SPLINE_SET_RUN_SPEED {
    const OPCODE: u32 = 0x02fe;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02FE, size: body_size });
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            speed,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPLINE_SET_RUN_SPEED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPLINE_SET_RUN_SPEED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPLINE_SET_RUN_SPEED {}

impl SMSG_SPLINE_SET_RUN_SPEED {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // speed: f32
    }
}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_SPLINE_SET_RUN_SPEED;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_SPLINE_SET_RUN_SPEED, expected: &SMSG_SPLINE_SET_RUN_SPEED) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.speed, expected.speed);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0xFE, 0x02, 0x01, 0x06, 0x00, 0x00, 0xE0,
         0x40, ];

    pub(crate) fn expected0() -> SMSG_SPLINE_SET_RUN_SPEED {
        SMSG_SPLINE_SET_RUN_SPEED {
            guid: Guid::new(0x6),
            speed: 7_f32,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm` line 11.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_spline_set_run_speed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPLINE_SET_RUN_SPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm` line 11.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_spline_set_run_speed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPLINE_SET_RUN_SPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm` line 11.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_spline_set_run_speed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPLINE_SET_RUN_SPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_SPLINE_SET_RUN_SPEED;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_SPLINE_SET_RUN_SPEED, expected: &SMSG_SPLINE_SET_RUN_SPEED) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.speed, expected.speed);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0xFE, 0x02, 0x01, 0x06, 0x00, 0x00, 0xE0,
         0x40, ];

    pub(crate) fn expected0() -> SMSG_SPLINE_SET_RUN_SPEED {
        SMSG_SPLINE_SET_RUN_SPEED {
            guid: Guid::new(0x6),
            speed: 7_f32,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm` line 11.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_spline_set_run_speed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPLINE_SET_RUN_SPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm` line 11.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_spline_set_run_speed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPLINE_SET_RUN_SPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm` line 11.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_spline_set_run_speed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPLINE_SET_RUN_SPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_SPLINE_SET_RUN_SPEED;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_SPLINE_SET_RUN_SPEED, expected: &SMSG_SPLINE_SET_RUN_SPEED) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.speed, expected.speed);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0xFE, 0x02, 0x01, 0x06, 0x00, 0x00, 0xE0,
         0x40, ];

    pub(crate) fn expected0() -> SMSG_SPLINE_SET_RUN_SPEED {
        SMSG_SPLINE_SET_RUN_SPEED {
            guid: Guid::new(0x6),
            speed: 7_f32,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm` line 11.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_spline_set_run_speed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPLINE_SET_RUN_SPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm` line 11.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_spline_set_run_speed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPLINE_SET_RUN_SPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_spline_run_speed.wowm` line 11.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_spline_set_run_speed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPLINE_SET_RUN_SPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPLINE_SET_RUN_SPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

