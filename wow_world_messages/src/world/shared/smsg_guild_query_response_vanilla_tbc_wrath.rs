use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_guild_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_guild_query_response.wowm#L1):
/// ```text
/// smsg SMSG_GUILD_QUERY_RESPONSE = 0x0055 {
///     u32 id;
///     CString name;
///     CString[10] rank_names;
///     u32 emblem_style;
///     u32 emblem_color;
///     u32 border_style;
///     u32 border_color;
///     u32 background_color;
/// }
/// ```
pub struct SMSG_GUILD_QUERY_RESPONSE {
    pub id: u32,
    pub name: String,
    pub rank_names: [String; 10],
    pub emblem_style: u32,
    pub emblem_color: u32,
    pub border_style: u32,
    pub border_color: u32,
    pub background_color: u32,
}

impl crate::Message for SMSG_GUILD_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0055;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // rank_names: CString[10]
        for i in self.rank_names.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

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
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(35..=2840).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0055, size: body_size as u32 });
        }

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(name)?
        };

        // rank_names: CString[10]
        let rank_names = {
            let mut rank_names = Vec::with_capacity(10);
            for i in 0..10 {
                let s = crate::util::read_c_string_to_vec(r)?;
                rank_names.push(String::from_utf8(s)?);
            }
            let rank_names = rank_names.try_into().unwrap();
            rank_names
        };

        // emblem_style: u32
        let emblem_style = crate::util::read_u32_le(r)?;

        // emblem_color: u32
        let emblem_color = crate::util::read_u32_le(r)?;

        // border_style: u32
        let border_style = crate::util::read_u32_le(r)?;

        // border_color: u32
        let border_color = crate::util::read_u32_le(r)?;

        // background_color: u32
        let background_color = crate::util::read_u32_le(r)?;

        Ok(Self {
            id,
            name,
            rank_names,
            emblem_style,
            emblem_color,
            border_style,
            border_color,
            background_color,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GUILD_QUERY_RESPONSE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GUILD_QUERY_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GUILD_QUERY_RESPONSE {}

impl SMSG_GUILD_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // id: u32
        + self.name.len() + 1 // name: CString
        + self.rank_names.iter().fold(0, |acc, x| acc + x.len() + 1) // rank_names: CString[10]
        + 4 // emblem_style: u32
        + 4 // emblem_color: u32
        + 4 // border_style: u32
        + 4 // border_color: u32
        + 4 // background_color: u32
    }
}

