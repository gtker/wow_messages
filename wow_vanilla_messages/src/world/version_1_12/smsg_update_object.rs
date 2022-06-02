use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Object;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm:186`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm#L186):
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

impl ServerMessage for SMSG_UPDATE_OBJECT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_objects: u32
        w.write_all(&(self.objects.len() as u32).to_le_bytes())?;

        // has_transport: u8
        w.write_all(&self.has_transport.to_le_bytes())?;

        // objects: Object[amount_of_objects]
        for i in self.objects.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x00a9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // amount_of_objects: u32
        let amount_of_objects = crate::util::read_u32_le(r)?;

        // has_transport: u8
        let has_transport = crate::util::read_u8_le(r)?;

        // objects: Object[amount_of_objects]
        let mut objects = Vec::with_capacity(amount_of_objects as usize);
        for i in 0..amount_of_objects {
            objects.push(Object::read(r)?);
        }

        Ok(Self {
            has_transport,
            objects,
        })
    }

}

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
    use crate::world::version_1_12::MovementBlock;
    use crate::world::version_1_12::MovementFlags;
    use crate::world::version_1_12::Object;
    use crate::world::version_1_12::ObjectType;
    use crate::world::version_1_12::SplineFlag;
    use crate::world::version_1_12::TransportInfo;
    use crate::world::version_1_12::UpdateFlag;
    use crate::world::version_1_12::UpdateType;
    use crate::world::version_1_12::Vector3d;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ServerOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 99] = [ 0x00, 0x61, 0xA9, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
         0x03, 0x01, 0x04, 0x04, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0xCD, 0xD7, 0x0B, 0xC6, 0x35, 0x7E, 0x04, 0xC3, 0xF9, 0x0F, 0xA7,
         0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
         0x3F, 0x00, 0x00, 0xE0, 0x40, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0xDB, 0x0F, 0x49, 0x40, 0x01, 0x00, 0x00,
         0x00, 0x02, 0x07, 0x00, 0x40, 0x00, 0x10, 0x00, 0x00, 0x00, 0x04, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x64, 0x00,
         0x00, 0x00, 0x01, 0x01, 0x01, 0x01, ];

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 192.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_UPDATE_OBJECT0() {
        let expected = SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CREATE_OBJECT2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::new()
                            .set_object_GUID(Guid::new(4))
                            .set_unit_HEALTH(100)
                            .set_unit_BYTES_0(1, 1, 1, 1)
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_LIVING(MovementBlock_UpdateFlag_LIVING::LIVING {
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
                                .set_ALL(MovementBlock_UpdateFlag_ALL {
                                    unknown1: 0x1,
                                })
                                .set_SELF()
                                ,
                        },
                        object_type: ObjectType::PLAYER,
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

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 192.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_UPDATE_OBJECT0() {
        let expected = SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CREATE_OBJECT2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::new()
                            .set_object_GUID(Guid::new(4))
                            .set_unit_HEALTH(100)
                            .set_unit_BYTES_0(1, 1, 1, 1)
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_LIVING(MovementBlock_UpdateFlag_LIVING::LIVING {
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
                                .set_ALL(MovementBlock_UpdateFlag_ALL {
                                    unknown1: 0x1,
                                })
                                .set_SELF()
                                ,
                        },
                        object_type: ObjectType::PLAYER,
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

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 192.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_UPDATE_OBJECT0() {
        let expected = SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CREATE_OBJECT2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::new()
                            .set_object_GUID(Guid::new(4))
                            .set_unit_HEALTH(100)
                            .set_unit_BYTES_0(1, 1, 1, 1)
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_LIVING(MovementBlock_UpdateFlag_LIVING::LIVING {
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
                                .set_ALL(MovementBlock_UpdateFlag_ALL {
                                    unknown1: 0x1,
                                })
                                .set_SELF()
                                ,
                        },
                        object_type: ObjectType::PLAYER,
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

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 259.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_UPDATE_OBJECT1() {
        let expected = SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CREATE_OBJECT2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::new()
                            .set_object_GUID(Guid::new(4))
                            .set_object_SCALE_X(1.0)
                            .set_unit_HEALTH(100)
                            .set_unit_MAXHEALTH(100)
                            .set_unit_LEVEL(1)
                            .set_unit_FACTIONTEMPLATE(1)
                            .set_unit_BYTES_0(1, 1, 1, 1)
                            .set_unit_DISPLAYID(50)
                            .set_unit_NATIVEDISPLAYID(50)
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_LIVING(MovementBlock_UpdateFlag_LIVING::LIVING {
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
                                .set_ALL(MovementBlock_UpdateFlag_ALL {
                                    unknown1: 0x1,
                                })
                                .set_SELF()
                                ,
                        },
                        object_type: ObjectType::PLAYER,
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

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 259.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_UPDATE_OBJECT1() {
        let expected = SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CREATE_OBJECT2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::new()
                            .set_object_GUID(Guid::new(4))
                            .set_object_SCALE_X(1.0)
                            .set_unit_HEALTH(100)
                            .set_unit_MAXHEALTH(100)
                            .set_unit_LEVEL(1)
                            .set_unit_FACTIONTEMPLATE(1)
                            .set_unit_BYTES_0(1, 1, 1, 1)
                            .set_unit_DISPLAYID(50)
                            .set_unit_NATIVEDISPLAYID(50)
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_LIVING(MovementBlock_UpdateFlag_LIVING::LIVING {
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
                                .set_ALL(MovementBlock_UpdateFlag_ALL {
                                    unknown1: 0x1,
                                })
                                .set_SELF()
                                ,
                        },
                        object_type: ObjectType::PLAYER,
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

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 259.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_UPDATE_OBJECT1() {
        let expected = SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CREATE_OBJECT2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::new()
                            .set_object_GUID(Guid::new(4))
                            .set_object_SCALE_X(1.0)
                            .set_unit_HEALTH(100)
                            .set_unit_MAXHEALTH(100)
                            .set_unit_LEVEL(1)
                            .set_unit_FACTIONTEMPLATE(1)
                            .set_unit_BYTES_0(1, 1, 1, 1)
                            .set_unit_DISPLAYID(50)
                            .set_unit_NATIVEDISPLAYID(50)
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_LIVING(MovementBlock_UpdateFlag_LIVING::LIVING {
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
                                .set_ALL(MovementBlock_UpdateFlag_ALL {
                                    unknown1: 0x1,
                                })
                                .set_SELF()
                                ,
                        },
                        object_type: ObjectType::PLAYER,
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
