use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    PreviewTalent, Talent,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_learn_preview_talents_pet.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_learn_preview_talents_pet.wowm#L1):
/// ```text
/// cmsg CMSG_LEARN_PREVIEW_TALENTS_PET = 0x04C2 {
///     Guid pet;
///     u32 amount_of_talents;
///     PreviewTalent[amount_of_talents] talents;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_LEARN_PREVIEW_TALENTS_PET {
    pub pet: Guid,
    pub talents: Vec<PreviewTalent>,
}

impl crate::private::Sealed for CMSG_LEARN_PREVIEW_TALENTS_PET {}
impl CMSG_LEARN_PREVIEW_TALENTS_PET {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(12..=10240).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // pet: Guid
        let pet = crate::util::read_guid(&mut r)?;

        // amount_of_talents: u32
        let amount_of_talents = crate::util::read_u32_le(&mut r)?;

        // talents: PreviewTalent[amount_of_talents]
        let talents = {
            let mut talents = Vec::with_capacity(amount_of_talents as usize);

            let allocation_size = u64::from(amount_of_talents) * 8;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_talents {
                talents.push(PreviewTalent::read(&mut r)?);
            }
            talents
        };

        Ok(Self {
            pet,
            talents,
        })
    }

}

impl crate::Message for CMSG_LEARN_PREVIEW_TALENTS_PET {
    const OPCODE: u32 = 0x04c2;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_LEARN_PREVIEW_TALENTS_PET"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LEARN_PREVIEW_TALENTS_PET {{").unwrap();
        // Members
        writeln!(s, "    pet = {};", self.pet.guid()).unwrap();
        writeln!(s, "    amount_of_talents = {};", self.talents.len()).unwrap();
        writeln!(s, "    talents = [").unwrap();
        for v in self.talents.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            talent = {};", v.talent.as_test_case_value()).unwrap();
            writeln!(s, "            rank = {};", v.rank).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1218_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "pet", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_talents", "    ");
        if !self.talents.is_empty() {
            writeln!(s, "    /* talents: PreviewTalent[amount_of_talents] start */").unwrap();
            for (i, v) in self.talents.iter().enumerate() {
                writeln!(s, "    /* talents: PreviewTalent[amount_of_talents] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "talent", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "rank", "        ");
                writeln!(s, "    /* talents: PreviewTalent[amount_of_talents] {i} end */").unwrap();
            }
            writeln!(s, "    /* talents: PreviewTalent[amount_of_talents] end */").unwrap();
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
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // amount_of_talents: u32
        w.write_all(&(self.talents.len() as u32).to_le_bytes())?;

        // talents: PreviewTalent[amount_of_talents]
        for i in self.talents.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1218, "CMSG_LEARN_PREVIEW_TALENTS_PET", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LEARN_PREVIEW_TALENTS_PET {}

impl CMSG_LEARN_PREVIEW_TALENTS_PET {
    pub(crate) fn size(&self) -> usize {
        8 // pet: Guid
        + 4 // amount_of_talents: u32
        + self.talents.len() * 8 // talents: PreviewTalent[amount_of_talents]
    }
}

