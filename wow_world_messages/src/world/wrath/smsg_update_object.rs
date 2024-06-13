use std::io::{Read, Write};

use crate::wrath::Object;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm:201`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm#L201):
/// ```text
/// smsg SMSG_UPDATE_OBJECT = 0x00A9 {
///     u32 amount_of_objects;
///     Object[amount_of_objects] objects;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_UPDATE_OBJECT {
    pub objects: Vec<Object>,
}

impl crate::private::Sealed for SMSG_UPDATE_OBJECT {}
impl SMSG_UPDATE_OBJECT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_objects: u32
        let amount_of_objects = crate::util::read_u32_le(&mut r)?;

        // objects: Object[amount_of_objects]
        let objects = {
            let mut objects = Vec::with_capacity(amount_of_objects as usize);

            let allocation_size = u64::from(amount_of_objects);
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_objects {
                objects.push(Object::read(&mut r)?);
            }
            objects
        };

        Ok(Self {
            objects,
        })
    }

}

impl crate::Message for SMSG_UPDATE_OBJECT {
    const OPCODE: u32 = 0x00a9;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_UPDATE_OBJECT"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_objects: u32
        w.write_all(&(self.objects.len() as u32).to_le_bytes())?;

        // objects: Object[amount_of_objects]
        for i in self.objects.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(169, "SMSG_UPDATE_OBJECT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_OBJECT {}

impl SMSG_UPDATE_OBJECT {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_objects: u32
        + self.objects.iter().fold(0, |acc, x| acc + x.size()) // objects: Object[amount_of_objects]
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_UPDATE_OBJECT;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 117] = [ 0x00, 0x73, 0xA9, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03,
         0x01, 0x08, 0x04, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0xCD, 0xD7, 0x0B, 0xC6, 0x35, 0x7E, 0x04, 0xC3, 0xF9,
         0x0F, 0xA7, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x80, 0x3F, 0x00, 0x00, 0x8C, 0x42, 0x00, 0x00, 0x90, 0x40, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0xD0, 0x0F, 0x49, 0x40, 0x00, 0x00, 0x00, 0x00, 0x03,
         0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x18, 0x00, 0x00, 0x00,
         0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00,
         0x01, 0x00, 0x00, 0x00, 0x0C, 0x4D, 0x00, 0x00, 0x0C, 0x4D, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_UPDATE_OBJECT {
        SMSG_UPDATE_OBJECT {
            objects: vec![
                Object::CreateObject2 {
                    guid3: Guid::new(0x8),
                    mask2: UpdateMask::Player(UpdatePlayer::builder()
                        .set_object_guid(Guid::new(8))
                        .set_unit_factiontemplate(1)
                        .set_unit_displayid(0x4D0C)
                        .set_unit_nativedisplayid(0x4D0C)
                        .finalize()
                    ),
                    movement2: MovementBlock {
                        update_flag: MovementBlock_UpdateFlag::empty()
                            .set_living(MovementBlock_UpdateFlag_Living::Living {
                                backwards_flight_speed: 0_f32,
                                backwards_running_speed: 4.5_f32,
                                backwards_swimming_speed: 0_f32,
                                fall_time: 0_f32,
                                flags: MovementBlock_MovementFlags::empty()
                                    ,
                                flight_speed: 0_f32,
                                orientation: 0_f32,
                                pitch_rate: 0_f32,
                                position: Vector3d {
                                    x: -8949.95_f32,
                                    y: -132.493_f32,
                                    z: 83.5312_f32,
                                },
                                running_speed: 70_f32,
                                swimming_speed: 0_f32,
                                timestamp: 0x0,
                                turn_rate: 3.14159_f32,
                                walking_speed: 1_f32,
                            })
                            .set_self()
                            ,
                    },
                    object_type: ObjectType::Player,
                }
,            ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm` line 206.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_update_object0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm` line 206.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_update_object0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm` line 206.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_update_object0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

