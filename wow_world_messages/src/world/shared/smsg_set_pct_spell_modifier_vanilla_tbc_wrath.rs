use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_set_pct_spell_modifier.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_set_pct_spell_modifier.wowm#L3):
/// ```text
/// smsg SMSG_SET_PCT_SPELL_MODIFIER = 0x0267 {
///     u8 eff;
///     u8 op;
///     u32 value;
/// }
/// ```
pub struct SMSG_SET_PCT_SPELL_MODIFIER {
    pub eff: u8,
    pub op: u8,
    pub value: u32,
}

impl crate::private::Sealed for SMSG_SET_PCT_SPELL_MODIFIER {}
impl SMSG_SET_PCT_SPELL_MODIFIER {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 6 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // eff: u8
        let eff = crate::util::read_u8_le(&mut r)?;

        // op: u8
        let op = crate::util::read_u8_le(&mut r)?;

        // value: u32
        let value = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            eff,
            op,
            value,
        })
    }

}

impl crate::Message for SMSG_SET_PCT_SPELL_MODIFIER {
    const OPCODE: u32 = 0x0267;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SET_PCT_SPELL_MODIFIER"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SET_PCT_SPELL_MODIFIER {{").unwrap();
        // Members
        writeln!(s, "    eff = {};", self.eff).unwrap();
        writeln!(s, "    op = {};", self.op).unwrap();
        writeln!(s, "    value = {};", self.value).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 615_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "eff", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "op", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "value", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        6
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // eff: u8
        w.write_all(&self.eff.to_le_bytes())?;

        // op: u8
        w.write_all(&self.op.to_le_bytes())?;

        // value: u32
        w.write_all(&self.value.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(615, "SMSG_SET_PCT_SPELL_MODIFIER", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SET_PCT_SPELL_MODIFIER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SET_PCT_SPELL_MODIFIER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SET_PCT_SPELL_MODIFIER {}

