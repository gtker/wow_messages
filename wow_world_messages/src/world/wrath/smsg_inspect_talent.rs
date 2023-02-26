use crate::Guid;
use crate::wrath::InspectTalentGearMask;
use crate::wrath::InspectTalentSpec;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm:33`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm#L33):
/// ```text
/// smsg SMSG_INSPECT_TALENT = 0x03F4 {
///     PackedGuid player;
///     u32 unspent_talent_points;
///     u8 amount_of_specs;
///     u8 active_spec;
///     InspectTalentSpec[amount_of_specs] specs;
///     u8 amount_of_glyphs;
///     u16[amount_of_glyphs] glyphs;
///     InspectTalentGearMask talent_gear_mask;
/// }
/// ```
pub struct SMSG_INSPECT_TALENT {
    pub player: Guid,
    pub unspent_talent_points: u32,
    pub active_spec: u8,
    pub specs: Vec<InspectTalentSpec>,
    pub glyphs: Vec<u16>,
    pub talent_gear_mask: InspectTalentGearMask,
}

impl crate::Message for SMSG_INSPECT_TALENT {
    const OPCODE: u32 = 0x03f4;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w)?;

        // unspent_talent_points: u32
        w.write_all(&self.unspent_talent_points.to_le_bytes())?;

        // amount_of_specs: u8
        w.write_all(&(self.specs.len() as u8).to_le_bytes())?;

        // active_spec: u8
        w.write_all(&self.active_spec.to_le_bytes())?;

        // specs: InspectTalentSpec[amount_of_specs]
        for i in self.specs.iter() {
            i.write_into_vec(w)?;
        }

        // amount_of_glyphs: u8
        w.write_all(&(self.glyphs.len() as u8).to_le_bytes())?;

        // glyphs: u16[amount_of_glyphs]
        for i in self.glyphs.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // talent_gear_mask: InspectTalentGearMask
        self.talent_gear_mask.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(13..=330164).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03F4, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // unspent_talent_points: u32
        let unspent_talent_points = crate::util::read_u32_le(r)?;

        // amount_of_specs: u8
        let amount_of_specs = crate::util::read_u8_le(r)?;

        // active_spec: u8
        let active_spec = crate::util::read_u8_le(r)?;

        // specs: InspectTalentSpec[amount_of_specs]
        let specs = {
            let mut specs = Vec::with_capacity(amount_of_specs as usize);
            for i in 0..amount_of_specs {
                specs.push(InspectTalentSpec::read(r)?);
            }
            specs
        };
        // amount_of_glyphs: u8
        let amount_of_glyphs = crate::util::read_u8_le(r)?;

        // glyphs: u16[amount_of_glyphs]
        let glyphs = {
            let mut glyphs = Vec::with_capacity(amount_of_glyphs as usize);
            for i in 0..amount_of_glyphs {
                glyphs.push(crate::util::read_u16_le(r)?);
            }
            glyphs
        };
        // talent_gear_mask: InspectTalentGearMask
        let talent_gear_mask = InspectTalentGearMask::read(r)?;

        Ok(Self {
            player,
            unspent_talent_points,
            active_spec,
            specs,
            glyphs,
            talent_gear_mask,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_INSPECT_TALENT {}

impl SMSG_INSPECT_TALENT {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + 4 // unspent_talent_points: u32
        + 1 // amount_of_specs: u8
        + 1 // active_spec: u8
        + self.specs.iter().fold(0, |acc, x| acc + x.size()) // specs: InspectTalentSpec[amount_of_specs]
        + 1 // amount_of_glyphs: u8
        + self.glyphs.len() * core::mem::size_of::<u16>() // glyphs: u16[amount_of_glyphs]
        + self.talent_gear_mask.size() // talent_gear_mask: InspectTalentGearMask
    }
}

