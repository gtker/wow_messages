use std::io::{Read, Write};

use crate::tbc::NpcTextUpdate;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_npc_text_update.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_npc_text_update.wowm#L10):
/// ```text
/// smsg SMSG_NPC_TEXT_UPDATE = 0x0180 {
///     u32 text_id;
///     NpcTextUpdate[8] texts;
/// }
/// ```
pub struct SMSG_NPC_TEXT_UPDATE {
    pub text_id: u32,
    pub texts: [NpcTextUpdate; 8],
}

#[cfg(feature = "print-testcase")]
impl SMSG_NPC_TEXT_UPDATE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_NPC_TEXT_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    text_id = {};", self.text_id).unwrap();
        write!(s, "    texts = [").unwrap();
        for v in self.texts.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    {}", if v.probability.to_string().contains(".") { v.probability.to_string() } else { format!("{}.0", v.probability) }).unwrap();
            write!(s, "        texts = [").unwrap();
            for v in v.texts.as_slice() {
                write!(s, "\"{v}\", ").unwrap();
            }
            writeln!(s, "];").unwrap();
            writeln!(s, "        language = {};", v.language.as_test_case_value()).unwrap();
            write!(s, "        emotes = [").unwrap();
            for v in v.emotes.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "            delay = {};", v.delay).unwrap();
                writeln!(s, "            emote = {};", v.emote).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 384_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "text_id", "    ");
        writeln!(s, "    /* texts: NpcTextUpdate[8] start */").unwrap();
        for (i, v) in self.texts.iter().enumerate() {
            writeln!(s, "    /* texts: NpcTextUpdate[8] {i} start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "probability", "        ");
            writeln!(s, "    /* texts: CString[2] start */").unwrap();
            for (i, v) in v.texts.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, &format!("texts {i}"), "        ");
            }
            writeln!(s, "    /* texts: CString[2] end */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 1, "language", "        ");
            writeln!(s, "    /* emotes: NpcTextUpdateEmote[3] start */").unwrap();
            for (i, v) in v.emotes.iter().enumerate() {
                writeln!(s, "    /* emotes: NpcTextUpdateEmote[3] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "delay", "            ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "emote", "            ");
                writeln!(s, "    /* emotes: NpcTextUpdateEmote[3] {i} end */").unwrap();
            }
            writeln!(s, "    /* emotes: NpcTextUpdateEmote[3] end */").unwrap();
            writeln!(s, "    /* texts: NpcTextUpdate[8] {i} end */").unwrap();
        }
        writeln!(s, "    /* texts: NpcTextUpdate[8] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_NPC_TEXT_UPDATE {}
impl crate::Message for SMSG_NPC_TEXT_UPDATE {
    const OPCODE: u32 = 0x0180;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_NPC_TEXT_UPDATE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // text_id: u32
        w.write_all(&self.text_id.to_le_bytes())?;

        // texts: NpcTextUpdate[8]
        for i in self.texts.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(252..=4332).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0180, size: body_size });
        }

        // text_id: u32
        let text_id = crate::util::read_u32_le(&mut r)?;

        // texts: NpcTextUpdate[8]
        let texts = {
            let mut texts = [(); 8].map(|_| NpcTextUpdate::default());
            for i in texts.iter_mut() {
                *i = NpcTextUpdate::read(&mut r)?;
            }
            texts
        };

        Ok(Self {
            text_id,
            texts,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_NPC_TEXT_UPDATE {}

impl SMSG_NPC_TEXT_UPDATE {
    pub(crate) fn size(&self) -> usize {
        4 // text_id: u32
        + self.texts.iter().fold(0, |acc, x| acc + x.size()) // texts: NpcTextUpdate[8]
    }
}

