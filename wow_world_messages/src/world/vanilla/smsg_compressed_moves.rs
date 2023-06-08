use crate::Message;
use std::io::{Read, Write};

use crate::vanilla::CompressedMove;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm:48`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_compressed_moves.wowm#L48):
/// ```text
/// smsg SMSG_COMPRESSED_MOVES = 0x02FB {
///     CompressedMove[-] moves;
/// }
/// ```
pub struct SMSG_COMPRESSED_MOVES {
    pub moves: Vec<CompressedMove>,
}

impl crate::private::Sealed for SMSG_COMPRESSED_MOVES {}
impl crate::Message for SMSG_COMPRESSED_MOVES {
    const OPCODE: u32 = 0x02fb;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_COMPRESSED_MOVES {{").unwrap();
        // Members
        write!(s, "    moves = [").unwrap();
        for v in self.moves.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        opcode = {};", crate::vanilla::CompressedMoveOpcode::try_from(v.opcode.as_int()).unwrap().as_test_case_value()).unwrap();
            writeln!(s, "        guid = {};", v.guid.guid()).unwrap();
            match &v.opcode {
                crate::vanilla::CompressedMove_CompressedMoveOpcode::SmsgMonsterMove {
                    monster_move,
                } => {
                    // monster_move: MonsterMove
                    writeln!(s, "        monster_move = {{").unwrap();
                    // Members
                    // spline_point: Vector3d
                    writeln!(s, "            spline_point = {{").unwrap();
                    // Members
                    writeln!(s, "    {}", if monster_move.spline_point.x.to_string().contains('.') { monster_move.spline_point.x.to_string() } else { format!("{}.0", monster_move.spline_point.x) }).unwrap();
                    writeln!(s, "    {}", if monster_move.spline_point.y.to_string().contains('.') { monster_move.spline_point.y.to_string() } else { format!("{}.0", monster_move.spline_point.y) }).unwrap();
                    writeln!(s, "    {}", if monster_move.spline_point.z.to_string().contains('.') { monster_move.spline_point.z.to_string() } else { format!("{}.0", monster_move.spline_point.z) }).unwrap();

                    writeln!(s, "    }};").unwrap();
                    writeln!(s, "            spline_id = {};", monster_move.spline_id).unwrap();
                    writeln!(s, "            move_type = {};", crate::vanilla::MonsterMoveType::try_from(monster_move.move_type.as_int()).unwrap().as_test_case_value()).unwrap();
                    match &monster_move.move_type {
                        crate::vanilla::MonsterMove_MonsterMoveType::Normal {
                            duration,
                            spline_flags,
                            splines,
                        } => {
                        }
                        crate::vanilla::MonsterMove_MonsterMoveType::FacingSpot {
                            duration,
                            position,
                            spline_flags,
                            splines,
                        } => {
                            // position: Vector3d
                            writeln!(s, "            position = {{").unwrap();
                            // Members
                            writeln!(s, "    {}", if position.x.to_string().contains('.') { position.x.to_string() } else { format!("{}.0", position.x) }).unwrap();
                            writeln!(s, "    {}", if position.y.to_string().contains('.') { position.y.to_string() } else { format!("{}.0", position.y) }).unwrap();
                            writeln!(s, "    {}", if position.z.to_string().contains('.') { position.z.to_string() } else { format!("{}.0", position.z) }).unwrap();

                            writeln!(s, "    }};").unwrap();
                        }
                        crate::vanilla::MonsterMove_MonsterMoveType::FacingTarget {
                            duration,
                            spline_flags,
                            splines,
                            target,
                        } => {
                            writeln!(s, "            target = {};", target.guid()).unwrap();
                        }
                        crate::vanilla::MonsterMove_MonsterMoveType::FacingAngle {
                            angle,
                            duration,
                            spline_flags,
                            splines,
                        } => {
                            writeln!(s, "    {}", if angle.to_string().contains('.') { angle.to_string() } else { format!("{}.0", angle) }).unwrap();
                        }
                        _ => {}
                    }

                    match &monster_move.move_type {
                        crate::vanilla::MonsterMove_MonsterMoveType::Normal {
                            duration,
                            spline_flags,
                            splines,
                        } => {
                            writeln!(s, "            spline_flags = {};", spline_flags.as_test_case_value()).unwrap();
                            writeln!(s, "            duration = {};", duration).unwrap();
                            return None;
                        }
                        crate::vanilla::MonsterMove_MonsterMoveType::FacingSpot {
                            duration,
                            position,
                            spline_flags,
                            splines,
                        } => {
                            writeln!(s, "            spline_flags = {};", spline_flags.as_test_case_value()).unwrap();
                            writeln!(s, "            duration = {};", duration).unwrap();
                            return None;
                        }
                        crate::vanilla::MonsterMove_MonsterMoveType::FacingTarget {
                            duration,
                            spline_flags,
                            splines,
                            target,
                        } => {
                            writeln!(s, "            spline_flags = {};", spline_flags.as_test_case_value()).unwrap();
                            writeln!(s, "            duration = {};", duration).unwrap();
                            return None;
                        }
                        crate::vanilla::MonsterMove_MonsterMoveType::FacingAngle {
                            angle,
                            duration,
                            spline_flags,
                            splines,
                        } => {
                            writeln!(s, "            spline_flags = {};", spline_flags.as_test_case_value()).unwrap();
                            writeln!(s, "            duration = {};", duration).unwrap();
                            return None;
                        }
                        _ => {}
                    }


                    writeln!(s, "    }};").unwrap();
                }
                crate::vanilla::CompressedMove_CompressedMoveOpcode::SmsgMonsterMoveTransport {
                    monster_move_transport,
                    transport,
                } => {
                    writeln!(s, "        transport = {};", transport.guid()).unwrap();
                    // monster_move_transport: MonsterMove
                    writeln!(s, "        monster_move_transport = {{").unwrap();
                    // Members
                    // spline_point: Vector3d
                    writeln!(s, "            spline_point = {{").unwrap();
                    // Members
                    writeln!(s, "    {}", if monster_move_transport.spline_point.x.to_string().contains('.') { monster_move_transport.spline_point.x.to_string() } else { format!("{}.0", monster_move_transport.spline_point.x) }).unwrap();
                    writeln!(s, "    {}", if monster_move_transport.spline_point.y.to_string().contains('.') { monster_move_transport.spline_point.y.to_string() } else { format!("{}.0", monster_move_transport.spline_point.y) }).unwrap();
                    writeln!(s, "    {}", if monster_move_transport.spline_point.z.to_string().contains('.') { monster_move_transport.spline_point.z.to_string() } else { format!("{}.0", monster_move_transport.spline_point.z) }).unwrap();

                    writeln!(s, "    }};").unwrap();
                    writeln!(s, "            spline_id = {};", monster_move_transport.spline_id).unwrap();
                    writeln!(s, "            move_type = {};", crate::vanilla::MonsterMoveType::try_from(monster_move_transport.move_type.as_int()).unwrap().as_test_case_value()).unwrap();
                    match &monster_move_transport.move_type {
                        crate::vanilla::MonsterMove_MonsterMoveType::Normal {
                            duration,
                            spline_flags,
                            splines,
                        } => {
                        }
                        crate::vanilla::MonsterMove_MonsterMoveType::FacingSpot {
                            duration,
                            position,
                            spline_flags,
                            splines,
                        } => {
                            // position: Vector3d
                            writeln!(s, "            position = {{").unwrap();
                            // Members
                            writeln!(s, "    {}", if position.x.to_string().contains('.') { position.x.to_string() } else { format!("{}.0", position.x) }).unwrap();
                            writeln!(s, "    {}", if position.y.to_string().contains('.') { position.y.to_string() } else { format!("{}.0", position.y) }).unwrap();
                            writeln!(s, "    {}", if position.z.to_string().contains('.') { position.z.to_string() } else { format!("{}.0", position.z) }).unwrap();

                            writeln!(s, "    }};").unwrap();
                        }
                        crate::vanilla::MonsterMove_MonsterMoveType::FacingTarget {
                            duration,
                            spline_flags,
                            splines,
                            target,
                        } => {
                            writeln!(s, "            target = {};", target.guid()).unwrap();
                        }
                        crate::vanilla::MonsterMove_MonsterMoveType::FacingAngle {
                            angle,
                            duration,
                            spline_flags,
                            splines,
                        } => {
                            writeln!(s, "    {}", if angle.to_string().contains('.') { angle.to_string() } else { format!("{}.0", angle) }).unwrap();
                        }
                        _ => {}
                    }

                    match &monster_move_transport.move_type {
                        crate::vanilla::MonsterMove_MonsterMoveType::Normal {
                            duration,
                            spline_flags,
                            splines,
                        } => {
                            writeln!(s, "            spline_flags = {};", spline_flags.as_test_case_value()).unwrap();
                            writeln!(s, "            duration = {};", duration).unwrap();
                            return None;
                        }
                        crate::vanilla::MonsterMove_MonsterMoveType::FacingSpot {
                            duration,
                            position,
                            spline_flags,
                            splines,
                        } => {
                            writeln!(s, "            spline_flags = {};", spline_flags.as_test_case_value()).unwrap();
                            writeln!(s, "            duration = {};", duration).unwrap();
                            return None;
                        }
                        crate::vanilla::MonsterMove_MonsterMoveType::FacingTarget {
                            duration,
                            spline_flags,
                            splines,
                            target,
                        } => {
                            writeln!(s, "            spline_flags = {};", spline_flags.as_test_case_value()).unwrap();
                            writeln!(s, "            duration = {};", duration).unwrap();
                            return None;
                        }
                        crate::vanilla::MonsterMove_MonsterMoveType::FacingAngle {
                            angle,
                            duration,
                            spline_flags,
                            splines,
                        } => {
                            writeln!(s, "            spline_flags = {};", spline_flags.as_test_case_value()).unwrap();
                            writeln!(s, "            duration = {};", duration).unwrap();
                            return None;
                        }
                        _ => {}
                    }


                    writeln!(s, "    }};").unwrap();
                }
                crate::vanilla::CompressedMove_CompressedMoveOpcode::SmsgSplineSetRunSpeed {
                    speed,
                } => {
                    writeln!(s, "    {}", if speed.to_string().contains('.') { speed.to_string() } else { format!("{}.0", speed) }).unwrap();
                }
                _ => {}
            }


            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 763_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        if !self.moves.is_empty() {
            writeln!(s, "    /* moves: CompressedMove[-] start */").unwrap();
            for (i, v) in self.moves.iter().enumerate() {
                writeln!(s, "    /* moves: CompressedMove[-] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "size", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 2, "opcode", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&v.guid), "guid", "        ");
                match &v.opcode {
                    crate::vanilla::CompressedMove_CompressedMoveOpcode::SmsgMonsterMove {
                        monster_move,
                    } => {
                        writeln!(s, "    /* monster_move: MonsterMove start */").unwrap();
                        writeln!(s, "    /* spline_point: Vector3d start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                        writeln!(s, "    /* spline_point: Vector3d end */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_id", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "move_type", "            ");
                        match &monster_move.move_type {
                            crate::vanilla::MonsterMove_MonsterMoveType::Normal {
                                duration,
                                spline_flags,
                                splines,
                            } => {
                            }
                            crate::vanilla::MonsterMove_MonsterMoveType::FacingSpot {
                                duration,
                                position,
                                spline_flags,
                                splines,
                            } => {
                                writeln!(s, "    /* position: Vector3d start */").unwrap();
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                writeln!(s, "    /* position: Vector3d end */").unwrap();
                            }
                            crate::vanilla::MonsterMove_MonsterMoveType::FacingTarget {
                                duration,
                                spline_flags,
                                splines,
                                target,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "            ");
                            }
                            crate::vanilla::MonsterMove_MonsterMoveType::FacingAngle {
                                angle,
                                duration,
                                spline_flags,
                                splines,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "angle", "            ");
                            }
                            _ => {}
                        }

                        match &monster_move.move_type {
                            crate::vanilla::MonsterMove_MonsterMoveType::Normal {
                                duration,
                                spline_flags,
                                splines,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "            ");
                                panic!("unsupported type Vec<Vector3d> for variable 'splines'");
                            }
                            crate::vanilla::MonsterMove_MonsterMoveType::FacingSpot {
                                duration,
                                position,
                                spline_flags,
                                splines,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "            ");
                                panic!("unsupported type Vec<Vector3d> for variable 'splines'");
                            }
                            crate::vanilla::MonsterMove_MonsterMoveType::FacingTarget {
                                duration,
                                spline_flags,
                                splines,
                                target,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "            ");
                                panic!("unsupported type Vec<Vector3d> for variable 'splines'");
                            }
                            crate::vanilla::MonsterMove_MonsterMoveType::FacingAngle {
                                angle,
                                duration,
                                spline_flags,
                                splines,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "            ");
                                panic!("unsupported type Vec<Vector3d> for variable 'splines'");
                            }
                            _ => {}
                        }

                        writeln!(s, "    /* monster_move: MonsterMove end */").unwrap();
                    }
                    crate::vanilla::CompressedMove_CompressedMoveOpcode::SmsgMonsterMoveTransport {
                        monster_move_transport,
                        transport,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport), "transport", "        ");
                        writeln!(s, "    /* monster_move_transport: MonsterMove start */").unwrap();
                        writeln!(s, "    /* spline_point: Vector3d start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                        writeln!(s, "    /* spline_point: Vector3d end */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_id", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "move_type", "            ");
                        match &monster_move_transport.move_type {
                            crate::vanilla::MonsterMove_MonsterMoveType::Normal {
                                duration,
                                spline_flags,
                                splines,
                            } => {
                            }
                            crate::vanilla::MonsterMove_MonsterMoveType::FacingSpot {
                                duration,
                                position,
                                spline_flags,
                                splines,
                            } => {
                                writeln!(s, "    /* position: Vector3d start */").unwrap();
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                writeln!(s, "    /* position: Vector3d end */").unwrap();
                            }
                            crate::vanilla::MonsterMove_MonsterMoveType::FacingTarget {
                                duration,
                                spline_flags,
                                splines,
                                target,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "            ");
                            }
                            crate::vanilla::MonsterMove_MonsterMoveType::FacingAngle {
                                angle,
                                duration,
                                spline_flags,
                                splines,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "angle", "            ");
                            }
                            _ => {}
                        }

                        match &monster_move_transport.move_type {
                            crate::vanilla::MonsterMove_MonsterMoveType::Normal {
                                duration,
                                spline_flags,
                                splines,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "            ");
                                panic!("unsupported type Vec<Vector3d> for variable 'splines'");
                            }
                            crate::vanilla::MonsterMove_MonsterMoveType::FacingSpot {
                                duration,
                                position,
                                spline_flags,
                                splines,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "            ");
                                panic!("unsupported type Vec<Vector3d> for variable 'splines'");
                            }
                            crate::vanilla::MonsterMove_MonsterMoveType::FacingTarget {
                                duration,
                                spline_flags,
                                splines,
                                target,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "            ");
                                panic!("unsupported type Vec<Vector3d> for variable 'splines'");
                            }
                            crate::vanilla::MonsterMove_MonsterMoveType::FacingAngle {
                                angle,
                                duration,
                                spline_flags,
                                splines,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "            ");
                                panic!("unsupported type Vec<Vector3d> for variable 'splines'");
                            }
                            _ => {}
                        }

                        writeln!(s, "    /* monster_move_transport: MonsterMove end */").unwrap();
                    }
                    crate::vanilla::CompressedMove_CompressedMoveOpcode::SmsgSplineSetRunSpeed {
                        speed,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "speed", "        ");
                    }
                    _ => {}
                }

                writeln!(s, "    /* moves: CompressedMove[-] {i} end */").unwrap();
            }
            writeln!(s, "    /* moves: CompressedMove[-] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
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

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size > 65535 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02FB, size: body_size });
        }

        let decompressed_size = crate::util::read_u32_le(r)?;;
        let decompressed_buffer = vec![0; decompressed_size as usize];
        let mut r = &mut flate2::read::ZlibDecoder::new_with_buf(r, decompressed_buffer);

        // moves: CompressedMove[-]
        let moves = {
            let mut current_size = {
                0
            };
            let mut moves = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                moves.push(CompressedMove::read(&mut r)?);
                current_size += 1;
            }
            moves
        };

        Ok(Self {
            moves,
        })
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

