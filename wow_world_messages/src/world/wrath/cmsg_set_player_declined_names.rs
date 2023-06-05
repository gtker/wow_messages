use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_set_player_declined_names.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_set_player_declined_names.wowm#L9):
/// ```text
/// cmsg CMSG_SET_PLAYER_DECLINED_NAMES = 0x0419 {
///     Guid player;
///     CString name;
///     CString[5] declined_names;
/// }
/// ```
pub struct CMSG_SET_PLAYER_DECLINED_NAMES {
    pub player: Guid,
    pub name: String,
    pub declined_names: [String; 5],
}

#[cfg(feature = "print-testcase")]
impl CMSG_SET_PLAYER_DECLINED_NAMES {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_PLAYER_DECLINED_NAMES {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    name = \"{}\";", self.name).unwrap();
        write!(s, "    declined_names = [").unwrap();
        for v in self.declined_names.as_slice() {
            write!(s, "\"{v}\", ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 6).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1049_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "player");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_SET_PLAYER_DECLINED_NAMES {}
impl crate::Message for CMSG_SET_PLAYER_DECLINED_NAMES {
    const OPCODE: u32 = 0x0419;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(14..=1544).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0419, size: body_size });
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
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_PLAYER_DECLINED_NAMES {}

impl CMSG_SET_PLAYER_DECLINED_NAMES {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + self.name.len() + 1 // name: CString
        + self.declined_names.iter().fold(0, |acc, x| acc + x.len() + 1) // declined_names: CString[5]
    }
}

