use std::io::{Read, Write};

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
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

impl crate::private::Sealed for SMSG_GUILD_QUERY_RESPONSE {}
impl SMSG_GUILD_QUERY_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(35..=2840).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // rank_names: CString[10]
        let rank_names = {
            let mut rank_names = [(); 10].map(|_| String::default());
            for i in rank_names.iter_mut() {
                let s = crate::util::read_c_string_to_vec(&mut r)?;
                *i = String::from_utf8(s)?;
            }
            rank_names
        };

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

impl crate::Message for SMSG_GUILD_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0055;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_GUILD_QUERY_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GUILD_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    id = {};", self.id).unwrap();
        writeln!(s, "    name = \"{}\";", self.name).unwrap();
        writeln!(s, "    rank_names = [").unwrap();
        for v in self.rank_names.as_slice() {
            write!(s, "\"{v}\", ").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    emblem_style = {};", self.emblem_style).unwrap();
        writeln!(s, "    emblem_color = {};", self.emblem_color).unwrap();
        writeln!(s, "    border_style = {};", self.border_style).unwrap();
        writeln!(s, "    border_color = {};", self.border_color).unwrap();
        writeln!(s, "    background_color = {};", self.background_color).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 85_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 1, "name", "    ");
        writeln!(s, "    /* rank_names: CString[10] start */").unwrap();
        for (i, v) in self.rank_names.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, &format!("rank_names {i}"), "    ");
        }
        writeln!(s, "    /* rank_names: CString[10] end */").unwrap();
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
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().next_back(), Some(&0_u8), "String `name` must not be null-terminated.");
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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(85, "SMSG_GUILD_QUERY_RESPONSE", body_size, a))
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

