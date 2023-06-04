use std::collections::BTreeMap;
use std::io::Read;

pub(crate) fn update_mask_size(dirty_mask: &[u32], header: &[u32]) -> usize {
    let amount_of_blocks = dirty_mask.len() * 4;
    let amount_of_values = dirty_mask
        .iter()
        .zip(header)
        .fold(0, |acc, x| acc + (x.0 & x.1).count_ones())
        * 4;

    1 + amount_of_blocks + amount_of_values as usize
}

pub(crate) fn array_set(array: &mut Vec<u32>, bit: u16) {
    let index = bit / 32;
    let offset = bit % 32;

    if index >= array.len() as u16 {
        let extras = index - array.len() as u16;
        for _ in 0..=extras {
            array.push(0);
        }
    }

    array[index as usize] |= 1 << offset;
}

pub(crate) fn array_reset(array: &mut [u32]) {
    for item in array {
        *item = 0;
    }
}

pub(crate) fn array_fill_ones(array: &mut [u32]) {
    for item in array {
        *item = 0xFFFFFFFF;
    }
}

pub(crate) fn header_set(
    values: &mut BTreeMap<u16, u32>,
    header: &mut Vec<u32>,
    dirty_mask: Option<&mut Vec<u32>>,
    bit: u16,
    value: u32,
) {
    values.insert(bit, value);
    array_set(header, bit);
    //Any modification to the header also means we set it dirty
    if let Some(dirty_mask) = dirty_mask {
        array_set(dirty_mask, bit);
    }
}

pub(crate) fn set_guid(
    values: &mut BTreeMap<u16, u32>,
    header: &mut Vec<u32>,
    dirty_mask: Option<&mut Vec<u32>>,
    bit: u16,
    guid: crate::Guid,
) {
    let (lower, upper) = guid.to_u32s();

    if let Some(dirty_mask) = dirty_mask {
        header_set(values, header, Some(dirty_mask), bit, lower);
        header_set(values, header, Some(dirty_mask), bit + 1, upper);
    } else {
        header_set(values, header, None, bit, lower);
        header_set(values, header, None, bit + 1, upper);
    }
}

pub(crate) fn get_guid(values: &BTreeMap<u16, u32>, bit: u16) -> Option<crate::Guid> {
    let lower = values.get(&bit);
    let upper = values.get(&(bit + 1));

    lower.map(|lower| crate::Guid::from_u32s(*lower, *upper.unwrap()))
}

pub(crate) fn set_int(
    values: &mut BTreeMap<u16, u32>,
    header: &mut Vec<u32>,
    dirty_mask: Option<&mut Vec<u32>>,
    bit: u16,
    v: i32,
) {
    header_set(
        values,
        header,
        dirty_mask,
        bit,
        u32::from_le_bytes(v.to_le_bytes()),
    );
}

pub(crate) fn set_float(
    values: &mut BTreeMap<u16, u32>,
    header: &mut Vec<u32>,
    dirty_mask: Option<&mut Vec<u32>>,
    bit: u16,
    v: f32,
) {
    header_set(
        values,
        header,
        dirty_mask,
        bit,
        u32::from_le_bytes(v.to_le_bytes()),
    );
}

pub(crate) fn set_bytes(
    values: &mut BTreeMap<u16, u32>,
    header: &mut Vec<u32>,
    dirty_mask: Option<&mut Vec<u32>>,
    bit: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
) {
    header_set(
        values,
        header,
        dirty_mask,
        bit,
        u32::from_le_bytes([a, b, c, d]),
    );
}

pub(crate) fn set_shorts(
    values: &mut BTreeMap<u16, u32>,
    header: &mut Vec<u32>,
    dirty_mask: Option<&mut Vec<u32>>,
    bit: u16,
    a: u16,
    b: u16,
) {
    header_set(
        values,
        header,
        dirty_mask,
        bit,
        crate::util::u16s_to_u32(a, b),
    );
}

pub(crate) const fn has_array_bit_set(array: &[u32], bit: u16) -> bool {
    let index = bit / 32;
    let offset = bit % 32;
    let item = array[index as usize];

    item & (1 << offset) > 0
}

pub(crate) fn has_any_bit_set(array: &[u32]) -> bool {
    array.iter().any(|h| *h != 0)
}

pub(crate) fn write_into_vec(
    mut v: impl std::io::Write,
    header: &[u32],
    dirty_mask: &[u32],
    values: &BTreeMap<u16, u32>,
) -> Result<(), std::io::Error> {
    assert_eq!(header.len(), dirty_mask.len());

    v.write_all(&[header.len() as u8])?;

    for (h, d) in header.iter().zip(dirty_mask) {
        let masked = h & d;
        v.write_all(masked.to_le_bytes().as_slice())?;
    }

    for (&index, value) in values.iter() {
        if has_array_bit_set(header, index) && has_array_bit_set(dirty_mask, index) {
            v.write_all(&value.to_le_bytes())?;
        }
    }

    Ok(())
}

pub(crate) fn read_inner(
    r: &mut impl Read,
) -> Result<(Vec<u32>, BTreeMap<u16, u32>), std::io::Error> {
    let amount_of_blocks = crate::util::read_u8_le(r)?;

    let mut header = Vec::new();
    for _ in 0..amount_of_blocks {
        header.push(crate::util::read_u32_le(r)?);
    }

    let mut values = BTreeMap::new();
    for (index, block) in header.iter().enumerate() {
        for bit in 0..32 {
            if (block & 1 << bit) != 0 {
                values.insert(index as u16 * 32 + bit, crate::util::read_u32_le(r)?);
            }
        }
    }

    Ok((header, values))
}
