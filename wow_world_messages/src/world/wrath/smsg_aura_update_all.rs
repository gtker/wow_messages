use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::AuraUpdate;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_aura_update_all.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_aura_update_all.wowm#L35):
/// ```text
/// smsg SMSG_AURA_UPDATE_ALL = 0x0495 {
///     PackedGuid unit;
///     AuraUpdate[-] aura_updates;
/// }
/// ```
pub struct SMSG_AURA_UPDATE_ALL {
    pub unit: Guid,
    pub aura_updates: Vec<AuraUpdate>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_AURA_UPDATE_ALL {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AURA_UPDATE_ALL {{").unwrap();
        // Members
        writeln!(s, "    unit = {};", self.unit.guid()).unwrap();
        write!(s, "    aura_updates = [").unwrap();
        for v in self.aura_updates.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        visual_slot = {};", v.visual_slot).unwrap();
            writeln!(s, "        spell = {};", v.spell).unwrap();
            writeln!(s, "        flags = {};", crate::wrath::AuraFlag::new(v.flags.as_int()).as_test_case_value()).unwrap();
            writeln!(s, "        level = {};", v.level.as_int()).unwrap();
            writeln!(s, "        aura_stack_count = {};", v.aura_stack_count).unwrap();
            if let Some(if_statement) = &v.flags.get_not_caster() {
                writeln!(s, "        caster = {};", if_statement.caster.guid()).unwrap();
            }

            if let Some(if_statement) = &v.flags.get_duration() {
                writeln!(s, "        duration = {};", if_statement.duration).unwrap();
                writeln!(s, "        time_left = {};", if_statement.time_left).unwrap();
            }


            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1173_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.unit), "unit", "    ");
        if !self.aura_updates.is_empty() {
            writeln!(s, "    /* aura_updates: AuraUpdate[-] start */").unwrap();
            for (i, v) in self.aura_updates.iter().enumerate() {
                writeln!(s, "    /* aura_updates: AuraUpdate[-] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "visual_slot", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "flags", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "level", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "aura_stack_count", "        ");
                if let Some(if_statement) = &v.flags.get_not_caster() {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.caster), "caster", "        ");
                }

                if let Some(if_statement) = &v.flags.get_duration() {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "time_left", "        ");
                }

                writeln!(s, "    /* aura_updates: AuraUpdate[-] {i} end */").unwrap();
            }
            writeln!(s, "    /* aura_updates: AuraUpdate[-] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_AURA_UPDATE_ALL {}
impl crate::Message for SMSG_AURA_UPDATE_ALL {
    const OPCODE: u32 = 0x0495;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_AURA_UPDATE_ALL::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

        // aura_updates: AuraUpdate[-]
        for i in self.aura_updates.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=65544).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0495, size: body_size });
        }

        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

        // aura_updates: AuraUpdate[-]
        let aura_updates = {
            let mut current_size = {
                crate::util::packed_guid_size(&unit) // unit: PackedGuid
            };
            let mut aura_updates = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                aura_updates.push(AuraUpdate::read(&mut r)?);
                current_size += 1;
            }
            aura_updates
        };

        Ok(Self {
            unit,
            aura_updates,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AURA_UPDATE_ALL {}

impl SMSG_AURA_UPDATE_ALL {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
        + self.aura_updates.iter().fold(0, |acc, x| acc + x.size()) // aura_updates: AuraUpdate[-]
    }
}

