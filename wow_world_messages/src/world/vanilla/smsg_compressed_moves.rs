use crate::Message;
use std::io::{Read, Write};

use crate::vanilla::CompressedMove;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm:48`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm#L48):
/// ```text
/// smsg SMSG_COMPRESSED_MOVES = 0x02FB {
///     CompressedMove[-] moves;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_COMPRESSED_MOVES {
    pub moves: Vec<CompressedMove>,
}

impl crate::private::Sealed for SMSG_COMPRESSED_MOVES {}
impl SMSG_COMPRESSED_MOVES {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size > 65535 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        let decompressed_size = crate::util::read_u32_le(r)?;;
        let decompressed_buffer = vec![0; decompressed_size as usize];
        let mut r = &mut flate2::read::ZlibDecoder::new_with_buf(r, decompressed_buffer);
        let mut buf = Vec::with_capacity(decompressed_size as usize);
        r.read_to_end(&mut buf).unwrap();
        let mut r = &buf[..];

        // moves: CompressedMove[-]
        let moves = {
            let mut current_size = {
                0
            };
            let mut moves = Vec::with_capacity(body_size as usize - current_size);
            while !r.is_empty() {
                let a = CompressedMove::read(&mut r)?;
                current_size += a.size();
                moves.push(a);
            }
            moves
        };

        Ok(Self {
            moves,
        })
    }

}

impl crate::Message for SMSG_COMPRESSED_MOVES {
    const OPCODE: u32 = 0x02fb;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_COMPRESSED_MOVES"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        w.write_all(&(self.size_uncompressed() as u32).to_le_bytes())?;

        let mut w = &mut flate2::write::ZlibEncoder::new(w, flate2::Compression::fast());

        // moves: CompressedMove[-]
        for i in self.moves.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(763, "SMSG_COMPRESSED_MOVES", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_COMPRESSED_MOVES {
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

impl SMSG_COMPRESSED_MOVES {
    pub(crate) fn size(&self) -> usize {
        use crate::traits::Message;

        let mut v = Vec::new();
        self.write_into_vec(&mut v);
        v.len()
    }
}

impl SMSG_COMPRESSED_MOVES {
    pub(crate) fn size_uncompressed(&self) -> usize {
        self.moves.iter().fold(0, |acc, x| acc + x.size()) // moves: CompressedMove[-]
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_COMPRESSED_MOVES;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_COMPRESSED_MOVES, expected: &SMSG_COMPRESSED_MOVES) {
        assert_eq!(t.moves, expected.moves);
    }

    const RAW0: [u8; 52] = [ 0x00, 0x32, 0xFB, 0x02, 0x2E, 0x00, 0x00, 0x00, 0x78,
         0x01, 0xD3, 0xBD, 0xCB, 0xC0, 0x28, 0x91, 0xB7, 0x9A, 0xFB, 0xD8, 0xBA,
         0x58, 0xE6, 0xC3, 0x2B, 0xD4, 0x97, 0x3B, 0x31, 0x20, 0x03, 0x46, 0x20,
         0xA7, 0x64, 0x39, 0xF7, 0xB1, 0xF5, 0xEF, 0x5F, 0x1D, 0x3A, 0x79, 0x66,
         0x89, 0x13, 0x00, 0x26, 0x1E, 0x0E, 0x49, ];

    pub(crate) fn expected0() -> SMSG_COMPRESSED_MOVES {
        SMSG_COMPRESSED_MOVES {
            moves: vec![
                CompressedMove {
                    opcode: CompressedMove_CompressedMoveOpcode::SmsgMonsterMove {
                        monster_move: MonsterMove {
                            spline_point: Vector3d {
                                x: -8938.857_f32,
                                y: -131.36594_f32,
                                z: 83.57745_f32,
                            },
                            spline_id: 0x0,
                            move_type: MonsterMove_MonsterMoveType::Normal {
                                duration: 0x0,
                                spline_flags: SplineFlag::empty()
                                    ,
                                splines: vec![
                                    Vector3d {
                                        x: -8937.863,
                                        y: -117.46813,
                                        z: 82.39997,
                                    },
                                ],
                            },
                        },
                    },
                    guid: Guid::new(0x18),
                },
            ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm` line 35.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_compressed_moves0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_MOVES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        let s = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&dest)).unwrap();
        let s = match s {
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(s) => s,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_MOVES, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm` line 35.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_compressed_moves0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_MOVES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        let s = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&dest)).await.unwrap();
        let s = match s {
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(s) => s,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_MOVES, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm` line 35.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_compressed_moves0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_MOVES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        let s = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&dest)).await.unwrap();
        let s = match s {
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(s) => s,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_MOVES, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

    const RAW1: [u8; 72] = [ 0x00, 0x46, 0xFB, 0x02, 0x3B, 0x00, 0x00, 0x00, 0x78,
         0x01, 0xB3, 0xBA, 0xCB, 0x70, 0x3B, 0x5F, 0xC6, 0x41, 0xDC, 0xE0, 0x63,
         0xF5, 0x1B, 0xB1, 0x23, 0xD7, 0x17, 0x37, 0x1F, 0x6D, 0x50, 0x94, 0x71,
         0xD2, 0x57, 0x64, 0x00, 0x02, 0x46, 0x06, 0x86, 0x63, 0x1C, 0x0C, 0x0C,
         0xCC, 0x40, 0x66, 0xEB, 0x6B, 0xB1, 0x23, 0x5C, 0xE5, 0xCD, 0x47, 0x15,
         0xB7, 0xCB, 0x3A, 0x31, 0x14, 0x00, 0x05, 0x0C, 0x18, 0x18, 0x00, 0x58,
         0xE3, 0x11, 0x04, ];

    pub(crate) fn expected1() -> SMSG_COMPRESSED_MOVES {
        SMSG_COMPRESSED_MOVES {
            moves: vec![
                CompressedMove {
                    opcode: CompressedMove_CompressedMoveOpcode::SmsgMonsterMove {
                        monster_move: MonsterMove {
                            spline_point: Vector3d {
                                x: -603.695_f32,
                                y: -4212.48_f32,
                                z: 39.032715_f32,
                            },
                            spline_id: 0x212F,
                            move_type: MonsterMove_MonsterMoveType::Normal {
                                duration: 0x8C6,
                                spline_flags: SplineFlag::empty()
                                    .set_run_mode()
                                    ,
                                splines: vec![
                                    Vector3d {
                                        x: -603.68,
                                        y: -4206.88,
                                        z: 39.428837,
                                    },
                                    Vector3d {
                                        x: 0.0,
                                        y: 3.0,
                                        z: 0.0,
                                    },
                                    Vector3d {
                                        x: 0.0,
                                        y: 1.0,
                                        z: 0.0,
                                    },
                                ],
                            },
                        },
                    },
                    guid: Guid::new(0xF130001740001C6F),
                },
            ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm` line 74.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_compressed_moves1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_MOVES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        let s = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&dest)).unwrap();
        let s = match s {
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(s) => s,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_MOVES, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm` line 74.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_compressed_moves1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_MOVES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        let s = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&dest)).await.unwrap();
        let s = match s {
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(s) => s,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_MOVES, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm` line 74.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_compressed_moves1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_MOVES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        let s = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&dest)).await.unwrap();
        let s = match s {
            ServerOpcodeMessage::SMSG_COMPRESSED_MOVES(s) => s,
            opcode => panic!("incorrect opcode. Expected SMSG_COMPRESSED_MOVES, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

}

