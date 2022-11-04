use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::Object;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Compressed version of [`SMSG_UPDATE_OBJECT`](crate::world::vanilla::SMSG_UPDATE_OBJECT). Has the same fields when uncompressed
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_compressed_object.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_compressed_object.wowm#L3):
/// ```text
/// smsg SMSG_COMPRESSED_UPDATE_OBJECT = 0x01F6 {
///     u32 amount_of_objects;
///     u8 has_transport;
///     Object[amount_of_objects] objects;
/// }
/// ```
pub struct SMSG_COMPRESSED_UPDATE_OBJECT {
    pub has_transport: u8,
    pub objects: Vec<Object>,
}

impl crate::Message for SMSG_COMPRESSED_UPDATE_OBJECT {
    const OPCODE: u32 = 0x01f6;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        w.write_all(&(self.size_uncompressed() as u32).to_le_bytes())?;

        let mut w = &mut flate2::write::ZlibEncoder::new(w, flate2::Compression::fast());

        // amount_of_objects: u32
        w.write_all(&(self.objects.len() as u32).to_le_bytes())?;

        // has_transport: u8
        w.write_all(&self.has_transport.to_le_bytes())?;

        // objects: Object[amount_of_objects]
        for i in self.objects.iter() {
            let mut vec = Vec::new();
            i.write_into_vec(&mut vec)?;
            w.write_all(vec.as_slice());
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01F6, size: body_size as u32 });
        }

        let decompressed_size = crate::util::read_u32_le(r)?;;
        let decompressed_buffer = vec![0; decompressed_size as usize];
        let mut r = &mut flate2::read::ZlibDecoder::new_with_buf(r, decompressed_buffer);

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
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_COMPRESSED_UPDATE_OBJECT {}

impl SMSG_COMPRESSED_UPDATE_OBJECT {
    pub(crate) fn size(&self) -> usize {
        use crate::traits::Message;

        let mut v = Vec::new();
        self.write_into_vec(&mut v);
        v.len()
    }
}

impl SMSG_COMPRESSED_UPDATE_OBJECT {
    pub(crate) fn size_uncompressed(&self) -> usize {
        4 // amount_of_objects: u32
        + 1 // has_transport: u8
        + self.objects.iter().fold(0, |acc, x| acc + x.size()) // objects: Object[amount_of_objects]
    }
}

#[cfg(test)]
mod test {
    use super::SMSG_COMPRESSED_UPDATE_OBJECT;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::world::vanilla::{UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 139] = [ 0x00, 0x89, 0xF6, 0x01, 0x3C, 0x01, 0x00, 0x00, 0x78,
         0x01, 0x63, 0x61, 0x60, 0x60, 0x60, 0x64, 0x3A, 0xEC, 0x11, 0x78, 0x40,
         0x9E, 0x35, 0x08, 0xC8, 0x86, 0x03, 0xBF, 0x33, 0x0D, 0x0E, 0x8C, 0x40,
         0xDE, 0xC3, 0x27, 0xAC, 0x0C, 0x8C, 0xF2, 0xCE, 0x0A, 0x8C, 0x1E, 0x81,
         0x20, 0xB9, 0x03, 0xF2, 0x8A, 0x40, 0x12, 0xC2, 0x6E, 0xB0, 0x3F, 0xCE,
         0xCD, 0xC0, 0xA0, 0x01, 0xE4, 0x83, 0x14, 0xF2, 0x03, 0x71, 0x0A, 0x10,
         0x33, 0x1D, 0x2F, 0xD9, 0xC0, 0x84, 0x6E, 0xDA, 0x51, 0xE1, 0x93, 0xF6,
         0x28, 0xA6, 0x01, 0xD5, 0x00, 0xD5, 0x42, 0x4C, 0x83, 0xB0, 0x71, 0x99,
         0xC6, 0xDE, 0x82, 0x61, 0xDA, 0xE7, 0x9A, 0x09, 0xA8, 0x6E, 0x03, 0xAA,
         0x81, 0x9B, 0x06, 0x61, 0x37, 0xD8, 0x5F, 0xC7, 0xEA, 0xB6, 0xC0, 0x2D,
         0x18, 0xA6, 0x2D, 0x9D, 0xB1, 0x15, 0xD5, 0x6D, 0x40, 0x35, 0x70, 0xD3,
         0x20, 0x6C, 0xEC, 0x6E, 0x03, 0x00, 0x36, 0x4C, 0x30, 0x21, ];

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_compressed_object.wowm` line 13.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_COMPRESSED_UPDATE_OBJECT0() {
        let expected = SMSG_COMPRESSED_UPDATE_OBJECT {
            has_transport: 0x1,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CreateObject {
                        guid3: Guid::new(0x1FC0000000005148),
                        mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                            .set_object_GUID(Guid::new(2287828610704232776))
                            .set_object_ENTRY(20808)
                            .set_object_SCALE_X(1.0)
                            .set_gameobject_DISPLAYID(3015)
                            .set_gameobject_FLAGS(40)
                            .set_gameobject_STATE(1)
                            .set_gameobject_TYPE_ID(15)
                            .set_gameobject_ANIMPROGRESS(100)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_TRANSPORT(MovementBlock_UpdateFlag_Transport {
                                    transport_progress_in_ms: 0x5E4E1,
                                })
                                .set_LIVING(MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation: 4.0249395_f32,
                                    position: Vector3d {
                                        x: 0_f32,
                                        y: 0_f32,
                                        z: 0_f32,
                                    },
                                })
                                ,
                        },
                        object_type: ObjectType::GameObject,
                    },
                },
                Object {
                    update_type: Object_UpdateType::CreateObject {
                        guid3: Guid::new(0x1FC000000002B074),
                        mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                            .set_object_GUID(Guid::new(2287828610704388212))
                            .set_object_ENTRY(176244)
                            .set_object_SCALE_X(1.0)
                            .set_gameobject_DISPLAYID(3015)
                            .set_gameobject_FLAGS(40)
                            .set_gameobject_STATE(1)
                            .set_gameobject_TYPE_ID(15)
                            .set_gameobject_ANIMPROGRESS(100)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_TRANSPORT(MovementBlock_UpdateFlag_Transport {
                                    transport_progress_in_ms: 0x5E4E1,
                                })
                                .set_LIVING(MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation: 1.5709158_f32,
                                    position: Vector3d {
                                        x: 0_f32,
                                        y: 0_f32,
                                        z: 0_f32,
                                    },
                                })
                                ,
                        },
                        object_type: ObjectType::GameObject,
                    },
                },
                Object {
                    update_type: Object_UpdateType::CreateObject {
                        guid3: Guid::new(0x1FC0000000028407),
                        mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                            .set_object_GUID(Guid::new(2287828610704376839))
                            .set_object_ENTRY(164871)
                            .set_object_SCALE_X(1.0)
                            .set_gameobject_DISPLAYID(3031)
                            .set_gameobject_FLAGS(40)
                            .set_gameobject_STATE(1)
                            .set_gameobject_TYPE_ID(15)
                            .set_gameobject_ANIMPROGRESS(100)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_TRANSPORT(MovementBlock_UpdateFlag_Transport {
                                    transport_progress_in_ms: 0x5E4E1,
                                })
                                .set_LIVING(MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation: 4.5152526_f32,
                                    position: Vector3d {
                                        x: 0_f32,
                                        y: 0_f32,
                                        z: 0_f32,
                                    },
                                })
                                ,
                        },
                        object_type: ObjectType::GameObject,
                    },
                },
                Object {
                    update_type: Object_UpdateType::CreateObject {
                        guid3: Guid::new(0x1FC000000002B451),
                        mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                            .set_object_GUID(Guid::new(2287828610704389201))
                            .set_object_ENTRY(177233)
                            .set_object_SCALE_X(1.0)
                            .set_gameobject_DISPLAYID(3015)
                            .set_gameobject_FLAGS(40)
                            .set_gameobject_STATE(1)
                            .set_gameobject_TYPE_ID(15)
                            .set_gameobject_ANIMPROGRESS(100)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_TRANSPORT(MovementBlock_UpdateFlag_Transport {
                                    transport_progress_in_ms: 0x5E4E1,
                                })
                                .set_LIVING(MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation: 1.4187208_f32,
                                    position: Vector3d {
                                        x: 0_f32,
                                        y: 0_f32,
                                        z: 0_f32,
                                    },
                                })
                                ,
                        },
                        object_type: ObjectType::GameObject,
                    },
                },
            ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_COMPRESSED_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_UPDATE_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.has_transport, expected.has_transport);
        assert_eq!(t.objects, expected.objects);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_compressed_object.wowm` line 13.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_COMPRESSED_UPDATE_OBJECT0() {
        let expected = SMSG_COMPRESSED_UPDATE_OBJECT {
            has_transport: 0x1,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CreateObject {
                        guid3: Guid::new(0x1FC0000000005148),
                        mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                            .set_object_GUID(Guid::new(2287828610704232776))
                            .set_object_ENTRY(20808)
                            .set_object_SCALE_X(1.0)
                            .set_gameobject_DISPLAYID(3015)
                            .set_gameobject_FLAGS(40)
                            .set_gameobject_STATE(1)
                            .set_gameobject_TYPE_ID(15)
                            .set_gameobject_ANIMPROGRESS(100)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_TRANSPORT(MovementBlock_UpdateFlag_Transport {
                                    transport_progress_in_ms: 0x5E4E1,
                                })
                                .set_LIVING(MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation: 4.0249395_f32,
                                    position: Vector3d {
                                        x: 0_f32,
                                        y: 0_f32,
                                        z: 0_f32,
                                    },
                                })
                                ,
                        },
                        object_type: ObjectType::GameObject,
                    },
                },
                Object {
                    update_type: Object_UpdateType::CreateObject {
                        guid3: Guid::new(0x1FC000000002B074),
                        mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                            .set_object_GUID(Guid::new(2287828610704388212))
                            .set_object_ENTRY(176244)
                            .set_object_SCALE_X(1.0)
                            .set_gameobject_DISPLAYID(3015)
                            .set_gameobject_FLAGS(40)
                            .set_gameobject_STATE(1)
                            .set_gameobject_TYPE_ID(15)
                            .set_gameobject_ANIMPROGRESS(100)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_TRANSPORT(MovementBlock_UpdateFlag_Transport {
                                    transport_progress_in_ms: 0x5E4E1,
                                })
                                .set_LIVING(MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation: 1.5709158_f32,
                                    position: Vector3d {
                                        x: 0_f32,
                                        y: 0_f32,
                                        z: 0_f32,
                                    },
                                })
                                ,
                        },
                        object_type: ObjectType::GameObject,
                    },
                },
                Object {
                    update_type: Object_UpdateType::CreateObject {
                        guid3: Guid::new(0x1FC0000000028407),
                        mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                            .set_object_GUID(Guid::new(2287828610704376839))
                            .set_object_ENTRY(164871)
                            .set_object_SCALE_X(1.0)
                            .set_gameobject_DISPLAYID(3031)
                            .set_gameobject_FLAGS(40)
                            .set_gameobject_STATE(1)
                            .set_gameobject_TYPE_ID(15)
                            .set_gameobject_ANIMPROGRESS(100)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_TRANSPORT(MovementBlock_UpdateFlag_Transport {
                                    transport_progress_in_ms: 0x5E4E1,
                                })
                                .set_LIVING(MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation: 4.5152526_f32,
                                    position: Vector3d {
                                        x: 0_f32,
                                        y: 0_f32,
                                        z: 0_f32,
                                    },
                                })
                                ,
                        },
                        object_type: ObjectType::GameObject,
                    },
                },
                Object {
                    update_type: Object_UpdateType::CreateObject {
                        guid3: Guid::new(0x1FC000000002B451),
                        mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                            .set_object_GUID(Guid::new(2287828610704389201))
                            .set_object_ENTRY(177233)
                            .set_object_SCALE_X(1.0)
                            .set_gameobject_DISPLAYID(3015)
                            .set_gameobject_FLAGS(40)
                            .set_gameobject_STATE(1)
                            .set_gameobject_TYPE_ID(15)
                            .set_gameobject_ANIMPROGRESS(100)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_TRANSPORT(MovementBlock_UpdateFlag_Transport {
                                    transport_progress_in_ms: 0x5E4E1,
                                })
                                .set_LIVING(MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation: 1.4187208_f32,
                                    position: Vector3d {
                                        x: 0_f32,
                                        y: 0_f32,
                                        z: 0_f32,
                                    },
                                })
                                ,
                        },
                        object_type: ObjectType::GameObject,
                    },
                },
            ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_COMPRESSED_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_UPDATE_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.has_transport, expected.has_transport);
        assert_eq!(t.objects, expected.objects);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_compressed_object.wowm` line 13.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_COMPRESSED_UPDATE_OBJECT0() {
        let expected = SMSG_COMPRESSED_UPDATE_OBJECT {
            has_transport: 0x1,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CreateObject {
                        guid3: Guid::new(0x1FC0000000005148),
                        mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                            .set_object_GUID(Guid::new(2287828610704232776))
                            .set_object_ENTRY(20808)
                            .set_object_SCALE_X(1.0)
                            .set_gameobject_DISPLAYID(3015)
                            .set_gameobject_FLAGS(40)
                            .set_gameobject_STATE(1)
                            .set_gameobject_TYPE_ID(15)
                            .set_gameobject_ANIMPROGRESS(100)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_TRANSPORT(MovementBlock_UpdateFlag_Transport {
                                    transport_progress_in_ms: 0x5E4E1,
                                })
                                .set_LIVING(MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation: 4.0249395_f32,
                                    position: Vector3d {
                                        x: 0_f32,
                                        y: 0_f32,
                                        z: 0_f32,
                                    },
                                })
                                ,
                        },
                        object_type: ObjectType::GameObject,
                    },
                },
                Object {
                    update_type: Object_UpdateType::CreateObject {
                        guid3: Guid::new(0x1FC000000002B074),
                        mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                            .set_object_GUID(Guid::new(2287828610704388212))
                            .set_object_ENTRY(176244)
                            .set_object_SCALE_X(1.0)
                            .set_gameobject_DISPLAYID(3015)
                            .set_gameobject_FLAGS(40)
                            .set_gameobject_STATE(1)
                            .set_gameobject_TYPE_ID(15)
                            .set_gameobject_ANIMPROGRESS(100)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_TRANSPORT(MovementBlock_UpdateFlag_Transport {
                                    transport_progress_in_ms: 0x5E4E1,
                                })
                                .set_LIVING(MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation: 1.5709158_f32,
                                    position: Vector3d {
                                        x: 0_f32,
                                        y: 0_f32,
                                        z: 0_f32,
                                    },
                                })
                                ,
                        },
                        object_type: ObjectType::GameObject,
                    },
                },
                Object {
                    update_type: Object_UpdateType::CreateObject {
                        guid3: Guid::new(0x1FC0000000028407),
                        mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                            .set_object_GUID(Guid::new(2287828610704376839))
                            .set_object_ENTRY(164871)
                            .set_object_SCALE_X(1.0)
                            .set_gameobject_DISPLAYID(3031)
                            .set_gameobject_FLAGS(40)
                            .set_gameobject_STATE(1)
                            .set_gameobject_TYPE_ID(15)
                            .set_gameobject_ANIMPROGRESS(100)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_TRANSPORT(MovementBlock_UpdateFlag_Transport {
                                    transport_progress_in_ms: 0x5E4E1,
                                })
                                .set_LIVING(MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation: 4.5152526_f32,
                                    position: Vector3d {
                                        x: 0_f32,
                                        y: 0_f32,
                                        z: 0_f32,
                                    },
                                })
                                ,
                        },
                        object_type: ObjectType::GameObject,
                    },
                },
                Object {
                    update_type: Object_UpdateType::CreateObject {
                        guid3: Guid::new(0x1FC000000002B451),
                        mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                            .set_object_GUID(Guid::new(2287828610704389201))
                            .set_object_ENTRY(177233)
                            .set_object_SCALE_X(1.0)
                            .set_gameobject_DISPLAYID(3015)
                            .set_gameobject_FLAGS(40)
                            .set_gameobject_STATE(1)
                            .set_gameobject_TYPE_ID(15)
                            .set_gameobject_ANIMPROGRESS(100)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_ALL(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_TRANSPORT(MovementBlock_UpdateFlag_Transport {
                                    transport_progress_in_ms: 0x5E4E1,
                                })
                                .set_LIVING(MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation: 1.4187208_f32,
                                    position: Vector3d {
                                        x: 0_f32,
                                        y: 0_f32,
                                        z: 0_f32,
                                    },
                                })
                                ,
                        },
                        object_type: ObjectType::GameObject,
                    },
                },
            ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_COMPRESSED_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_UPDATE_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.has_transport, expected.has_transport);
        assert_eq!(t.objects, expected.objects);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

