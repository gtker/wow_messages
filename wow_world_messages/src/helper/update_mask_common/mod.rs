pub(crate) mod inners;

pub const OBJECT: u32 = 0x0001;
pub const ITEM: u32 = 0x0002;
pub const CONTAINER: u32 = 0x0004;
pub const UNIT: u32 = 0x0008;
pub const PLAYER: u32 = 0x0010;
pub const GAMEOBJECT: u32 = 0x0020;
pub const DYNAMICOBJECT: u32 = 0x0040;
pub const CORPSE: u32 = 0x0080;

macro_rules! update_item {
    ($name:ident, $builder_name:ident, $type_value:expr) => {
        #[derive(Debug, Hash, Clone, Default, PartialEq, Eq, PartialOrd)]
        pub struct $builder_name {
            header: Vec<u32>,
            values: std::collections::BTreeMap<u16, u32>,
        }

        impl $builder_name {
            pub fn finalize(self) -> $name {
                $name::from_inners(self.header, self.values)
            }

            #[allow(unused)]
            pub(crate) fn header_set(&mut self, bit: u16, value: u32) {
                $crate::helper::update_mask_common::inners::header_set(
                    &mut self.values,
                    &mut self.header,
                    None,
                    bit,
                    value,
                );
            }

            pub(crate) fn set_guid(&mut self, bit: u16, guid: $crate::Guid) {
                $crate::helper::update_mask_common::inners::set_guid(
                    &mut self.values,
                    &mut self.header,
                    None,
                    bit,
                    guid,
                );
            }

            #[allow(unused)]
            pub(crate) fn set_int(&mut self, bit: u16, v: i32) {
                $crate::helper::update_mask_common::inners::set_int(
                    &mut self.values,
                    &mut self.header,
                    None,
                    bit,
                    v,
                );
            }

            #[allow(unused)]
            pub(crate) fn set_float(&mut self, bit: u16, v: f32) {
                $crate::helper::update_mask_common::inners::set_float(
                    &mut self.values,
                    &mut self.header,
                    None,
                    bit,
                    v,
                );
            }

            #[allow(unused)]
            pub(crate) fn set_bytes(&mut self, bit: u16, a: u8, b: u8, c: u8, d: u8) {
                $crate::helper::update_mask_common::inners::set_bytes(
                    &mut self.values,
                    &mut self.header,
                    None,
                    bit,
                    a,
                    b,
                    c,
                    d,
                );
            }

            #[allow(unused)]
            pub(crate) fn set_shorts(&mut self, bit: u16, a: u16, b: u16) {
                $crate::helper::update_mask_common::inners::set_shorts(
                    &mut self.values,
                    &mut self.header,
                    None,
                    bit,
                    a,
                    b,
                );
            }

            pub fn new() -> Self {
                const OBJECT_FIELD_TYPE: u16 = 2;

                let mut header = vec![];
                let mut values = std::collections::BTreeMap::new();

                $crate::helper::update_mask_common::inners::array_set(
                    &mut header,
                    OBJECT_FIELD_TYPE,
                );
                values.insert(
                    OBJECT_FIELD_TYPE,
                    $crate::helper::update_mask_common::OBJECT | $type_value,
                );

                Self { header, values }
            }
        }

        impl From<$builder_name> for $name {
            fn from(e: $builder_name) -> $name {
                e.finalize()
            }
        }

        #[derive(Debug, Hash, Clone, Default, PartialEq, Eq, PartialOrd)]
        pub struct $name {
            header: Vec<u32>,
            dirty_mask: Vec<u32>,
            values: std::collections::BTreeMap<u16, u32>,
        }

        impl $name {
            pub fn new() -> Self {
                $builder_name::new().finalize()
            }

            pub fn builder() -> $builder_name {
                $builder_name::new()
            }

            fn from_inners(header: Vec<u32>, values: std::collections::BTreeMap<u16, u32>) -> Self {
                Self {
                    header: header.clone(),
                    dirty_mask: header,
                    values,
                }
            }

            pub(crate) fn set_guid(&mut self, bit: u16, guid: $crate::Guid) {
                $crate::helper::update_mask_common::inners::set_guid(
                    &mut self.values,
                    &mut self.header,
                    Some(&mut self.dirty_mask),
                    bit,
                    guid,
                );
            }

            pub(crate) fn get_guid(&self, bit: u16) -> Option<$crate::Guid> {
                $crate::helper::update_mask_common::inners::get_guid(&self.values, bit)
            }

            #[allow(unused)]
            pub(crate) fn set_int(&mut self, bit: u16, v: i32) {
                $crate::helper::update_mask_common::inners::set_int(
                    &mut self.values,
                    &mut self.header,
                    Some(&mut self.dirty_mask),
                    bit,
                    v,
                );
            }

            #[allow(unused)]
            pub(crate) fn get_int(&self, bit: u16) -> Option<i32> {
                $crate::helper::update_mask_common::inners::get_int(&self.values, bit)
            }

            #[allow(unused)]
            pub(crate) fn set_float(&mut self, bit: u16, v: f32) {
                $crate::helper::update_mask_common::inners::set_float(
                    &mut self.values,
                    &mut self.header,
                    Some(&mut self.dirty_mask),
                    bit,
                    v,
                );
            }

            #[allow(unused)]
            pub(crate) fn get_float(&self, bit: u16) -> Option<f32> {
                $crate::helper::update_mask_common::inners::get_float(&self.values, bit)
            }

            #[allow(unused)]
            pub(crate) fn set_bytes(&mut self, bit: u16, a: u8, b: u8, c: u8, d: u8) {
                $crate::helper::update_mask_common::inners::set_bytes(
                    &mut self.values,
                    &mut self.header,
                    Some(&mut self.dirty_mask),
                    bit,
                    a,
                    b,
                    c,
                    d,
                );
            }

            #[allow(unused)]
            pub(crate) fn get_bytes(&self, bit: u16) -> Option<(u8, u8, u8, u8)> {
                $crate::helper::update_mask_common::inners::get_bytes(&self.values, bit)
            }

            #[allow(unused)]
            pub(crate) fn set_shorts(&mut self, bit: u16, a: u16, b: u16) {
                $crate::helper::update_mask_common::inners::set_shorts(
                    &mut self.values,
                    &mut self.header,
                    Some(&mut self.dirty_mask),
                    bit,
                    a,
                    b,
                );
            }

            #[allow(unused)]
            pub(crate) fn get_shorts(&self, bit: u16) -> Option<(u16, u16)> {
                $crate::helper::update_mask_common::inners::get_shorts(&self.values, bit)
            }

            #[allow(unused)]
            pub(crate) fn header_set(&mut self, bit: u16, value: u32) {
                $crate::helper::update_mask_common::inners::header_set(
                    &mut self.values,
                    &mut self.header,
                    Some(&mut self.dirty_mask),
                    bit,
                    value,
                );
            }

            pub fn dirty_reset(&mut self) {
                $crate::helper::update_mask_common::inners::array_reset(&mut self.dirty_mask);
            }

            pub fn mark_fully_dirty(&mut self) {
                $crate::helper::update_mask_common::inners::array_fill_ones(&mut self.dirty_mask);
            }

            pub fn has_any_dirty_fields(&self) -> bool {
                $crate::helper::update_mask_common::inners::has_any_bit_set(&self.dirty_mask)
            }

            pub fn is_bit_dirty(&self, bit: u16) -> bool {
                $crate::helper::update_mask_common::inners::has_array_bit_set(&self.dirty_mask, bit)
            }

            pub(crate) fn write_into_vec(
                &self,
                v: impl std::io::Write,
            ) -> Result<(), std::io::Error> {
                $crate::helper::update_mask_common::inners::write_into_vec(
                    v,
                    &self.header,
                    &self.dirty_mask,
                    &self.values,
                )
            }
        }
    };
}
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) use update_item;

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
macro_rules! update_mask {
    () => {
        #[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd)]
        pub enum UpdateMask {
            Item(UpdateItem),
            Container(UpdateContainer),
            Unit(UpdateUnit),
            Player(UpdatePlayer),
            GameObject(UpdateGameObject),
            DynamicObject(UpdateDynamicObject),
            Corpse(UpdateCorpse),
        }

        impl UpdateMask {
            pub(crate) fn read(r: &mut impl std::io::Read) -> Result<Self, std::io::Error> {
                let (header, values) = $crate::helper::update_mask_common::inners::read_inner(r)?;

                let ty = match values.get(&2) {
                    None => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Missing object TYPE",
                        ))
                    }
                    Some(ty) => *ty,
                };

                Ok(
                    if (ty & $crate::helper::update_mask_common::CONTAINER) != 0 {
                        Self::Container(UpdateContainer::from_inners(header, values))
                    } else if (ty & $crate::helper::update_mask_common::ITEM) != 0 {
                        Self::Item(UpdateItem::from_inners(header, values))
                    } else if (ty & $crate::helper::update_mask_common::PLAYER) != 0 {
                        Self::Player(UpdatePlayer::from_inners(header, values))
                    } else if (ty & $crate::helper::update_mask_common::UNIT) != 0 {
                        Self::Unit(UpdateUnit::from_inners(header, values))
                    } else if (ty & $crate::helper::update_mask_common::GAMEOBJECT) != 0 {
                        Self::GameObject(UpdateGameObject::from_inners(header, values))
                    } else if (ty & $crate::helper::update_mask_common::DYNAMICOBJECT) != 0 {
                        Self::DynamicObject(UpdateDynamicObject::from_inners(header, values))
                    } else if (ty & $crate::helper::update_mask_common::CORPSE) != 0 {
                        Self::Corpse(UpdateCorpse::from_inners(header, values))
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Object type not valid",
                        ));
                    },
                )
            }

            pub(crate) fn write_into_vec(
                &self,
                v: impl std::io::Write,
            ) -> Result<(), std::io::Error> {
                match self {
                    UpdateMask::Item(i) => i.write_into_vec(v),
                    UpdateMask::Container(i) => i.write_into_vec(v),
                    UpdateMask::Unit(i) => i.write_into_vec(v),
                    UpdateMask::Player(i) => i.write_into_vec(v),
                    UpdateMask::GameObject(i) => i.write_into_vec(v),
                    UpdateMask::DynamicObject(i) => i.write_into_vec(v),
                    UpdateMask::Corpse(i) => i.write_into_vec(v),
                }
            }

            pub(crate) fn size(&self) -> usize {
                match self {
                    UpdateMask::Item(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::Container(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::Unit(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::Player(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::GameObject(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::DynamicObject(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                    UpdateMask::Corpse(i) => {
                        $crate::helper::update_mask_common::inners::update_mask_size(
                            &i.dirty_mask,
                            &i.header,
                        )
                    }
                }
            }
        }

        impl From<UpdateItem> for UpdateMask {
            fn from(e: UpdateItem) -> UpdateMask {
                Self::Item(e)
            }
        }

        impl From<UpdateContainer> for UpdateMask {
            fn from(e: UpdateContainer) -> UpdateMask {
                Self::Container(e)
            }
        }

        impl From<UpdateUnit> for UpdateMask {
            fn from(e: UpdateUnit) -> UpdateMask {
                Self::Unit(e)
            }
        }

        impl From<UpdatePlayer> for UpdateMask {
            fn from(e: UpdatePlayer) -> UpdateMask {
                Self::Player(e)
            }
        }

        impl From<UpdateGameObject> for UpdateMask {
            fn from(e: UpdateGameObject) -> UpdateMask {
                Self::GameObject(e)
            }
        }

        impl From<UpdateDynamicObject> for UpdateMask {
            fn from(e: UpdateDynamicObject) -> UpdateMask {
                Self::DynamicObject(e)
            }
        }

        impl From<UpdateCorpse> for UpdateMask {
            fn from(e: UpdateCorpse) -> UpdateMask {
                Self::Corpse(e)
            }
        }

        impl Default for UpdateMask {
            fn default() -> Self {
                Self::Item(Default::default())
            }
        }
    };
}
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) use update_mask;

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
macro_rules! skill_info {
    ($skill:ty, $index:ty, $skill_info:ty) => {
        impl $skill_info {
            pub(crate) const fn mask_values(&self, index: $index) -> [(u16, u32); 3] {
                let offset = index.offset();

                [
                    (
                        offset,
                        $crate::util::u16s_to_u32(self.skill_step, self.skill.as_int()),
                    ),
                    (
                        offset + 1,
                        $crate::util::u16s_to_u32(self.maximum, self.minimum),
                    ),
                    (
                        offset + 2,
                        $crate::util::u16s_to_u32(self.permanent_bonus, self.temporary_bonus),
                    ),
                ]
            }

            pub(crate) fn from_range<'a>(
                mut range: impl Iterator<Item = (&'a u16, &'a u32)>,
            ) -> Option<Self> {
                let (_, first) = range.next()?;
                let (_, second) = range.next()?;
                let (_, third) = range.next()?;

                let (skill_step, skill) = $crate::util::u32_to_u16s(*first);
                let (maximum, minimum) = $crate::util::u32_to_u16s(*second);
                let (permanent_bonus, temporary_bonus) = $crate::util::u32_to_u16s(*third);
                let skill = match <$skill>::try_from(skill) {
                    Ok(v) => v,
                    Err(_) => return None,
                };

                Some(Self {
                    skill,
                    skill_step,
                    minimum,
                    maximum,
                    permanent_bonus,
                    temporary_bonus,
                })
            }
        }
    };
}
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) use skill_info;
