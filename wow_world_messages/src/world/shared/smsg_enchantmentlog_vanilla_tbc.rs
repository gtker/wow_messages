use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// cmangos and vmangos/mangoszero disagree about packed and extra u8
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_enchantmentlog.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_enchantmentlog.wowm#L1):
/// ```text
/// smsg SMSG_ENCHANTMENTLOG = 0x01D7 {
///     Guid target;
///     Guid caster;
///     u32 item;
///     u32 spell;
///     Bool show_affiliation;
/// }
/// ```
pub struct SMSG_ENCHANTMENTLOG {
    pub target: Guid,
    /// vmangos: message says enchant has faded if empty
    pub caster: Guid,
    pub item: u32,
    pub spell: u32,
    /// vmangos: Only used if `caster` is not 0.
    pub show_affiliation: bool,
}

impl crate::private::Sealed for SMSG_ENCHANTMENTLOG {}
impl SMSG_ENCHANTMENTLOG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 25 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        // caster: Guid
        let caster = crate::util::read_guid(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // show_affiliation: Bool
        let show_affiliation = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            target,
            caster,
            item,
            spell,
            show_affiliation,
        })
    }

}

impl crate::Message for SMSG_ENCHANTMENTLOG {
    const OPCODE: u32 = 0x01d7;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ENCHANTMENTLOG {{").unwrap();
        // Members
        writeln!(s, "    target = {};", self.target.guid()).unwrap();
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    item = {};", self.item).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    show_affiliation = {};", if self.show_affiliation { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 27_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 471_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "show_affiliation", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        25
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // show_affiliation: Bool
        w.write_all(u8::from(self.show_affiliation).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(471, "SMSG_ENCHANTMENTLOG", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ENCHANTMENTLOG {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ENCHANTMENTLOG {}

