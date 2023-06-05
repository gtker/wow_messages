use std::io::{Read, Write};

use crate::vanilla::Addon;

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

#[cfg(feature = "print-testcase")]
impl SMSG_ADDON_INFO {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ADDON_INFO {{").unwrap();
        // Members
        write!(s, "    addons = [").unwrap();
        for v in self.addons.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    addon_type = {};", v.addon_type.as_test_case_value()).unwrap();
            writeln!(s, "    info_block = {};", crate::vanilla::InfoBlock::try_from(v.info_block.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.info_block {
                crate::vanilla::Addon_InfoBlock::Available {
                    key_version,
                    update_available_flag,
                } => {
                    writeln!(s, "    key_version = {};", crate::vanilla::KeyVersion::try_from(key_version.as_int()).unwrap().as_test_case_value()).unwrap();
                    match &key_version {
                        crate::vanilla::Addon_KeyVersion::One {
                            public_key,
                        } => {
                            write!(s, "    public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Two {
                            public_key,
                        } => {
                            write!(s, "    public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Three {
                            public_key,
                        } => {
                            write!(s, "    public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Four {
                            public_key,
                        } => {
                            write!(s, "    public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Five {
                            public_key,
                        } => {
                            write!(s, "    public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Six {
                            public_key,
                        } => {
                            write!(s, "    public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Seven {
                            public_key,
                        } => {
                            write!(s, "    public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Eight {
                            public_key,
                        } => {
                            write!(s, "    public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "];").unwrap();
                        }
                        crate::vanilla::Addon_KeyVersion::Nine {
                            public_key,
                        } => {
                            write!(s, "    public_key = [").unwrap();
                            for v in public_key.as_slice() {
                                write!(s, "{v:#04X}, ").unwrap();
                            }
                            writeln!(s, "];").unwrap();
                        }
                        _ => {}
                    }

                    writeln!(s, "    update_available_flag = {};", update_available_flag).unwrap();
                }
                _ => {}
            }

            writeln!(s, "    url_info = {};", crate::vanilla::UrlInfo::try_from(v.url_info.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.url_info {
                crate::vanilla::Addon_UrlInfo::Available {
                    url,
                } => {
                    writeln!(s, "    url = \"{}\";", url).unwrap();
                }
                _ => {}
            }


            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 751_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_ADDON_INFO {}
impl crate::Message for SMSG_ADDON_INFO {
    const OPCODE: u32 = 0x02ef;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size > 65535 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02EF, size: body_size });
        }

        // addons: Addon[-]
        let addons = {
            let mut current_size = {
                0
            };
            let mut addons = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                addons.push(Addon::read(&mut r)?);
                current_size += 1;
            }
            addons
        };

        Ok(Self {
            addons,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ADDON_INFO {}

impl SMSG_ADDON_INFO {
    pub(crate) fn size(&self) -> usize {
        self.addons.iter().fold(0, |acc, x| acc + x.size()) // addons: Addon[-]
    }
}

