use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_change_sub_group.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_change_sub_group.wowm#L3):
/// ```text
/// cmsg CMSG_GROUP_CHANGE_SUB_GROUP = 0x027E {
///     CString name;
///     u8 group_number;
/// }
/// ```
pub struct CMSG_GROUP_CHANGE_SUB_GROUP {
    pub name: String,
    pub group_number: u8,
}

impl crate::private::Sealed for CMSG_GROUP_CHANGE_SUB_GROUP {}
impl CMSG_GROUP_CHANGE_SUB_GROUP {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(2..=257).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // group_number: u8
        let group_number = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            name,
            group_number,
        })
    }

}

impl crate::Message for CMSG_GROUP_CHANGE_SUB_GROUP {
    const OPCODE: u32 = 0x027e;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_GROUP_CHANGE_SUB_GROUP"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GROUP_CHANGE_SUB_GROUP {{").unwrap();
        // Members
        writeln!(s, "    name = \"{}\";", self.name).unwrap();
        writeln!(s, "    group_number = {};", self.group_number).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 638_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 1, "name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "group_number", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().next_back(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // group_number: u8
        w.write_all(&self.group_number.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(638, "CMSG_GROUP_CHANGE_SUB_GROUP", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GROUP_CHANGE_SUB_GROUP {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GROUP_CHANGE_SUB_GROUP {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GROUP_CHANGE_SUB_GROUP {}

impl CMSG_GROUP_CHANGE_SUB_GROUP {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + 1 // group_number: u8
    }
}

