use crate:: {
    Guid,
};
use crate::vanilla::AuraLog;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L1):
/// ```text
/// smsg SMSG_PERIODICAURALOG = 0x024E {
///     PackedGuid target;
///     PackedGuid caster;
///     u32 spell;
///     u32 amount_of_auras;
///     AuraLog[amount_of_auras] auras;
/// }
/// ```
pub struct SMSG_PERIODICAURALOG {
    pub target: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub auras: Vec<AuraLog>,
}

impl crate::Message for SMSG_PERIODICAURALOG {
    const OPCODE: u32 = 0x024e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // target: PackedGuid
        self.target.write_packed_guid_into_vec(&mut w)?;

        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(&mut w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_auras: u32
        w.write_all(&(self.auras.len() as u32).to_le_bytes())?;

        // auras: AuraLog[amount_of_auras]
        for i in self.auras.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(12..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x024E, size: body_size as u32 });
        }

        // target: PackedGuid
        let target = Guid::read_packed(&mut r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // amount_of_auras: u32
        let amount_of_auras = crate::util::read_u32_le(&mut r)?;

        // auras: AuraLog[amount_of_auras]
        let auras = {
            let mut auras = Vec::with_capacity(amount_of_auras as usize);
            for i in 0..amount_of_auras {
                auras.push(AuraLog::read(&mut r)?);
            }
            auras
        };

        Ok(Self {
            target,
            caster,
            spell,
            auras,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PERIODICAURALOG {}

impl SMSG_PERIODICAURALOG {
    pub(crate) fn size(&self) -> usize {
        self.target.size() // target: Guid
        + self.caster.size() // caster: Guid
        + 4 // spell: u32
        + 4 // amount_of_auras: u32
        + self.auras.iter().fold(0, |acc, x| acc + x.size()) // auras: AuraLog[amount_of_auras]
    }
}

