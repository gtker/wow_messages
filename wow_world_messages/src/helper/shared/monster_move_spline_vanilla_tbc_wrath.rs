use crate::util::read_u32_le;
use std::io;
use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MonsterMoveSplines {
    pub splines: Vec<Vector3d>,
}

impl MonsterMoveSplines {
    pub const fn new() -> Self {
        Self { splines: vec![] }
    }

    pub(crate) fn read(mut r: &mut impl io::Read) -> Result<Self, crate::errors::ParseError> {
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

        Ok(Self { splines })
    }

    pub(crate) fn write_into_vec(&self, mut v: impl std::io::Write) -> Result<(), io::Error> {
        let amount_of_splines: u32 = self.splines.len().try_into().unwrap();

        v.write_all(&amount_of_splines.to_le_bytes())?;

        let mut splines = self.splines.iter();
        if let Some(first) = splines.next() {
            first.write_into_vec(&mut v)?;
        }

        for spline in splines {
            v.write_all(&vector3d_to_packed(spline).to_le_bytes())?;
        }

        Ok(())
    }

    pub(crate) fn size(&self) -> usize {
        let mut splines = self.splines.iter();

        let mut size = core::mem::size_of::<u32>(); // amount_of_splines: u32
        if splines.next().is_some() {
            size += 3 * core::mem::size_of::<f32>(); // First member is Vector3d
        }
        for _ in splines {
            size += core::mem::size_of::<u32>(); // All other members are packed u32
        }

        size
    }
}

fn vector3d_to_packed(v: &Vector3d) -> u32 {
    let mut packed = 0;

    packed |= (v.x / 0.25) as u32 & 0x7FF;
    packed |= ((v.y / 0.25) as u32 & 0x7FF) << 11;
    packed |= ((v.z / 0.25) as u32 & 0x3FF) << 22;

    packed
}

const fn packed_to_vector3d(p: u32) -> Vector3d {
    let x = ((p & 0x7FF) / 4) as f32;
    let y = (((p >> 11) & 0x7FF) / 4) as f32;
    let z = (((p >> 22) & 0x3FF) / 4) as f32;

    Vector3d { x, y, z }
}
