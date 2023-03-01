use crate:: {
    Guid,
};
use crate::vanilla::MovementInfo;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Sent to acknowledge the new speed. Reply to [`SMSG_FORCE_RUN_SPEED_CHANGE`](crate::vanilla::SMSG_FORCE_RUN_SPEED_CHANGE).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_force_run_speed_change_ack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_force_run_speed_change_ack.wowm#L1):
/// ```text
/// cmsg CMSG_FORCE_RUN_SPEED_CHANGE_ACK = 0x00E3 {
///     Guid guid;
///     u32 counter;
///     MovementInfo info;
///     f32 new_speed;
/// }
/// ```
pub struct CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub info: MovementInfo,
    pub new_speed: f32,
}

impl crate::Message for CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
    const OPCODE: u32 = 0x00e3;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(44..=97).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00E3, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // new_speed: f32
        let new_speed = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            counter,
            info,
            new_speed,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_FORCE_RUN_SPEED_CHANGE_ACK {}

impl CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // new_speed: f32
    }
}

#[cfg(test)]
mod test {
    use super::CMSG_FORCE_RUN_SPEED_CHANGE_ACK;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 50] = [ 0x00, 0x30, 0xE3, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x40, 0x17, 0xF6, 0x01, 0xCB, 0xAB, 0x0B, 0xC6, 0x07, 0x86, 0xF8,
         0xC2, 0x8E, 0xD1, 0xA5, 0x42, 0xED, 0x99, 0x7F, 0x40, 0x39, 0x03, 0x00,
         0x00, 0x00, 0x00, 0xE0, 0x40, ];

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_force_run_speed_change_ack.wowm` line 11.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_FORCE_RUN_SPEED_CHANGE_ACK0() {
        let expected = CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
            guid: Guid::new(0x6),
            counter: 0x0,
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    ,
                timestamp: 0x1F61740,
                position: Vector3d {
                    x: -8938.948_f32,
                    y: -124.26177_f32,
                    z: 82.90929_f32,
                },
                orientation: 3.99377_f32,
                fall_time: 0.000000000000000000000000000000000000000001156_f32,
            },
            new_speed: 7_f32,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_FORCE_RUN_SPEED_CHANGE_ACK, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.counter, expected.counter);
        assert_eq!(t.info, expected.info);
        assert_eq!(t.new_speed, expected.new_speed);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_force_run_speed_change_ack.wowm` line 11.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_FORCE_RUN_SPEED_CHANGE_ACK0() {
        let expected = CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
            guid: Guid::new(0x6),
            counter: 0x0,
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    ,
                timestamp: 0x1F61740,
                position: Vector3d {
                    x: -8938.948_f32,
                    y: -124.26177_f32,
                    z: 82.90929_f32,
                },
                orientation: 3.99377_f32,
                fall_time: 0.000000000000000000000000000000000000000001156_f32,
            },
            new_speed: 7_f32,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_FORCE_RUN_SPEED_CHANGE_ACK, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.counter, expected.counter);
        assert_eq!(t.info, expected.info);
        assert_eq!(t.new_speed, expected.new_speed);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_force_run_speed_change_ack.wowm` line 11.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_FORCE_RUN_SPEED_CHANGE_ACK0() {
        let expected = CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
            guid: Guid::new(0x6),
            counter: 0x0,
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    ,
                timestamp: 0x1F61740,
                position: Vector3d {
                    x: -8938.948_f32,
                    y: -124.26177_f32,
                    z: 82.90929_f32,
                },
                orientation: 3.99377_f32,
                fall_time: 0.000000000000000000000000000000000000000001156_f32,
            },
            new_speed: 7_f32,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_FORCE_RUN_SPEED_CHANGE_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_FORCE_RUN_SPEED_CHANGE_ACK, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.counter, expected.counter);
        assert_eq!(t.info, expected.info);
        assert_eq!(t.new_speed, expected.new_speed);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

