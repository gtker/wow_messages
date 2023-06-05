use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    InspectTalentGearMask, InspectTalentSpec,
};

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

#[cfg(feature = "print-testcase")]
impl SMSG_INSPECT_TALENT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_INSPECT_TALENT {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    unspent_talent_points = {};", self.unspent_talent_points).unwrap();
        writeln!(s, "    amount_of_specs = {};", self.specs.len()).unwrap();
        writeln!(s, "    active_spec = {};", self.active_spec).unwrap();
        write!(s, "    specs = [").unwrap();
        for v in self.specs.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        amount_of_talents = {};", v.talents.len()).unwrap();
            write!(s, "        talents = [").unwrap();
            for v in v.talents.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "            talent = {};", v.talent.as_test_case_value()).unwrap();
                writeln!(s, "            max_rank = {};", v.max_rank).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    amount_of_glyphs = {};", self.glyphs.len()).unwrap();
        write!(s, "    glyphs = [").unwrap();
        for v in self.glyphs.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();
        return None;

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1012_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.player), "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unspent_talent_points", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_specs", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "active_spec", "    ");
        if !self.specs.is_empty() {
            writeln!(s, "    /* specs: InspectTalentSpec[amount_of_specs] start */").unwrap();
            for (i, v) in self.specs.iter().enumerate() {
                writeln!(s, "    /* specs: InspectTalentSpec[amount_of_specs] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_talents", "        ");
                if !v.talents.is_empty() {
                    writeln!(s, "    /* talents: InspectTalent[amount_of_talents] start */").unwrap();
                    for (i, v) in v.talents.iter().enumerate() {
                        writeln!(s, "    /* talents: InspectTalent[amount_of_talents] {i} start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "talent", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "max_rank", "            ");
                        writeln!(s, "    /* talents: InspectTalent[amount_of_talents] {i} end */").unwrap();
                    }
                    writeln!(s, "    /* talents: InspectTalent[amount_of_talents] end */").unwrap();
                }
                writeln!(s, "    /* specs: InspectTalentSpec[amount_of_specs] {i} end */").unwrap();
            }
            writeln!(s, "    /* specs: InspectTalentSpec[amount_of_specs] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_glyphs", "    ");
        if !self.glyphs.is_empty() {
            writeln!(s, "    /* glyphs: u16[amount_of_glyphs] start */").unwrap();
            for (i, v) in self.glyphs.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 2, &format!("glyphs {i}"), "    ");
            }
            writeln!(s, "    /* glyphs: u16[amount_of_glyphs] end */").unwrap();
        }
        panic!("unsupported type InspectTalentGearMask for variable 'talent_gear_mask'");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_INSPECT_TALENT {}
impl crate::Message for SMSG_INSPECT_TALENT {
    const OPCODE: u32 = 0x03f4;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_INSPECT_TALENT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        crate::util::write_packed_guid(&self.player, &mut w)?;

        // unspent_talent_points: u32
        w.write_all(&self.unspent_talent_points.to_le_bytes())?;

        // amount_of_specs: u8
        w.write_all(&(self.specs.len() as u8).to_le_bytes())?;

        // active_spec: u8
        w.write_all(&self.active_spec.to_le_bytes())?;

        // specs: InspectTalentSpec[amount_of_specs]
        for i in self.specs.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_glyphs: u8
        w.write_all(&(self.glyphs.len() as u8).to_le_bytes())?;

        // glyphs: u16[amount_of_glyphs]
        for i in self.glyphs.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // talent_gear_mask: InspectTalentGearMask
        self.talent_gear_mask.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(13..=330164).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03F4, size: body_size });
        }

        // player: PackedGuid
        let player = crate::util::read_packed_guid(&mut r)?;

        // unspent_talent_points: u32
        let unspent_talent_points = crate::util::read_u32_le(&mut r)?;

        // amount_of_specs: u8
        let amount_of_specs = crate::util::read_u8_le(&mut r)?;

        // active_spec: u8
        let active_spec = crate::util::read_u8_le(&mut r)?;

        // specs: InspectTalentSpec[amount_of_specs]
        let specs = {
            let mut specs = Vec::with_capacity(amount_of_specs as usize);
            for _ in 0..amount_of_specs {
                specs.push(InspectTalentSpec::read(&mut r)?);
            }
            specs
        };

        // amount_of_glyphs: u8
        let amount_of_glyphs = crate::util::read_u8_le(&mut r)?;

        // glyphs: u16[amount_of_glyphs]
        let glyphs = {
            let mut glyphs = Vec::with_capacity(amount_of_glyphs as usize);
            for _ in 0..amount_of_glyphs {
                glyphs.push(crate::util::read_u16_le(&mut r)?);
            }
            glyphs
        };

        // talent_gear_mask: InspectTalentGearMask
        let talent_gear_mask = InspectTalentGearMask::read(&mut r)?;

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
        crate::util::packed_guid_size(&self.player) // player: PackedGuid
        + 4 // unspent_talent_points: u32
        + 1 // amount_of_specs: u8
        + 1 // active_spec: u8
        + self.specs.iter().fold(0, |acc, x| acc + x.size()) // specs: InspectTalentSpec[amount_of_specs]
        + 1 // amount_of_glyphs: u8
        + self.glyphs.len() * core::mem::size_of::<u16>() // glyphs: u16[amount_of_glyphs]
        + self.talent_gear_mask.size() // talent_gear_mask: InspectTalentGearMask
    }
}

