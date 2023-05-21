use std::io::{Read, Write};

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

impl crate::private::Sealed for CMSG_LFG_JOIN {}
impl CMSG_LFG_JOIN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=1544).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // roles: u32
        let roles = crate::util::read_u32_le(&mut r)?;

        // no_partial_clear: Bool
        let no_partial_clear = crate::util::read_u8_le(&mut r)? != 0;

        // achievements: Bool
        let achievements = crate::util::read_u8_le(&mut r)? != 0;

        // amount_of_slots: u8
        let amount_of_slots = crate::util::read_u8_le(&mut r)?;

        // slots: u32[amount_of_slots]
        let slots = {
            let mut slots = Vec::with_capacity(amount_of_slots as usize);
            for _ in 0..amount_of_slots {
                slots.push(crate::util::read_u32_le(&mut r)?);
            }
            slots
        };

        // amount_of_needs: u8
        let amount_of_needs = crate::util::read_u8_le(&mut r)?;

        // needs: u8[amount_of_needs]
        let needs = {
            let mut needs = Vec::with_capacity(amount_of_needs as usize);
            for _ in 0..amount_of_needs {
                needs.push(crate::util::read_u8_le(&mut r)?);
            }
            needs
        };

        // comment: CString
        let comment = {
            let comment = crate::util::read_c_string_to_vec(&mut r)?;
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

impl crate::Message for CMSG_LFG_JOIN {
    const OPCODE: u32 = 0x035c;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LFG_JOIN {{").unwrap();
        // Members
        writeln!(s, "    roles = {};", self.roles).unwrap();
        writeln!(s, "    no_partial_clear = {};", if self.no_partial_clear { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    achievements = {};", if self.achievements { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    amount_of_slots = {};", self.slots.len()).unwrap();
        write!(s, "    slots = [").unwrap();
        for v in self.slots.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    amount_of_needs = {};", self.needs.len()).unwrap();
        write!(s, "    needs = [").unwrap();
        for v in self.needs.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    comment = \"{}\";", self.comment).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 860_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "roles", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "no_partial_clear", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "achievements", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_slots", "    ");
        if !self.slots.is_empty() {
            writeln!(s, "    /* slots: u32[amount_of_slots] start */").unwrap();
            for (i, v) in self.slots.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("slots {i}"), "    ");
            }
            writeln!(s, "    /* slots: u32[amount_of_slots] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_needs", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.needs.len(), "needs", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.comment.len() + 1, "comment", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(860, "CMSG_LFG_JOIN", body_size, a))
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

