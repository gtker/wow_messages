use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_query_quests_completed_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_query_quests_completed_response.wowm#L1):
/// ```text
/// smsg SMSG_QUERY_QUESTS_COMPLETED_RESPONSE = 0x0501 {
///     u32 amount_of_reward_quests;
///     u32[amount_of_reward_quests] reward_quests;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {
    pub reward_quests: Vec<u32>,
}

impl crate::private::Sealed for SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {}
impl SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_reward_quests: u32
        let amount_of_reward_quests = crate::util::read_u32_le(&mut r)?;

        // reward_quests: u32[amount_of_reward_quests]
        let reward_quests = {
            let mut reward_quests = Vec::with_capacity(amount_of_reward_quests as usize);

            let allocation_size = u64::from(amount_of_reward_quests) * 4;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_reward_quests {
                reward_quests.push(crate::util::read_u32_le(&mut r)?);
            }
            reward_quests
        };

        Ok(Self {
            reward_quests,
        })
    }

}

impl crate::Message for SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {
    const OPCODE: u32 = 0x0501;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_QUERY_QUESTS_COMPLETED_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    amount_of_reward_quests = {};", self.reward_quests.len()).unwrap();
        writeln!(s, "    reward_quests = [").unwrap();
        for v in self.reward_quests.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1281_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_reward_quests", "    ");
        if !self.reward_quests.is_empty() {
            writeln!(s, "    /* reward_quests: u32[amount_of_reward_quests] start */").unwrap();
            for (i, v) in self.reward_quests.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("reward_quests {i}"), "    ");
            }
            writeln!(s, "    /* reward_quests: u32[amount_of_reward_quests] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_reward_quests: u32
        w.write_all(&(self.reward_quests.len() as u32).to_le_bytes())?;

        // reward_quests: u32[amount_of_reward_quests]
        for i in self.reward_quests.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1281, "SMSG_QUERY_QUESTS_COMPLETED_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {}

impl SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_reward_quests: u32
        + self.reward_quests.len() * core::mem::size_of::<u32>() // reward_quests: u32[amount_of_reward_quests]
    }
}

