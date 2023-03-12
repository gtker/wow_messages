use std::io::{Read, Write};

use crate::vanilla::Object;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm:189`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm#L189):
/// ```text
/// smsg SMSG_UPDATE_OBJECT = 0x00A9 {
///     u32 amount_of_objects;
///     u8 has_transport;
///     Object[amount_of_objects] objects;
/// }
/// ```
pub struct SMSG_UPDATE_OBJECT {
    pub has_transport: u8,
    pub objects: Vec<Object>,
}

impl crate::Message for SMSG_UPDATE_OBJECT {
    const OPCODE: u32 = 0x00a9;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // amount_of_objects: u32
        w.write_all(&(self.objects.len() as u32).to_le_bytes())?;

        // has_transport: u8
        w.write_all(&self.has_transport.to_le_bytes())?;

        // objects: Object[amount_of_objects]
        for i in self.objects.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00A9, size: body_size as u32 });
        }

        // amount_of_objects: u32
        let amount_of_objects = crate::util::read_u32_le(&mut r)?;

        // has_transport: u8
        let has_transport = crate::util::read_u8_le(&mut r)?;

        // objects: Object[amount_of_objects]
        let objects = {
            let mut objects = Vec::with_capacity(amount_of_objects as usize);
            for i in 0..amount_of_objects {
                objects.push(Object::read(&mut r)?);
            }
            objects
        };

        Ok(Self {
            has_transport,
            objects,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_UPDATE_OBJECT {}

impl SMSG_UPDATE_OBJECT {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_objects: u32
        + 1 // has_transport: u8
        + self.objects.iter().fold(0, |acc, x| acc + x.size()) // objects: Object[amount_of_objects]
    }
}

#[cfg(test)]
mod test {
    use super::SMSG_UPDATE_OBJECT;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 99] = [ 0x00, 0x61, 0xA9, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
         0x03, 0x01, 0x04, 0x04, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0xCD, 0xD7, 0x0B, 0xC6, 0x35, 0x7E, 0x04, 0xC3, 0xF9, 0x0F, 0xA7,
         0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
         0x3F, 0x00, 0x00, 0xE0, 0x40, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0xDB, 0x0F, 0x49, 0x40, 0x01, 0x00, 0x00,
         0x00, 0x02, 0x07, 0x00, 0x40, 0x00, 0x10, 0x00, 0x00, 0x00, 0x04, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x64, 0x00,
         0x00, 0x00, 0x01, 0x01, 0x01, 0x01, ];

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 195.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_UPDATE_OBJECT0() {
        let expected = SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CreateObject2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::builder()
                            .set_object_GUID(Guid::new(4))
                            .set_unit_HEALTH(100)
                            .set_unit_BYTES_0(1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap())
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_LIVING(MovementBlock_UpdateFlag_Living::Living {
                                    backwards_running_speed: 4.5_f32,
                                    backwards_swimming_speed: 0_f32,
                                    fall_time: 0_f32,
                                    flags: MovementBlock_MovementFlags::empty()
                                        ,
                                    living_orientation: 0_f32,
                                    living_position: Vector3d {
                                        x: -8949.95_f32,
                                        y: -132.493_f32,
                                        z: 83.5312_f32,
                                    },
                                    running_speed: 7_f32,
                                    swimming_speed: 0_f32,
                                    timestamp: 0x0,
                                    turn_rate: 3.1415927_f32,
                                    walking_speed: 1_f32,
                                })
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_SELF()
                                ,
                        },
                        object_type: ObjectType::Player,
                    },
                },
            ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.has_transport, expected.has_transport);
        assert_eq!(t.objects, expected.objects);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 195.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_UPDATE_OBJECT0() {
        let expected = SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CreateObject2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::builder()
                            .set_object_GUID(Guid::new(4))
                            .set_unit_HEALTH(100)
                            .set_unit_BYTES_0(1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap())
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_LIVING(MovementBlock_UpdateFlag_Living::Living {
                                    backwards_running_speed: 4.5_f32,
                                    backwards_swimming_speed: 0_f32,
                                    fall_time: 0_f32,
                                    flags: MovementBlock_MovementFlags::empty()
                                        ,
                                    living_orientation: 0_f32,
                                    living_position: Vector3d {
                                        x: -8949.95_f32,
                                        y: -132.493_f32,
                                        z: 83.5312_f32,
                                    },
                                    running_speed: 7_f32,
                                    swimming_speed: 0_f32,
                                    timestamp: 0x0,
                                    turn_rate: 3.1415927_f32,
                                    walking_speed: 1_f32,
                                })
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_SELF()
                                ,
                        },
                        object_type: ObjectType::Player,
                    },
                },
            ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.has_transport, expected.has_transport);
        assert_eq!(t.objects, expected.objects);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 195.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_UPDATE_OBJECT0() {
        let expected = SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CreateObject2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::builder()
                            .set_object_GUID(Guid::new(4))
                            .set_unit_HEALTH(100)
                            .set_unit_BYTES_0(1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap())
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_LIVING(MovementBlock_UpdateFlag_Living::Living {
                                    backwards_running_speed: 4.5_f32,
                                    backwards_swimming_speed: 0_f32,
                                    fall_time: 0_f32,
                                    flags: MovementBlock_MovementFlags::empty()
                                        ,
                                    living_orientation: 0_f32,
                                    living_position: Vector3d {
                                        x: -8949.95_f32,
                                        y: -132.493_f32,
                                        z: 83.5312_f32,
                                    },
                                    running_speed: 7_f32,
                                    swimming_speed: 0_f32,
                                    timestamp: 0x0,
                                    turn_rate: 3.1415927_f32,
                                    walking_speed: 1_f32,
                                })
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_SELF()
                                ,
                        },
                        object_type: ObjectType::Player,
                    },
                },
            ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.has_transport, expected.has_transport);
        assert_eq!(t.objects, expected.objects);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 135] = [ 0x00, 0x85, 0xA9, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
         0x03, 0x01, 0x04, 0x04, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0xCD, 0xD7, 0x0B, 0xC6, 0x35, 0x7E, 0x04, 0xC3, 0xF9, 0x0F, 0xA7,
         0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
         0x3F, 0x00, 0x00, 0xE0, 0x40, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0xDB, 0x0F, 0x49, 0x40, 0x01, 0x00, 0x00,
         0x00, 0x05, 0x17, 0x00, 0x40, 0x10, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x04, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x80, 0x3F, 0x64, 0x00, 0x00, 0x00, 0x64, 0x00, 0x00, 0x00, 0x01, 0x00,
         0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x32, 0x00,
         0x00, 0x00, 0x32, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 262.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_UPDATE_OBJECT1() {
        let expected = SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CreateObject2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::builder()
                            .set_object_GUID(Guid::new(4))
                            .set_object_SCALE_X(1.0)
                            .set_unit_HEALTH(100)
                            .set_unit_MAXHEALTH(100)
                            .set_unit_LEVEL(1)
                            .set_unit_FACTIONTEMPLATE(1)
                            .set_unit_BYTES_0(1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap())
                            .set_unit_DISPLAYID(50)
                            .set_unit_NATIVEDISPLAYID(50)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_LIVING(MovementBlock_UpdateFlag_Living::Living {
                                    backwards_running_speed: 4.5_f32,
                                    backwards_swimming_speed: 0_f32,
                                    fall_time: 0_f32,
                                    flags: MovementBlock_MovementFlags::empty()
                                        ,
                                    living_orientation: 0_f32,
                                    living_position: Vector3d {
                                        x: -8949.95_f32,
                                        y: -132.493_f32,
                                        z: 83.5312_f32,
                                    },
                                    running_speed: 7_f32,
                                    swimming_speed: 0_f32,
                                    timestamp: 0x0,
                                    turn_rate: 3.1415927_f32,
                                    walking_speed: 1_f32,
                                })
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_SELF()
                                ,
                        },
                        object_type: ObjectType::Player,
                    },
                },
            ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.has_transport, expected.has_transport);
        assert_eq!(t.objects, expected.objects);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 262.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_UPDATE_OBJECT1() {
        let expected = SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CreateObject2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::builder()
                            .set_object_GUID(Guid::new(4))
                            .set_object_SCALE_X(1.0)
                            .set_unit_HEALTH(100)
                            .set_unit_MAXHEALTH(100)
                            .set_unit_LEVEL(1)
                            .set_unit_FACTIONTEMPLATE(1)
                            .set_unit_BYTES_0(1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap())
                            .set_unit_DISPLAYID(50)
                            .set_unit_NATIVEDISPLAYID(50)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_LIVING(MovementBlock_UpdateFlag_Living::Living {
                                    backwards_running_speed: 4.5_f32,
                                    backwards_swimming_speed: 0_f32,
                                    fall_time: 0_f32,
                                    flags: MovementBlock_MovementFlags::empty()
                                        ,
                                    living_orientation: 0_f32,
                                    living_position: Vector3d {
                                        x: -8949.95_f32,
                                        y: -132.493_f32,
                                        z: 83.5312_f32,
                                    },
                                    running_speed: 7_f32,
                                    swimming_speed: 0_f32,
                                    timestamp: 0x0,
                                    turn_rate: 3.1415927_f32,
                                    walking_speed: 1_f32,
                                })
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_SELF()
                                ,
                        },
                        object_type: ObjectType::Player,
                    },
                },
            ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.has_transport, expected.has_transport);
        assert_eq!(t.objects, expected.objects);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 262.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_UPDATE_OBJECT1() {
        let expected = SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CreateObject2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::builder()
                            .set_object_GUID(Guid::new(4))
                            .set_object_SCALE_X(1.0)
                            .set_unit_HEALTH(100)
                            .set_unit_MAXHEALTH(100)
                            .set_unit_LEVEL(1)
                            .set_unit_FACTIONTEMPLATE(1)
                            .set_unit_BYTES_0(1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap())
                            .set_unit_DISPLAYID(50)
                            .set_unit_NATIVEDISPLAYID(50)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_LIVING(MovementBlock_UpdateFlag_Living::Living {
                                    backwards_running_speed: 4.5_f32,
                                    backwards_swimming_speed: 0_f32,
                                    fall_time: 0_f32,
                                    flags: MovementBlock_MovementFlags::empty()
                                        ,
                                    living_orientation: 0_f32,
                                    living_position: Vector3d {
                                        x: -8949.95_f32,
                                        y: -132.493_f32,
                                        z: 83.5312_f32,
                                    },
                                    running_speed: 7_f32,
                                    swimming_speed: 0_f32,
                                    timestamp: 0x0,
                                    turn_rate: 3.1415927_f32,
                                    walking_speed: 1_f32,
                                })
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_SELF()
                                ,
                        },
                        object_type: ObjectType::Player,
                    },
                },
            ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.has_transport, expected.has_transport);
        assert_eq!(t.objects, expected.objects);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

