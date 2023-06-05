use crate::errors::ExpectedOpcodeError;
use crate::util::{read_u32_le, read_u8_le};
use std::io::{Read, Write};
use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

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

pub(crate) fn vector3d_to_packed(v: &Vector3d) -> u32 {
    let mut packed = 0;

    packed |= (v.x / 0.25) as u32 & 0x7FF;
    packed |= ((v.y / 0.25) as u32 & 0x7FF) << 11;
    packed |= ((v.z / 0.25) as u32 & 0x3FF) << 22;

    packed
}

pub(crate) const fn packed_to_vector3d(p: u32) -> Vector3d {
    let x = ((p & 0x7FF) / 4) as f32;
    let y = (((p >> 11) & 0x7FF) / 4) as f32;
    let z = (((p >> 22) & 0x3FF) / 4) as f32;

    Vector3d { x, y, z }
}

pub(crate) fn read_monster_move_spline(
    mut r: &mut impl Read,
) -> Result<Vec<Vector3d>, crate::errors::ParseError> {
    let amount_of_splines = read_u32_le(&mut r)?;
    let mut splines = Vec::with_capacity(amount_of_splines.try_into().unwrap());

    for i in 0..amount_of_splines {
        if i == 0 {
            let vec = Vector3d::read(&mut r)?;
            splines.push(vec);
        } else {
            let packed = read_u32_le(&mut r)?;
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
        first.write_into_vec(&mut v)?;
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
) -> Result<(), ExpectedOpcodeError> {
    if body_size != 0 {
        Err(ExpectedOpcodeError::Parse(
            crate::errors::ParseError::InvalidSize {
                opcode: opcode.into(),
                size: body_size,
            },
        ))
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
