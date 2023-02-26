use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_lfg_join.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_lfg_join.wowm#L1):
/// ```text
/// cmsg CMSG_LFG_JOIN = 0x035C {
///     u32 roles;
///     Bool no_partial_clear;
///     Bool achievements;
///     u8 amount_of_slots;
///     u32[amount_of_slots] slots;
///     u8 amount_of_needs;
///     u8[amount_of_needs] needs;
///     CString comment;
/// }
/// ```
pub struct CMSG_LFG_JOIN {
    pub roles: u32,
    pub no_partial_clear: bool,
    pub achievements: bool,
    pub slots: Vec<u32>,
    pub needs: Vec<u8>,
    pub comment: String,
}

impl crate::Message for CMSG_LFG_JOIN {
    const OPCODE: u32 = 0x035c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // roles: u32
        w.write_all(&self.roles.to_le_bytes())?;

        // no_partial_clear: Bool
        w.write_all(u8::from(self.no_partial_clear).to_le_bytes().as_slice())?;

        // achievements: Bool
        w.write_all(u8::from(self.achievements).to_le_bytes().as_slice())?;

        // amount_of_slots: u8
        w.write_all(&(self.slots.len() as u8).to_le_bytes())?;

        // slots: u32[amount_of_slots]
        for i in self.slots.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // amount_of_needs: u8
        w.write_all(&(self.needs.len() as u8).to_le_bytes())?;

        // needs: u8[amount_of_needs]
        for i in self.needs.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // comment: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.comment.as_bytes().iter().rev().next(), Some(&0_u8), "String `comment` must not be null-terminated.");
        w.write_all(self.comment.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=1544).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x035C, size: body_size as u32 });
        }

        // roles: u32
        let roles = crate::util::read_u32_le(r)?;

        // no_partial_clear: Bool
        let no_partial_clear = crate::util::read_u8_le(r)? != 0;
        // achievements: Bool
        let achievements = crate::util::read_u8_le(r)? != 0;
        // amount_of_slots: u8
        let amount_of_slots = crate::util::read_u8_le(r)?;

        // slots: u32[amount_of_slots]
        let slots = {
            let mut slots = Vec::with_capacity(amount_of_slots as usize);
            for i in 0..amount_of_slots {
                slots.push(crate::util::read_u32_le(r)?);
            }
            slots
        };
        // amount_of_needs: u8
        let amount_of_needs = crate::util::read_u8_le(r)?;

        // needs: u8[amount_of_needs]
        let needs = {
            let mut needs = Vec::with_capacity(amount_of_needs as usize);
            for i in 0..amount_of_needs {
                needs.push(crate::util::read_u8_le(r)?);
            }
            needs
        };
        // comment: CString
        let comment = {
            let comment = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(comment)?
        };

        Ok(Self {
            roles,
            no_partial_clear,
            achievements,
            slots,
            needs,
            comment,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LFG_JOIN {}

impl CMSG_LFG_JOIN {
    pub(crate) fn size(&self) -> usize {
        4 // roles: u32
        + 1 // no_partial_clear: Bool
        + 1 // achievements: Bool
        + 1 // amount_of_slots: u8
        + self.slots.len() * core::mem::size_of::<u32>() // slots: u32[amount_of_slots]
        + 1 // amount_of_needs: u8
        + self.needs.len() * core::mem::size_of::<u8>() // needs: u8[amount_of_needs]
        + self.comment.len() + 1 // comment: CString
    }
}

