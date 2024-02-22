use crate::errors::{ExpectedOpcodeError, ParseError};
use crate::util::{read_i32_le, read_u32_le, read_u8_le};
use std::io::{Read, Write};
use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ServerHeader {
    pub size: u32,
    pub opcode: u16,
}

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
impl ServerHeader {
    pub(crate) const fn from_array(b: [u8; 4]) -> Self {
        let size = u32::from_be_bytes([0, 0, b[0], b[1]]);
        let opcode = u16::from_le_bytes([b[2], b[3]]);

        Self { size, opcode }
    }

    #[cfg(feature = "wrath")]
    pub(crate) const fn from_large_array(b: [u8; 5]) -> Self {
        let size = u32::from_be_bytes([0, b[0], b[1], b[2]]);
        let opcode = u16::from_le_bytes([b[3], b[4]]);

        Self { size, opcode }
    }
}

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ClientHeader {
    pub size: u16,
    pub opcode: u32,
}

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
impl ClientHeader {
    pub(crate) const fn from_array(b: [u8; 6]) -> Self {
        let size: u16 = u16::from_be_bytes([b[0], b[1]]);
        let opcode: u32 = u32::from_le_bytes([b[2], b[3], b[4], b[5]]);

        Self { size, opcode }
    }
}

#[cfg(any(feature = "wrath", feature = "tbc"))]
pub(crate) fn write_addon_array(
    addons: &[crate::shared::addon_tbc_wrath::Addon],
    mut w: impl Write,
) -> Result<(), std::io::Error> {
    for addon in addons {
        addon.write_into_vec(&mut w)?;
    }

    Ok(())
}

pub(crate) fn vector3d_to_packed(v: &Vector3d) -> i32 {
    let mut packed = 0;

    packed |= (v.x / 0.25) as i32 & 0x7FF;
    packed |= ((v.y / 0.25) as i32 & 0x7FF) << 11;
    packed |= ((v.z / 0.25) as i32 & 0x3FF) << 22;

    packed
}

pub(crate) const fn packed_to_vector3d(p: i32) -> Vector3d {
    let x = ((p & 0x7FF) / 4) as f32;
    let y = (((p >> 11) & 0x7FF) / 4) as f32;
    let z = (((p >> 22) & 0x3FF) / 4) as f32;

    Vector3d { x, y, z }
}

pub(crate) fn read_monster_move_spline(
    mut r: &mut impl Read,
) -> Result<Vec<Vector3d>, crate::errors::ParseErrorKind> {
    let amount_of_splines = read_u32_le(&mut r)?;
    let mut splines = Vec::with_capacity(amount_of_splines.try_into().unwrap());

    for i in 0..amount_of_splines {
        if i == 0 {
            let vec = vanilla_tbc_wrath_vector3d_read(&mut r)?;
            splines.push(vec);
        } else {
            let packed = read_i32_le(&mut r)?;
            splines.push(packed_to_vector3d(packed));
        }
    }

    Ok(splines)
}

pub(crate) fn write_monster_move_spline(
    splines: &[Vector3d],
    mut v: impl Write,
) -> Result<(), std::io::Error> {
    let amount_of_splines: u32 = splines.len().try_into().unwrap();

    v.write_all(&amount_of_splines.to_le_bytes())?;

    let mut splines = splines.iter();
    if let Some(first) = splines.next() {
        vanilla_tbc_wrath_vector3d_write_into_vec(first, &mut v)?;
    }

    for spline in splines {
        v.write_all(&vector3d_to_packed(spline).to_le_bytes())?;
    }

    Ok(())
}

pub(crate) fn monster_move_spline_size(splines: &[Vector3d]) -> usize {
    let mut splines = splines.iter();

    let mut size = core::mem::size_of::<u32>(); // amount_of_splines: u32
    if splines.next().is_some() {
        size += 3 * core::mem::size_of::<f32>(); // First member is Vector3d
    }
    for _ in splines {
        size += core::mem::size_of::<u32>(); // All other members are packed u32
    }

    size
}

pub const fn packed_guid_size(guid: &crate::Guid) -> usize {
    let mut amount_of_bytes = 1;

    let mut i = 0;
    while i < 8 {
        if (guid.guid() & (0xFF << (i * 8))) != 0 {
            amount_of_bytes += 1;
        }

        i += 1;
    }

    amount_of_bytes
}

pub fn write_packed_guid(guid: &crate::Guid, mut v: impl Write) -> Result<(), std::io::Error> {
    let guid = guid.guid().to_le_bytes();
    let mut bit_pattern: u8 = 0;

    let mut placeholder = [0_u8; 9];
    let mut index = 1;
    for (i, &b) in guid.iter().enumerate() {
        if b != 0 {
            bit_pattern |= 1 << i;
            placeholder[index] = b;
            index += 1;
        }
    }

    placeholder[0] = bit_pattern;

    v.write_all(&placeholder[0..index])
}

pub fn read_guid(r: &mut impl Read) -> Result<crate::Guid, std::io::Error> {
    Ok(crate::Guid::new(crate::util::read_u64_le(r)?))
}

pub fn read_packed_guid(r: &mut impl Read) -> Result<crate::Guid, std::io::Error> {
    let bit_pattern = read_u8_le(r)?;
    let mut guid: u64 = 0;

    for index in 0..8 {
        let bit = bit_pattern & (1 << index);

        if bit != 0 {
            let byte = read_u8_le(r)?;
            guid |= (byte as u64) << (index * 8);
        }
    }

    Ok(crate::Guid::new(guid))
}

pub(crate) fn assert_empty(
    body_size: u32,
    opcode: impl Into<u32>,
    message: &'static str,
) -> Result<(), ExpectedOpcodeError> {
    let opcode: u32 = opcode.into();
    if body_size != 0 {
        Err(ExpectedOpcodeError::Parse(ParseError::new(
            opcode,
            message,
            body_size,
            crate::errors::ParseErrorKind::InvalidSize,
        )))
    } else {
        Ok(())
    }
}

pub fn zlib_compressed_size(data: &[u8]) -> usize {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder
        .write_all(data)
        .expect("Failed to compress data when calculating message size.");
    let compressed_data = encoder
        .finish()
        .expect("Failed to flush compressed data when calculating message size.");
    compressed_data.len()
}

// AUTOGENERATED_START
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) fn vanilla_tbc_wrath_itemdamagetype_read<R: std::io::Read>(mut r: R) -> Result<wow_world_base::shared::item_damage_type_vanilla_tbc_wrath::ItemDamageType, crate::errors::ParseErrorKind> {
    // damage_minimum: f32
    let damage_minimum = crate::util::read_f32_le(&mut r)?;

    // damage_maximum: f32
    let damage_maximum = crate::util::read_f32_le(&mut r)?;

    // school: SpellSchool
    let school = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

    Ok(wow_world_base::shared::item_damage_type_vanilla_tbc_wrath::ItemDamageType {
        damage_minimum,
        damage_maximum,
        school,
    })
}

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) fn vanilla_tbc_wrath_itemdamagetype_write_into_vec(s: &wow_world_base::shared::item_damage_type_vanilla_tbc_wrath::ItemDamageType, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
    // damage_minimum: f32
    w.write_all(&s.damage_minimum.to_le_bytes())?;

    // damage_maximum: f32
    w.write_all(&s.damage_maximum.to_le_bytes())?;

    // school: SpellSchool
    w.write_all(&u32::from(s.school.as_int()).to_le_bytes())?;

    Ok(())
}

#[cfg(any(feature = "tbc", feature = "wrath"))]
pub(crate) fn tbc_wrath_itemsocket_read<R: std::io::Read>(mut r: R) -> Result<wow_world_base::shared::item_socket_tbc_wrath::ItemSocket, std::io::Error> {
    // color: u32
    let color = crate::util::read_u32_le(&mut r)?;

    // content: u32
    let content = crate::util::read_u32_le(&mut r)?;

    Ok(wow_world_base::shared::item_socket_tbc_wrath::ItemSocket {
        color,
        content,
    })
}

#[cfg(any(feature = "tbc", feature = "wrath"))]
pub(crate) fn tbc_wrath_itemsocket_write_into_vec(s: &wow_world_base::shared::item_socket_tbc_wrath::ItemSocket, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
    // color: u32
    w.write_all(&s.color.to_le_bytes())?;

    // content: u32
    w.write_all(&s.content.to_le_bytes())?;

    Ok(())
}

#[cfg(feature = "vanilla")]
pub(crate) fn vanilla_itemspells_read<R: std::io::Read>(mut r: R) -> Result<wow_world_base::vanilla::ItemSpells, crate::errors::ParseErrorKind> {
    // spell: Spell
    let spell = crate::util::read_u32_le(&mut r)?;

    // spell_trigger: SpellTriggerType
    let spell_trigger = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

    // spell_charges: i32
    let spell_charges = crate::util::read_i32_le(&mut r)?;

    // spell_cooldown: i32
    let spell_cooldown = crate::util::read_i32_le(&mut r)?;

    // spell_category: u32
    let spell_category = crate::util::read_u32_le(&mut r)?;

    // spell_category_cooldown: i32
    let spell_category_cooldown = crate::util::read_i32_le(&mut r)?;

    Ok(wow_world_base::vanilla::ItemSpells {
        spell,
        spell_trigger,
        spell_charges,
        spell_cooldown,
        spell_category,
        spell_category_cooldown,
    })
}

#[cfg(feature = "vanilla")]
pub(crate) fn vanilla_itemspells_write_into_vec(s: &wow_world_base::vanilla::ItemSpells, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
    // spell: Spell
    w.write_all(&s.spell.to_le_bytes())?;

    // spell_trigger: SpellTriggerType
    w.write_all(&u32::from(s.spell_trigger.as_int()).to_le_bytes())?;

    // spell_charges: i32
    w.write_all(&s.spell_charges.to_le_bytes())?;

    // spell_cooldown: i32
    w.write_all(&s.spell_cooldown.to_le_bytes())?;

    // spell_category: u32
    w.write_all(&s.spell_category.to_le_bytes())?;

    // spell_category_cooldown: i32
    w.write_all(&s.spell_category_cooldown.to_le_bytes())?;

    Ok(())
}

#[cfg(any(feature = "tbc", feature = "wrath"))]
pub(crate) fn tbc_wrath_itemspells_read<R: std::io::Read>(mut r: R) -> Result<wow_world_base::shared::item_spells_tbc_wrath::ItemSpells, crate::errors::ParseErrorKind> {
    // spell: Spell
    let spell = crate::util::read_u32_le(&mut r)?;

    // spell_trigger: SpellTriggerType
    let spell_trigger = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

    // spell_charges: i32
    let spell_charges = crate::util::read_i32_le(&mut r)?;

    // spell_cooldown: i32
    let spell_cooldown = crate::util::read_i32_le(&mut r)?;

    // spell_category: u32
    let spell_category = crate::util::read_u32_le(&mut r)?;

    // spell_category_cooldown: i32
    let spell_category_cooldown = crate::util::read_i32_le(&mut r)?;

    Ok(wow_world_base::shared::item_spells_tbc_wrath::ItemSpells {
        spell,
        spell_trigger,
        spell_charges,
        spell_cooldown,
        spell_category,
        spell_category_cooldown,
    })
}

#[cfg(any(feature = "tbc", feature = "wrath"))]
pub(crate) fn tbc_wrath_itemspells_write_into_vec(s: &wow_world_base::shared::item_spells_tbc_wrath::ItemSpells, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
    // spell: Spell
    w.write_all(&s.spell.to_le_bytes())?;

    // spell_trigger: SpellTriggerType
    w.write_all(&u32::from(s.spell_trigger.as_int()).to_le_bytes())?;

    // spell_charges: i32
    w.write_all(&s.spell_charges.to_le_bytes())?;

    // spell_cooldown: i32
    w.write_all(&s.spell_cooldown.to_le_bytes())?;

    // spell_category: u32
    w.write_all(&s.spell_category.to_le_bytes())?;

    // spell_category_cooldown: i32
    w.write_all(&s.spell_category_cooldown.to_le_bytes())?;

    Ok(())
}

#[cfg(feature = "vanilla")]
pub(crate) fn vanilla_itemstat_read<R: std::io::Read>(mut r: R) -> Result<wow_world_base::vanilla::ItemStat, crate::errors::ParseErrorKind> {
    // stat_type: ItemStatType
    let stat_type = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

    // value: i32
    let value = crate::util::read_i32_le(&mut r)?;

    Ok(wow_world_base::vanilla::ItemStat {
        stat_type,
        value,
    })
}

#[cfg(feature = "vanilla")]
pub(crate) fn vanilla_itemstat_write_into_vec(s: &wow_world_base::vanilla::ItemStat, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
    // stat_type: ItemStatType
    w.write_all(&u32::from(s.stat_type.as_int()).to_le_bytes())?;

    // value: i32
    w.write_all(&s.value.to_le_bytes())?;

    Ok(())
}

#[cfg(any(feature = "tbc", feature = "wrath"))]
pub(crate) fn tbc_wrath_itemstat_read<R: std::io::Read>(mut r: R) -> Result<wow_world_base::shared::item_stat_tbc_wrath::ItemStat, std::io::Error> {
    // stat_type: u32
    let stat_type = crate::util::read_u32_le(&mut r)?;

    // value: i32
    let value = crate::util::read_i32_le(&mut r)?;

    Ok(wow_world_base::shared::item_stat_tbc_wrath::ItemStat {
        stat_type,
        value,
    })
}

#[cfg(any(feature = "tbc", feature = "wrath"))]
pub(crate) fn tbc_wrath_itemstat_write_into_vec(s: &wow_world_base::shared::item_stat_tbc_wrath::ItemStat, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
    // stat_type: u32
    w.write_all(&s.stat_type.to_le_bytes())?;

    // value: i32
    w.write_all(&s.value.to_le_bytes())?;

    Ok(())
}

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) fn vanilla_tbc_wrath_vector2d_read<R: std::io::Read>(mut r: R) -> Result<wow_world_base::shared::vector2d_vanilla_tbc_wrath::Vector2d, std::io::Error> {
    // x: f32
    let x = crate::util::read_f32_le(&mut r)?;

    // y: f32
    let y = crate::util::read_f32_le(&mut r)?;

    Ok(wow_world_base::shared::vector2d_vanilla_tbc_wrath::Vector2d {
        x,
        y,
    })
}

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) fn vanilla_tbc_wrath_vector2d_write_into_vec(s: &wow_world_base::shared::vector2d_vanilla_tbc_wrath::Vector2d, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
    // x: f32
    w.write_all(&s.x.to_le_bytes())?;

    // y: f32
    w.write_all(&s.y.to_le_bytes())?;

    Ok(())
}

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) fn vanilla_tbc_wrath_vector3d_read<R: std::io::Read>(mut r: R) -> Result<wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d, std::io::Error> {
    // x: f32
    let x = crate::util::read_f32_le(&mut r)?;

    // y: f32
    let y = crate::util::read_f32_le(&mut r)?;

    // z: f32
    let z = crate::util::read_f32_le(&mut r)?;

    Ok(wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d {
        x,
        y,
        z,
    })
}

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) fn vanilla_tbc_wrath_vector3d_write_into_vec(s: &wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
    // x: f32
    w.write_all(&s.x.to_le_bytes())?;

    // y: f32
    w.write_all(&s.y.to_le_bytes())?;

    // z: f32
    w.write_all(&s.z.to_le_bytes())?;

    Ok(())
}

// AUTOGENERATED_END
