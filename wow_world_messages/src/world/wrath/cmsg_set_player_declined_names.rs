use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_set_player_declined_names.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_set_player_declined_names.wowm#L9):
/// ```text
/// cmsg CMSG_SET_PLAYER_DECLINED_NAMES = 0x0419 {
///     Guid player;
///     CString name;
///     CString[5] declined_names;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_SET_PLAYER_DECLINED_NAMES {
    pub player: Guid,
    pub name: String,
    pub declined_names: [String; 5],
}

impl crate::private::Sealed for CMSG_SET_PLAYER_DECLINED_NAMES {}
impl CMSG_SET_PLAYER_DECLINED_NAMES {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(14..=1544).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // declined_names: CString[5]
        let declined_names = {
            let mut declined_names = [(); 5].map(|_| String::default());
            for i in declined_names.iter_mut() {
                let s = crate::util::read_c_string_to_vec(&mut r)?;
                *i = String::from_utf8(s)?;
            }
            declined_names
        };

        Ok(Self {
            player,
            name,
            declined_names,
        })
    }

}

impl crate::Message for CMSG_SET_PLAYER_DECLINED_NAMES {
    const OPCODE: u32 = 0x0419;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SET_PLAYER_DECLINED_NAMES"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_PLAYER_DECLINED_NAMES {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    name = \"{}\";", self.name).unwrap();
        writeln!(s, "    declined_names = [").unwrap();
        for v in self.declined_names.as_slice() {
            write!(s, "\"{v}\", ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1049_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 1, "name", "    ");
        writeln!(s, "    /* declined_names: CString[5] start */").unwrap();
        for (i, v) in self.declined_names.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, &format!("declined_names {i}"), "    ");
        }
        writeln!(s, "    /* declined_names: CString[5] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().next_back(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // declined_names: CString[5]
        for i in self.declined_names.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1049, "CMSG_SET_PLAYER_DECLINED_NAMES", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_PLAYER_DECLINED_NAMES {}

impl CMSG_SET_PLAYER_DECLINED_NAMES {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + self.name.len() + 1 // name: CString
        + self.declined_names.iter().fold(0, |acc, x| acc + x.len() + 1) // declined_names: CString[5]
    }
}

