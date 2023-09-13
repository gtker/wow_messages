use std::io::{Read, Write};

use crate::vanilla::{
    Addon, AddonType, InfoBlock, KeyVersion, UrlInfo,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm:60`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm#L60):
/// ```text
/// smsg SMSG_ADDON_INFO = 0x02EF {
///     Addon[-] addons;
/// }
/// ```
pub struct SMSG_ADDON_INFO {
    pub addons: Vec<Addon>,
}

impl crate::private::Sealed for SMSG_ADDON_INFO {}
impl SMSG_ADDON_INFO {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size > 65535 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // addons: Addon[-]
        let addons = {
            let mut current_size = {
                0
            };
            current_size += 4; // addons_decompressed_size: u32
            let mut addons = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                let a = Addon::read(&mut r)?;
                current_size += a.size();
                addons.push(a);
            }
            addons
        };

        Ok(Self {
            addons,
        })
    }

}

impl crate::Message for SMSG_ADDON_INFO {
    const OPCODE: u32 = 0x02ef;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ADDON_INFO"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ADDON_INFO {{").unwrap();
        // Members
        writeln!(s, "    addons = [").unwrap();
        for v in self.addons.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            addon_type = {};", v.addon_type.as_test_case_value()).unwrap();
            writeln!(s, "            info_block = {};", InfoBlock::try_from(v.info_block.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.info_block {
                crate::vanilla::Addon_InfoBlock::Available {
                    key_version,
                    update_available_flag,
                } => {
                    writeln!(s, "            key_version = {};", KeyVersion::try_from(key_version.as_int()).unwrap().as_test_case_value()).unwrap();
                    match &key_version {
                        crate::vanilla::Addon_KeyVersion::One {
                            public_key,
                        } => {
                            writeln!(s, "            public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "            ];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Two {
                            public_key,
                        } => {
                            writeln!(s, "            public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "            ];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Three {
                            public_key,
                        } => {
                            writeln!(s, "            public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "            ];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Four {
                            public_key,
                        } => {
                            writeln!(s, "            public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "            ];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Five {
                            public_key,
                        } => {
                            writeln!(s, "            public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "            ];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Six {
                            public_key,
                        } => {
                            writeln!(s, "            public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "            ];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Seven {
                            public_key,
                        } => {
                            writeln!(s, "            public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "            ];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Eight {
                            public_key,
                        } => {
                            writeln!(s, "            public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "            ];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Nine {
                            public_key,
                        } => {
                            writeln!(s, "            public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "            ];").unwrap();
                        }
                        _ => {}
                    }

                    writeln!(s, "            update_available_flag = {};", update_available_flag).unwrap();
                }
                _ => {}
            }

            writeln!(s, "            url_info = {};", UrlInfo::try_from(v.url_info.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.url_info {
                crate::vanilla::Addon_UrlInfo::Available {
                    url,
                } => {
                    writeln!(s, "            url = \"{}\";", url).unwrap();
                }
                _ => {}
            }


            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 751_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        if !self.addons.is_empty() {
            writeln!(s, "    /* addons: Addon[-] start */").unwrap();
            for (i, v) in self.addons.iter().enumerate() {
                writeln!(s, "    /* addons: Addon[-] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "addon_type", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "info_block", "        ");
                match &v.info_block {
                    crate::vanilla::Addon_InfoBlock::Available {
                        key_version,
                        update_available_flag,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "key_version", "        ");
                        match &key_version {
                            crate::vanilla::Addon_KeyVersion::One {
                                public_key,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, public_key.len(), "public_key", "        ");
                            }
                            crate::vanilla::Addon_KeyVersion::Two {
                                public_key,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, public_key.len(), "public_key", "        ");
                            }
                            crate::vanilla::Addon_KeyVersion::Three {
                                public_key,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, public_key.len(), "public_key", "        ");
                            }
                            crate::vanilla::Addon_KeyVersion::Four {
                                public_key,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, public_key.len(), "public_key", "        ");
                            }
                            crate::vanilla::Addon_KeyVersion::Five {
                                public_key,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, public_key.len(), "public_key", "        ");
                            }
                            crate::vanilla::Addon_KeyVersion::Six {
                                public_key,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, public_key.len(), "public_key", "        ");
                            }
                            crate::vanilla::Addon_KeyVersion::Seven {
                                public_key,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, public_key.len(), "public_key", "        ");
                            }
                            crate::vanilla::Addon_KeyVersion::Eight {
                                public_key,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, public_key.len(), "public_key", "        ");
                            }
                            crate::vanilla::Addon_KeyVersion::Nine {
                                public_key,
                            } => {
                                crate::util::write_bytes(&mut s, &mut bytes, public_key.len(), "public_key", "        ");
                            }
                            _ => {}
                        }

                        crate::util::write_bytes(&mut s, &mut bytes, 4, "update_available_flag", "        ");
                    }
                    _ => {}
                }

                crate::util::write_bytes(&mut s, &mut bytes, 1, "url_info", "        ");
                match &v.url_info {
                    crate::vanilla::Addon_UrlInfo::Available {
                        url,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, url.len() + 1, "url", "        ");
                    }
                    _ => {}
                }

                writeln!(s, "    /* addons: Addon[-] {i} end */").unwrap();
            }
            writeln!(s, "    /* addons: Addon[-] end */").unwrap();
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
        // addons: Addon[-]
        for i in self.addons.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(751, "SMSG_ADDON_INFO", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ADDON_INFO {}

impl SMSG_ADDON_INFO {
    pub(crate) fn size(&self) -> usize {
        self.addons.iter().fold(0, |acc, x| acc + x.size()) // addons: Addon[-]
    }
}

