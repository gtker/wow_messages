use std::io::{Read, Write};

use crate::wrath::{
    Map, RaidDifficulty,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/cmsg_set_saved_instance_extend.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/cmsg_set_saved_instance_extend.wowm#L1):
/// ```text
/// cmsg CMSG_SET_SAVED_INSTANCE_EXTEND = 0x0292 {
///     Map map;
///     RaidDifficulty difficulty;
///     Bool toggle_extend;
/// }
/// ```
pub struct CMSG_SET_SAVED_INSTANCE_EXTEND {
    pub map: Map,
    pub difficulty: RaidDifficulty,
    pub toggle_extend: bool,
}

impl crate::private::Sealed for CMSG_SET_SAVED_INSTANCE_EXTEND {}
impl crate::Message for CMSG_SET_SAVED_INSTANCE_EXTEND {
    const OPCODE: u32 = 0x0292;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_SAVED_INSTANCE_EXTEND {{").unwrap();
        // Members
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        writeln!(s, "    difficulty = {};", self.difficulty.as_test_case_value()).unwrap();
        writeln!(s, "    toggle_extend = {};", if self.toggle_extend { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 658_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "difficulty", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "toggle_extend", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        6
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // difficulty: RaidDifficulty
        w.write_all(&(self.difficulty.as_int().to_le_bytes()))?;

        // toggle_extend: Bool
        w.write_all(u8::from(self.toggle_extend).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 6 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0292, size: body_size });
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // difficulty: RaidDifficulty
        let difficulty = crate::util::read_u8_le(&mut r)?.try_into()?;

        // toggle_extend: Bool
        let toggle_extend = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            map,
            difficulty,
            toggle_extend,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_SAVED_INSTANCE_EXTEND {}

