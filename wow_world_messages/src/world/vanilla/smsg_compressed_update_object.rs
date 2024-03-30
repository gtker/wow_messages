use crate::Message;
use std::io::{Read, Write};

use crate::vanilla::Object;

/// Compressed version of [`SMSG_UPDATE_OBJECT`](crate::vanilla::SMSG_UPDATE_OBJECT). Has the same fields when uncompressed
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_compressed_object.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_compressed_object.wowm#L2):
/// ```text
/// smsg SMSG_COMPRESSED_UPDATE_OBJECT = 0x01F6 {
///     u32 amount_of_objects;
///     u8 has_transport;
///     Object[amount_of_objects] objects;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_COMPRESSED_UPDATE_OBJECT {
    pub has_transport: u8,
    pub objects: Vec<Object>,
}

impl crate::private::Sealed for SMSG_COMPRESSED_UPDATE_OBJECT {}
impl SMSG_COMPRESSED_UPDATE_OBJECT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        let decompressed_size = crate::util::read_u32_le(r)?;;
        let decompressed_buffer = vec![0; decompressed_size as usize];
        let mut r = &mut flate2::read::ZlibDecoder::new_with_buf(r, decompressed_buffer);
        let mut buf = Vec::with_capacity(decompressed_size as usize);
        r.read_to_end(&mut buf).unwrap();
        let mut r = &buf[..];

        // amount_of_objects: u32
        let amount_of_objects = crate::util::read_u32_le(&mut r)?;

        // has_transport: u8
        let has_transport = crate::util::read_u8_le(&mut r)?;

        // objects: Object[amount_of_objects]
        let objects = {
            let mut objects = Vec::with_capacity(amount_of_objects as usize);

            let allocation_size = u64::from(amount_of_objects);
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_objects {
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

impl crate::Message for SMSG_COMPRESSED_UPDATE_OBJECT {
    const OPCODE: u32 = 0x01f6;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_COMPRESSED_UPDATE_OBJECT"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        w.write_all(&(self.size_uncompressed() as u32).to_le_bytes())?;

        let mut w = &mut flate2::write::ZlibEncoder::new(w, flate2::Compression::fast());

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(502, "SMSG_COMPRESSED_UPDATE_OBJECT", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_COMPRESSED_UPDATE_OBJECT {
    #[cfg(feature = "sync")]
    fn write_unencrypted_server<W: Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(1024);
        let mut s = &mut v;
        crate::util::vanilla_get_unencrypted_server(&mut s, Self::OPCODE as u16, 0)?;
        self.write_into_vec(&mut s)?;
        let size_len = 2;
        let size = v.len().saturating_sub(size_len);
        let s = size.to_le_bytes();
        v[0] = s[1];
        v[1] = s[0];
        w.write_all(&v)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    fn write_encrypted_server<W: Write>(
        &self,
        mut w: W,
        e: &mut wow_srp::vanilla_header::EncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(1024);
        let mut s = &mut v;
        crate::util::vanilla_get_unencrypted_server(&mut s, Self::OPCODE as u16, 0)?;
        self.write_into_vec(&mut s)?;
        let size_len = 2;
        let size = v.len().saturating_sub(size_len) as u16;
        let header = e.encrypt_server_header(size, Self::OPCODE as u16);
        for (i, e) in header.iter().enumerate() {
            v[i] = *e;
        }
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_server<'s, 'async_trait, W>(
        &'s self,
        mut w: W,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::vanilla_get_unencrypted_server(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len);
            let s = size.to_le_bytes();
            v[0] = s[1];
            v[1] = s[0];
            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    fn tokio_write_encrypted_server<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut wow_srp::vanilla_header::EncrypterHalf,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::vanilla_get_unencrypted_server(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len) as u16;
            let header = e.encrypt_server_header(size, Self::OPCODE as u16);
            for (i, e) in header.iter().enumerate() {
                v[i] = *e;
            }
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_server<'s, 'async_trait, W>(
        &'s self,
        mut w: W,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::vanilla_get_unencrypted_server(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len);
            let s = size.to_le_bytes();
            v[0] = s[1];
            v[1] = s[0];
            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    fn astd_write_encrypted_server<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut wow_srp::vanilla_header::EncrypterHalf,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::vanilla_get_unencrypted_server(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len) as u16;
            let header = e.encrypt_server_header(size, Self::OPCODE as u16);
            for (i, e) in header.iter().enumerate() {
                v[i] = *e;
            }
            w.write_all(&v).await
        })
    }

}

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
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_COMPRESSED_UPDATE_OBJECT;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
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

    pub(crate) fn expected0() -> SMSG_COMPRESSED_UPDATE_OBJECT {
        SMSG_COMPRESSED_UPDATE_OBJECT {
            has_transport: 0x1,
            objects: vec![
                Object::CreateObject {
                    guid3: Guid::new(0x1FC0000000005148),
                    mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                        .set_object_guid(Guid::new(2287828610704232776))
                        .set_object_entry(20808)
                        .set_object_scale_x(1.0)
                        .set_gameobject_displayid(3015)
                        .set_gameobject_flags(40)
                        .set_gameobject_state(1)
                        .set_gameobject_type_id(15)
                        .set_gameobject_animprogress(100)
                        .finalize()
                    ),
                    movement2: MovementBlock {
                        update_flag: MovementBlock_UpdateFlag::empty()
                            .set_all(MovementBlock_UpdateFlag_All {
                                unknown1: 0x1,
                            })
                            .set_transport(MovementBlock_UpdateFlag_Transport {
                                transport_progress_in_ms: 0x5E4E1,
                            })
                            .set_living(MovementBlock_UpdateFlag_Living::HasPosition {
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
                }
,                Object::CreateObject {
                    guid3: Guid::new(0x1FC000000002B074),
                    mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                        .set_object_guid(Guid::new(2287828610704388212))
                        .set_object_entry(176244)
                        .set_object_scale_x(1.0)
                        .set_gameobject_displayid(3015)
                        .set_gameobject_flags(40)
                        .set_gameobject_state(1)
                        .set_gameobject_type_id(15)
                        .set_gameobject_animprogress(100)
                        .finalize()
                    ),
                    movement2: MovementBlock {
                        update_flag: MovementBlock_UpdateFlag::empty()
                            .set_all(MovementBlock_UpdateFlag_All {
                                unknown1: 0x1,
                            })
                            .set_transport(MovementBlock_UpdateFlag_Transport {
                                transport_progress_in_ms: 0x5E4E1,
                            })
                            .set_living(MovementBlock_UpdateFlag_Living::HasPosition {
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
                }
,                Object::CreateObject {
                    guid3: Guid::new(0x1FC0000000028407),
                    mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                        .set_object_guid(Guid::new(2287828610704376839))
                        .set_object_entry(164871)
                        .set_object_scale_x(1.0)
                        .set_gameobject_displayid(3031)
                        .set_gameobject_flags(40)
                        .set_gameobject_state(1)
                        .set_gameobject_type_id(15)
                        .set_gameobject_animprogress(100)
                        .finalize()
                    ),
                    movement2: MovementBlock {
                        update_flag: MovementBlock_UpdateFlag::empty()
                            .set_all(MovementBlock_UpdateFlag_All {
                                unknown1: 0x1,
                            })
                            .set_transport(MovementBlock_UpdateFlag_Transport {
                                transport_progress_in_ms: 0x5E4E1,
                            })
                            .set_living(MovementBlock_UpdateFlag_Living::HasPosition {
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
                }
,                Object::CreateObject {
                    guid3: Guid::new(0x1FC000000002B451),
                    mask2: UpdateMask::GameObject(UpdateGameObject::builder()
                        .set_object_guid(Guid::new(2287828610704389201))
                        .set_object_entry(177233)
                        .set_object_scale_x(1.0)
                        .set_gameobject_displayid(3015)
                        .set_gameobject_flags(40)
                        .set_gameobject_state(1)
                        .set_gameobject_type_id(15)
                        .set_gameobject_animprogress(100)
                        .finalize()
                    ),
                    movement2: MovementBlock {
                        update_flag: MovementBlock_UpdateFlag::empty()
                            .set_all(MovementBlock_UpdateFlag_All {
                                unknown1: 0x1,
                            })
                            .set_transport(MovementBlock_UpdateFlag_Transport {
                                transport_progress_in_ms: 0x5E4E1,
                            })
                            .set_living(MovementBlock_UpdateFlag_Living::HasPosition {
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
                }
,            ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_compressed_object.wowm` line 21.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_compressed_update_object0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_COMPRESSED_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_UPDATE_OBJECT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        let s = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&dest)).unwrap();
        let s = match s {
            ServerOpcodeMessage::SMSG_COMPRESSED_UPDATE_OBJECT(s) => s,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_UPDATE_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_compressed_object.wowm` line 21.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_compressed_update_object0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_COMPRESSED_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_UPDATE_OBJECT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        let s = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&dest)).await.unwrap();
        let s = match s {
            ServerOpcodeMessage::SMSG_COMPRESSED_UPDATE_OBJECT(s) => s,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_UPDATE_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_compressed_object.wowm` line 21.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_compressed_update_object0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_COMPRESSED_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_UPDATE_OBJECT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        let s = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&dest)).await.unwrap();
        let s = match s {
            ServerOpcodeMessage::SMSG_COMPRESSED_UPDATE_OBJECT(s) => s,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_UPDATE_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

}

