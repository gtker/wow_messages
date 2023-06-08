use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    DamageInfo, HitInfo,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm:50`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm#L50):
/// ```text
/// smsg SMSG_ATTACKERSTATEUPDATE = 0x014A {
///     HitInfo hit_info;
///     PackedGuid attacker;
///     PackedGuid target;
///     u32 total_damage;
///     u8 amount_of_damages;
///     DamageInfo[amount_of_damages] damages;
///     u32 damage_state;
///     u32 unknown1;
///     u32 spell_id;
///     u32 blocked_amount;
/// }
/// ```
pub struct SMSG_ATTACKERSTATEUPDATE {
    pub hit_info: HitInfo,
    pub attacker: Guid,
    pub target: Guid,
    pub total_damage: u32,
    pub damages: Vec<DamageInfo>,
    pub damage_state: u32,
    pub unknown1: u32,
    /// vmangos: spell id, seen with heroic strike and disarm as examples
    pub spell_id: u32,
    pub blocked_amount: u32,
}

impl crate::private::Sealed for SMSG_ATTACKERSTATEUPDATE {}
impl crate::Message for SMSG_ATTACKERSTATEUPDATE {
    const OPCODE: u32 = 0x014a;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ATTACKERSTATEUPDATE {{").unwrap();
        // Members
        writeln!(s, "    hit_info = {};", self.hit_info.as_test_case_value()).unwrap();
        writeln!(s, "    attacker = {};", self.attacker.guid()).unwrap();
        writeln!(s, "    target = {};", self.target.guid()).unwrap();
        writeln!(s, "    total_damage = {};", self.total_damage).unwrap();
        writeln!(s, "    amount_of_damages = {};", self.damages.len()).unwrap();
        write!(s, "    damages = [").unwrap();
        for v in self.damages.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        spell_school_mask = {};", v.spell_school_mask).unwrap();
            writeln!(s, "    {}", if v.damage_float.to_string().contains(".") { v.damage_float.to_string() } else { format!("{}.0", v.damage_float) }).unwrap();
            writeln!(s, "        damage_uint = {};", v.damage_uint).unwrap();
            writeln!(s, "        absorb = {};", v.absorb).unwrap();
            writeln!(s, "        resist = {};", v.resist).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    damage_state = {};", self.damage_state).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    spell_id = {};", self.spell_id).unwrap();
        writeln!(s, "    blocked_amount = {};", self.blocked_amount).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 330_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "hit_info", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.attacker), "attacker", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.target), "target", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "total_damage", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_damages", "    ");
        if !self.damages.is_empty() {
            writeln!(s, "    /* damages: DamageInfo[amount_of_damages] start */").unwrap();
            for (i, v) in self.damages.iter().enumerate() {
                writeln!(s, "    /* damages: DamageInfo[amount_of_damages] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_school_mask", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "damage_float", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "damage_uint", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "absorb", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "resist", "        ");
                writeln!(s, "    /* damages: DamageInfo[amount_of_damages] {i} end */").unwrap();
            }
            writeln!(s, "    /* damages: DamageInfo[amount_of_damages] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage_state", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "blocked_amount", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // hit_info: HitInfo
        w.write_all(&(self.hit_info.as_int().to_le_bytes()))?;

        // attacker: PackedGuid
        crate::util::write_packed_guid(&self.attacker, &mut w)?;

        // target: PackedGuid
        crate::util::write_packed_guid(&self.target, &mut w)?;

        // total_damage: u32
        w.write_all(&self.total_damage.to_le_bytes())?;

        // amount_of_damages: u8
        w.write_all(&(self.damages.len() as u8).to_le_bytes())?;

        // damages: DamageInfo[amount_of_damages]
        for i in self.damages.iter() {
            i.write_into_vec(&mut w)?;
        }

        // damage_state: u32
        w.write_all(&self.damage_state.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // spell_id: u32
        w.write_all(&self.spell_id.to_le_bytes())?;

        // blocked_amount: u32
        w.write_all(&self.blocked_amount.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(29..=5163).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x014A, size: body_size });
        }

        // hit_info: HitInfo
        let hit_info = crate::util::read_u32_le(&mut r)?.try_into()?;

        // attacker: PackedGuid
        let attacker = crate::util::read_packed_guid(&mut r)?;

        // target: PackedGuid
        let target = crate::util::read_packed_guid(&mut r)?;

        // total_damage: u32
        let total_damage = crate::util::read_u32_le(&mut r)?;

        // amount_of_damages: u8
        let amount_of_damages = crate::util::read_u8_le(&mut r)?;

        // damages: DamageInfo[amount_of_damages]
        let damages = {
            let mut damages = Vec::with_capacity(amount_of_damages as usize);
            for _ in 0..amount_of_damages {
                damages.push(DamageInfo::read(&mut r)?);
            }
            damages
        };

        // damage_state: u32
        let damage_state = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // spell_id: u32
        let spell_id = crate::util::read_u32_le(&mut r)?;

        // blocked_amount: u32
        let blocked_amount = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            hit_info,
            attacker,
            target,
            total_damage,
            damages,
            damage_state,
            unknown1,
            spell_id,
            blocked_amount,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ATTACKERSTATEUPDATE {}

impl SMSG_ATTACKERSTATEUPDATE {
    pub(crate) fn size(&self) -> usize {
        4 // hit_info: HitInfo
        + crate::util::packed_guid_size(&self.attacker) // attacker: PackedGuid
        + crate::util::packed_guid_size(&self.target) // target: PackedGuid
        + 4 // total_damage: u32
        + 1 // amount_of_damages: u8
        + self.damages.len() * 20 // damages: DamageInfo[amount_of_damages]
        + 4 // damage_state: u32
        + 4 // unknown1: u32
        + 4 // spell_id: u32
        + 4 // blocked_amount: u32
    }
}

