use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_update_search.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_update_search.wowm#L1):
/// ```text
/// smsg SMSG_LFG_UPDATE_SEARCH = 0x0369 {
///     Bool in_lfg_queue;
/// }
/// ```
pub struct SMSG_LFG_UPDATE_SEARCH {
    pub in_lfg_queue: bool,
}

impl crate::private::Sealed for SMSG_LFG_UPDATE_SEARCH {}
impl crate::Message for SMSG_LFG_UPDATE_SEARCH {
    const OPCODE: u32 = 0x0369;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_UPDATE_SEARCH {{").unwrap();
        // Members
        writeln!(s, "    in_lfg_queue = {};", if self.in_lfg_queue { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 3_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 873_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "in_lfg_queue", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // in_lfg_queue: Bool
        w.write_all(u8::from(self.in_lfg_queue).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0369, size: body_size });
        }

        // in_lfg_queue: Bool
        let in_lfg_queue = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            in_lfg_queue,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_UPDATE_SEARCH {}

