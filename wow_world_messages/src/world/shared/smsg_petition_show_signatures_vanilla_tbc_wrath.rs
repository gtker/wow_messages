use std::io::{Read, Write};

use crate::Guid;
use crate::shared::petition_signature_vanilla_tbc_wrath::PetitionSignature;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_show_signatures.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_show_signatures.wowm#L8):
/// ```text
/// smsg SMSG_PETITION_SHOW_SIGNATURES = 0x01BF {
///     Guid item;
///     Guid owner;
///     u32 petition;
///     u8 amount_of_signatures;
///     PetitionSignature[amount_of_signatures] signatures;
/// }
/// ```
pub struct SMSG_PETITION_SHOW_SIGNATURES {
    pub item: Guid,
    pub owner: Guid,
    pub petition: u32,
    pub signatures: Vec<PetitionSignature>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_PETITION_SHOW_SIGNATURES {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PETITION_SHOW_SIGNATURES {{").unwrap();
        // Members
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        writeln!(s, "    owner = {};", self.owner.guid()).unwrap();
        writeln!(s, "    petition = {};", self.petition).unwrap();
        writeln!(s, "    amount_of_signatures = {};", self.signatures.len()).unwrap();
        write!(s, "    signatures = [").unwrap();
        for v in self.signatures.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        signer = {};", v.signer.guid()).unwrap();
            writeln!(s, "        unknown1 = {};", v.unknown1).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 447_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "owner", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "petition", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_signatures", "    ");
        if !self.signatures.is_empty() {
            writeln!(s, "    /* signatures: PetitionSignature[amount_of_signatures] start */").unwrap();
            for (i, v) in self.signatures.iter().enumerate() {
                writeln!(s, "    /* signatures: PetitionSignature[amount_of_signatures] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "signer", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "        ");
                writeln!(s, "    /* signatures: PetitionSignature[amount_of_signatures] {i} end */").unwrap();
            }
            writeln!(s, "    /* signatures: PetitionSignature[amount_of_signatures] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_PETITION_SHOW_SIGNATURES {}
impl crate::Message for SMSG_PETITION_SHOW_SIGNATURES {
    const OPCODE: u32 = 0x01bf;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_PETITION_SHOW_SIGNATURES::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // owner: Guid
        w.write_all(&self.owner.guid().to_le_bytes())?;

        // petition: u32
        w.write_all(&self.petition.to_le_bytes())?;

        // amount_of_signatures: u8
        w.write_all(&(self.signatures.len() as u8).to_le_bytes())?;

        // signatures: PetitionSignature[amount_of_signatures]
        for i in self.signatures.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(21..=3093).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01BF, size: body_size });
        }

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // owner: Guid
        let owner = crate::util::read_guid(&mut r)?;

        // petition: u32
        let petition = crate::util::read_u32_le(&mut r)?;

        // amount_of_signatures: u8
        let amount_of_signatures = crate::util::read_u8_le(&mut r)?;

        // signatures: PetitionSignature[amount_of_signatures]
        let signatures = {
            let mut signatures = Vec::with_capacity(amount_of_signatures as usize);
            for _ in 0..amount_of_signatures {
                signatures.push(PetitionSignature::read(&mut r)?);
            }
            signatures
        };

        Ok(Self {
            item,
            owner,
            petition,
            signatures,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PETITION_SHOW_SIGNATURES {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PETITION_SHOW_SIGNATURES {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PETITION_SHOW_SIGNATURES {}

impl SMSG_PETITION_SHOW_SIGNATURES {
    pub(crate) fn size(&self) -> usize {
        8 // item: Guid
        + 8 // owner: Guid
        + 4 // petition: u32
        + 1 // amount_of_signatures: u8
        + self.signatures.len() * 12 // signatures: PetitionSignature[amount_of_signatures]
    }
}

