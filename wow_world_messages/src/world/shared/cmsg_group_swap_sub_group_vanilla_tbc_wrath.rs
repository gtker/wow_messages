use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_swap_sub_group.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_swap_sub_group.wowm#L3):
/// ```text
/// cmsg CMSG_GROUP_SWAP_SUB_GROUP = 0x0280 {
///     CString name;
///     CString swap_with_name;
/// }
/// ```
pub struct CMSG_GROUP_SWAP_SUB_GROUP {
    pub name: String,
    pub swap_with_name: String,
}

#[cfg(feature = "print-testcase")]
impl CMSG_GROUP_SWAP_SUB_GROUP {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GROUP_SWAP_SUB_GROUP {{").unwrap();
        // Members
        writeln!(s, "    name = \"{}\";", self.name).unwrap();
        writeln!(s, "    swap_with_name = \"{}\";", self.swap_with_name).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 640_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 1, "name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.swap_with_name.len() + 1, "swap_with_name", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_GROUP_SWAP_SUB_GROUP {}
impl crate::Message for CMSG_GROUP_SWAP_SUB_GROUP {
    const OPCODE: u32 = 0x0280;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_GROUP_SWAP_SUB_GROUP::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // swap_with_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.swap_with_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `swap_with_name` must not be null-terminated.");
        w.write_all(self.swap_with_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=512).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0280, size: body_size });
        }

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // swap_with_name: CString
        let swap_with_name = {
            let swap_with_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(swap_with_name)?
        };

        Ok(Self {
            name,
            swap_with_name,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GROUP_SWAP_SUB_GROUP {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GROUP_SWAP_SUB_GROUP {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GROUP_SWAP_SUB_GROUP {}

impl CMSG_GROUP_SWAP_SUB_GROUP {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + self.swap_with_name.len() + 1 // swap_with_name: CString
    }
}

