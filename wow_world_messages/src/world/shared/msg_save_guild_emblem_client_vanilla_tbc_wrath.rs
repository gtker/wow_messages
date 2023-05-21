use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_save_guild_emblem_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_save_guild_emblem_client.wowm#L3):
/// ```text
/// cmsg MSG_SAVE_GUILD_EMBLEM_Client = 0x01F1 {
///     Guid vendor;
///     u32 emblem_style;
///     u32 emblem_color;
///     u32 border_style;
///     u32 border_color;
///     u32 background_color;
/// }
/// ```
pub struct MSG_SAVE_GUILD_EMBLEM_Client {
    pub vendor: Guid,
    pub emblem_style: u32,
    pub emblem_color: u32,
    pub border_style: u32,
    pub border_color: u32,
    pub background_color: u32,
}

impl crate::private::Sealed for MSG_SAVE_GUILD_EMBLEM_Client {}
impl MSG_SAVE_GUILD_EMBLEM_Client {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 28 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // vendor: Guid
        let vendor = crate::util::read_guid(&mut r)?;

        // emblem_style: u32
        let emblem_style = crate::util::read_u32_le(&mut r)?;

        // emblem_color: u32
        let emblem_color = crate::util::read_u32_le(&mut r)?;

        // border_style: u32
        let border_style = crate::util::read_u32_le(&mut r)?;

        // border_color: u32
        let border_color = crate::util::read_u32_le(&mut r)?;

        // background_color: u32
        let background_color = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            vendor,
            emblem_style,
            emblem_color,
            border_style,
            border_color,
            background_color,
        })
    }

}

impl crate::Message for MSG_SAVE_GUILD_EMBLEM_Client {
    const OPCODE: u32 = 0x01f1;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_SAVE_GUILD_EMBLEM_Client {{").unwrap();
        // Members
        writeln!(s, "    vendor = {};", self.vendor.guid()).unwrap();
        writeln!(s, "    emblem_style = {};", self.emblem_style).unwrap();
        writeln!(s, "    emblem_color = {};", self.emblem_color).unwrap();
        writeln!(s, "    border_style = {};", self.border_style).unwrap();
        writeln!(s, "    border_color = {};", self.border_color).unwrap();
        writeln!(s, "    background_color = {};", self.background_color).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 32_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 497_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "vendor", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "emblem_style", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "emblem_color", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "border_style", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "border_color", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "background_color", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        28
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // emblem_style: u32
        w.write_all(&self.emblem_style.to_le_bytes())?;

        // emblem_color: u32
        w.write_all(&self.emblem_color.to_le_bytes())?;

        // border_style: u32
        w.write_all(&self.border_style.to_le_bytes())?;

        // border_color: u32
        w.write_all(&self.border_color.to_le_bytes())?;

        // background_color: u32
        w.write_all(&self.background_color.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(497, "MSG_SAVE_GUILD_EMBLEM_Client", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_SAVE_GUILD_EMBLEM_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_SAVE_GUILD_EMBLEM_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_SAVE_GUILD_EMBLEM_Client {}

