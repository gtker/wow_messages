use crate::Guid;
use std::convert::TryInto;
use crate::vanilla::{Race};
use crate::vanilla::{Class};
use crate::vanilla::{Gender};
use crate::vanilla::{Power};
use crate::vanilla::{UpdateContainer, UpdateCorpse, UpdateDynamicObject, UpdateGameObject, UpdateItem, UpdatePlayer, UpdateUnit};

impl UpdateItem {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&3) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&4) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_item_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn item_OWNER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_item_CONTAINED(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn item_CONTAINED(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_item_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(10);
        self.header_set(11);
        self.values.insert(10, v.guid() as u32);
        self.values.insert(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn item_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_item_GIFTCREATOR(mut self, v: Guid) -> Self {
        self.header_set(12);
        self.header_set(13);
        self.values.insert(12, v.guid() as u32);
        self.values.insert(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn item_GIFTCREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_item_STACK_COUNT(mut self, v: i32) -> Self {
        self.header_set(14);
        self.values.insert(14, v as u32);
        self
    }

    pub fn item_STACK_COUNT(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&14) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_DURATION(mut self, v: i32) -> Self {
        self.header_set(15);
        self.values.insert(15, v as u32);
        self
    }

    pub fn item_DURATION(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&15) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_SPELL_CHARGES(mut self, v: i32) -> Self {
        self.header_set(16);
        self.values.insert(16, v as u32);
        self
    }

    pub fn item_SPELL_CHARGES(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&16) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_FLAGS(mut self, v: i32) -> Self {
        self.header_set(21);
        self.values.insert(21, v as u32);
        self
    }

    pub fn item_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&21) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn item_ENCHANTMENT(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&22) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(43);
        self.values.insert(43, v as u32);
        self
    }

    pub fn item_PROPERTY_SEED(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&43) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(44);
        self.values.insert(44, v as u32);
        self
    }

    pub fn item_RANDOM_PROPERTIES_ID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&44) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_ITEM_TEXT_ID(mut self, v: i32) -> Self {
        self.header_set(45);
        self.values.insert(45, v as u32);
        self
    }

    pub fn item_ITEM_TEXT_ID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&45) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(46);
        self.values.insert(46, v as u32);
        self
    }

    pub fn item_DURABILITY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&46) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(47);
        self.values.insert(47, v as u32);
        self
    }

    pub fn item_MAXDURABILITY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&47) {
            Some(*v as i32)
        } else {
            None
        }
    }

}

impl UpdateContainer {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&3) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&4) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_item_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn item_OWNER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_item_CONTAINED(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn item_CONTAINED(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_item_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(10);
        self.header_set(11);
        self.values.insert(10, v.guid() as u32);
        self.values.insert(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn item_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_item_GIFTCREATOR(mut self, v: Guid) -> Self {
        self.header_set(12);
        self.header_set(13);
        self.values.insert(12, v.guid() as u32);
        self.values.insert(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn item_GIFTCREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_item_STACK_COUNT(mut self, v: i32) -> Self {
        self.header_set(14);
        self.values.insert(14, v as u32);
        self
    }

    pub fn item_STACK_COUNT(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&14) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_DURATION(mut self, v: i32) -> Self {
        self.header_set(15);
        self.values.insert(15, v as u32);
        self
    }

    pub fn item_DURATION(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&15) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_SPELL_CHARGES(mut self, v: i32) -> Self {
        self.header_set(16);
        self.values.insert(16, v as u32);
        self
    }

    pub fn item_SPELL_CHARGES(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&16) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_FLAGS(mut self, v: i32) -> Self {
        self.header_set(21);
        self.values.insert(21, v as u32);
        self
    }

    pub fn item_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&21) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn item_ENCHANTMENT(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&22) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(43);
        self.values.insert(43, v as u32);
        self
    }

    pub fn item_PROPERTY_SEED(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&43) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(44);
        self.values.insert(44, v as u32);
        self
    }

    pub fn item_RANDOM_PROPERTIES_ID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&44) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_ITEM_TEXT_ID(mut self, v: i32) -> Self {
        self.header_set(45);
        self.values.insert(45, v as u32);
        self
    }

    pub fn item_ITEM_TEXT_ID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&45) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(46);
        self.values.insert(46, v as u32);
        self
    }

    pub fn item_DURABILITY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&46) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_item_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(47);
        self.values.insert(47, v as u32);
        self
    }

    pub fn item_MAXDURABILITY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&47) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_container_NUM_SLOTS(mut self, v: i32) -> Self {
        self.header_set(48);
        self.values.insert(48, v as u32);
        self
    }

    pub fn container_NUM_SLOTS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&48) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_container_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(50);
        self.header_set(51);
        self.values.insert(50, v.guid() as u32);
        self.values.insert(51, (v.guid() >> 32) as u32);
        self
    }

    pub fn container_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&50);
        let upper = self.values.get(&51);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

}

impl UpdateUnit {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&3) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&4) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_CHARM(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_CHARM(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_SUMMON(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_SUMMON(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_CHARMEDBY(mut self, v: Guid) -> Self {
        self.header_set(10);
        self.header_set(11);
        self.values.insert(10, v.guid() as u32);
        self.values.insert(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_CHARMEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_SUMMONEDBY(mut self, v: Guid) -> Self {
        self.header_set(12);
        self.header_set(13);
        self.values.insert(12, v.guid() as u32);
        self.values.insert(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_SUMMONEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_CREATEDBY(mut self, v: Guid) -> Self {
        self.header_set(14);
        self.header_set(15);
        self.values.insert(14, v.guid() as u32);
        self.values.insert(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_CREATEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&14);
        let upper = self.values.get(&15);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_TARGET(mut self, v: Guid) -> Self {
        self.header_set(16);
        self.header_set(17);
        self.values.insert(16, v.guid() as u32);
        self.values.insert(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_TARGET(&self) -> Option<Guid> {
        let lower = self.values.get(&16);
        let upper = self.values.get(&17);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_PERSUADED(mut self, v: Guid) -> Self {
        self.header_set(18);
        self.header_set(19);
        self.values.insert(18, v.guid() as u32);
        self.values.insert(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_PERSUADED(&self) -> Option<Guid> {
        let lower = self.values.get(&18);
        let upper = self.values.get(&19);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_CHANNEL_OBJECT(mut self, v: Guid) -> Self {
        self.header_set(20);
        self.header_set(21);
        self.values.insert(20, v.guid() as u32);
        self.values.insert(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_CHANNEL_OBJECT(&self) -> Option<Guid> {
        let lower = self.values.get(&20);
        let upper = self.values.get(&21);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_HEALTH(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn unit_HEALTH(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&22) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_POWER1(mut self, v: i32) -> Self {
        self.header_set(23);
        self.values.insert(23, v as u32);
        self
    }

    pub fn unit_POWER1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&23) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_POWER2(mut self, v: i32) -> Self {
        self.header_set(24);
        self.values.insert(24, v as u32);
        self
    }

    pub fn unit_POWER2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&24) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_POWER3(mut self, v: i32) -> Self {
        self.header_set(25);
        self.values.insert(25, v as u32);
        self
    }

    pub fn unit_POWER3(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&25) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_POWER4(mut self, v: i32) -> Self {
        self.header_set(26);
        self.values.insert(26, v as u32);
        self
    }

    pub fn unit_POWER4(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&26) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_POWER5(mut self, v: i32) -> Self {
        self.header_set(27);
        self.values.insert(27, v as u32);
        self
    }

    pub fn unit_POWER5(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&27) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(28);
        self.values.insert(28, v as u32);
        self
    }

    pub fn unit_MAXHEALTH(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&28) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(29);
        self.values.insert(29, v as u32);
        self
    }

    pub fn unit_MAXPOWER1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&29) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(30);
        self.values.insert(30, v as u32);
        self
    }

    pub fn unit_MAXPOWER2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&30) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(31);
        self.values.insert(31, v as u32);
        self
    }

    pub fn unit_MAXPOWER3(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&31) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(32);
        self.values.insert(32, v as u32);
        self
    }

    pub fn unit_MAXPOWER4(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&32) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(33);
        self.values.insert(33, v as u32);
        self
    }

    pub fn unit_MAXPOWER5(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&33) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_LEVEL(mut self, v: i32) -> Self {
        self.header_set(34);
        self.values.insert(34, v as u32);
        self
    }

    pub fn unit_LEVEL(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&34) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(35);
        self.values.insert(35, v as u32);
        self
    }

    pub fn unit_FACTIONTEMPLATE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&35) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(36);
        self.values.insert(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn unit_BYTES_0(&self) -> Option<(Race, Class, Gender, Power)> {
        if let Some(v) = self.values.get(&36) {
            let v = v.to_le_bytes();
            let (race, class, gender, power) = (v[0], v[1], v[2], v[3]);
            Some((race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(mut self, v: i32) -> Self {
        self.header_set(37);
        self.values.insert(37, v as u32);
        self
    }

    pub fn unit_VIRTUAL_ITEM_SLOT_DISPLAY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&37) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(40);
        self.values.insert(40, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn unit_VIRTUAL_ITEM_INFO(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&40) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_FLAGS(mut self, v: i32) -> Self {
        self.header_set(46);
        self.values.insert(46, v as u32);
        self
    }

    pub fn unit_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&46) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_AURA(mut self, v: i32) -> Self {
        self.header_set(47);
        self.values.insert(47, v as u32);
        self
    }

    pub fn unit_AURA(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&47) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_AURAFLAGS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(95);
        self.values.insert(95, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn unit_AURAFLAGS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&95) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURALEVELS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(101);
        self.values.insert(101, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn unit_AURALEVELS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&101) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURAAPPLICATIONS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(113);
        self.values.insert(113, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn unit_AURAAPPLICATIONS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&113) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURASTATE(mut self, v: i32) -> Self {
        self.header_set(125);
        self.values.insert(125, v as u32);
        self
    }

    pub fn unit_AURASTATE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&125) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(126);
        self.values.insert(126, v as u32);
        self
    }

    pub fn unit_BASEATTACKTIME(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&126) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(128);
        self.values.insert(128, v as u32);
        self
    }

    pub fn unit_RANGEDATTACKTIME(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&128) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_BOUNDINGRADIUS(mut self, v: f32) -> Self {
        self.header_set(129);
        self.values.insert(129, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_BOUNDINGRADIUS(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&129) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_COMBATREACH(mut self, v: f32) -> Self {
        self.header_set(130);
        self.values.insert(130, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_COMBATREACH(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&130) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(131);
        self.values.insert(131, v as u32);
        self
    }

    pub fn unit_DISPLAYID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&131) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(132);
        self.values.insert(132, v as u32);
        self
    }

    pub fn unit_NATIVEDISPLAYID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&132) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(133);
        self.values.insert(133, v as u32);
        self
    }

    pub fn unit_MOUNTDISPLAYID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&133) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MINDAMAGE(mut self, v: f32) -> Self {
        self.header_set(134);
        self.values.insert(134, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MINDAMAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&134) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_MAXDAMAGE(mut self, v: f32) -> Self {
        self.header_set(135);
        self.values.insert(135, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MAXDAMAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&135) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_MINOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(136);
        self.values.insert(136, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MINOFFHANDDAMAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&136) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(137);
        self.values.insert(137, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MAXOFFHANDDAMAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&137) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(138);
        self.values.insert(138, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn unit_BYTES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&138) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_PETNUMBER(mut self, v: i32) -> Self {
        self.header_set(139);
        self.values.insert(139, v as u32);
        self
    }

    pub fn unit_PETNUMBER(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&139) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(140);
        self.values.insert(140, v as u32);
        self
    }

    pub fn unit_PET_NAME_TIMESTAMP(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&140) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(141);
        self.values.insert(141, v as u32);
        self
    }

    pub fn unit_PETEXPERIENCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&141) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(142);
        self.values.insert(142, v as u32);
        self
    }

    pub fn unit_PETNEXTLEVELEXP(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&142) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(143);
        self.values.insert(143, v as u32);
        self
    }

    pub fn unit_DYNAMIC_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&143) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(144);
        self.values.insert(144, v as u32);
        self
    }

    pub fn unit_CHANNEL_SPELL(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&144) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(145);
        self.values.insert(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MOD_CAST_SPEED(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&145) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(146);
        self.values.insert(146, v as u32);
        self
    }

    pub fn unit_CREATED_BY_SPELL(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&146) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(147);
        self.values.insert(147, v as u32);
        self
    }

    pub fn unit_NPC_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&147) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(148);
        self.values.insert(148, v as u32);
        self
    }

    pub fn unit_NPC_EMOTESTATE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&148) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_TRAINING_POINTS(mut self, a: u16, b: u16) -> Self {
        self.header_set(149);
        self.values.insert(149, (a as u32) << 16 | b as u32);
        self
    }

    pub fn unit_TRAINING_POINTS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&149) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(150);
        self.values.insert(150, v as u32);
        self
    }

    pub fn unit_STRENGTH(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&150) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_AGILITY(mut self, v: i32) -> Self {
        self.header_set(151);
        self.values.insert(151, v as u32);
        self
    }

    pub fn unit_AGILITY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&151) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_STAMINA(mut self, v: i32) -> Self {
        self.header_set(152);
        self.values.insert(152, v as u32);
        self
    }

    pub fn unit_STAMINA(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&152) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(153);
        self.values.insert(153, v as u32);
        self
    }

    pub fn unit_INTELLECT(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&153) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(154);
        self.values.insert(154, v as u32);
        self
    }

    pub fn unit_SPIRIT(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&154) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_NORMAL_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(155);
        self.values.insert(155, v as u32);
        self
    }

    pub fn unit_NORMAL_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&155) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_HOLY_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(156);
        self.values.insert(156, v as u32);
        self
    }

    pub fn unit_HOLY_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&156) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_FIRE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(157);
        self.values.insert(157, v as u32);
        self
    }

    pub fn unit_FIRE_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&157) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_NATURE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(158);
        self.values.insert(158, v as u32);
        self
    }

    pub fn unit_NATURE_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&158) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_FROST_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(159);
        self.values.insert(159, v as u32);
        self
    }

    pub fn unit_FROST_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&159) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_SHADOW_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(160);
        self.values.insert(160, v as u32);
        self
    }

    pub fn unit_SHADOW_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&160) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_ARCANE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(161);
        self.values.insert(161, v as u32);
        self
    }

    pub fn unit_ARCANE_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&161) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(162);
        self.values.insert(162, v as u32);
        self
    }

    pub fn unit_BASE_MANA(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&162) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(163);
        self.values.insert(163, v as u32);
        self
    }

    pub fn unit_BASE_HEALTH(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&163) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_BYTES_2(mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) -> Self {
        self.header_set(164);
        self.values.insert(164, u32::from_le_bytes([facial_hair, unknown, bank_bag_slots, rested_state]));
        self
    }

    pub fn unit_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&164) {
            let v = v.to_le_bytes();
            let (facial_hair, unknown, bank_bag_slots, rested_state) = (v[0], v[1], v[2], v[3]);
            Some((facial_hair.try_into().unwrap(), unknown.try_into().unwrap(), bank_bag_slots.try_into().unwrap(), rested_state.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(165);
        self.values.insert(165, v as u32);
        self
    }

    pub fn unit_ATTACK_POWER(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&165) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(166);
        self.values.insert(166, (a as u32) << 16 | b as u32);
        self
    }

    pub fn unit_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&166) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(167);
        self.values.insert(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&167) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(168);
        self.values.insert(168, v as u32);
        self
    }

    pub fn unit_RANGED_ATTACK_POWER(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&168) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(169);
        self.values.insert(169, (a as u32) << 16 | b as u32);
        self
    }

    pub fn unit_RANGED_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&169) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(170);
        self.values.insert(170, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_RANGED_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&170) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_MINRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(171);
        self.values.insert(171, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MINRANGEDDAMAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&171) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_MAXRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(172);
        self.values.insert(172, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MAXRANGEDDAMAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&172) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_POWER_COST_MODIFIER(mut self, v: i32) -> Self {
        self.header_set(173);
        self.values.insert(173, v as u32);
        self
    }

    pub fn unit_POWER_COST_MODIFIER(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&173) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(180);
        self.values.insert(180, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_POWER_COST_MULTIPLIER(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&180) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

}

impl UpdatePlayer {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&3) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&4) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_CHARM(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_CHARM(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_SUMMON(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_SUMMON(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_CHARMEDBY(mut self, v: Guid) -> Self {
        self.header_set(10);
        self.header_set(11);
        self.values.insert(10, v.guid() as u32);
        self.values.insert(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_CHARMEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_SUMMONEDBY(mut self, v: Guid) -> Self {
        self.header_set(12);
        self.header_set(13);
        self.values.insert(12, v.guid() as u32);
        self.values.insert(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_SUMMONEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_CREATEDBY(mut self, v: Guid) -> Self {
        self.header_set(14);
        self.header_set(15);
        self.values.insert(14, v.guid() as u32);
        self.values.insert(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_CREATEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&14);
        let upper = self.values.get(&15);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_TARGET(mut self, v: Guid) -> Self {
        self.header_set(16);
        self.header_set(17);
        self.values.insert(16, v.guid() as u32);
        self.values.insert(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_TARGET(&self) -> Option<Guid> {
        let lower = self.values.get(&16);
        let upper = self.values.get(&17);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_PERSUADED(mut self, v: Guid) -> Self {
        self.header_set(18);
        self.header_set(19);
        self.values.insert(18, v.guid() as u32);
        self.values.insert(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_PERSUADED(&self) -> Option<Guid> {
        let lower = self.values.get(&18);
        let upper = self.values.get(&19);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_CHANNEL_OBJECT(mut self, v: Guid) -> Self {
        self.header_set(20);
        self.header_set(21);
        self.values.insert(20, v.guid() as u32);
        self.values.insert(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn unit_CHANNEL_OBJECT(&self) -> Option<Guid> {
        let lower = self.values.get(&20);
        let upper = self.values.get(&21);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_unit_HEALTH(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn unit_HEALTH(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&22) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_POWER1(mut self, v: i32) -> Self {
        self.header_set(23);
        self.values.insert(23, v as u32);
        self
    }

    pub fn unit_POWER1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&23) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_POWER2(mut self, v: i32) -> Self {
        self.header_set(24);
        self.values.insert(24, v as u32);
        self
    }

    pub fn unit_POWER2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&24) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_POWER3(mut self, v: i32) -> Self {
        self.header_set(25);
        self.values.insert(25, v as u32);
        self
    }

    pub fn unit_POWER3(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&25) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_POWER4(mut self, v: i32) -> Self {
        self.header_set(26);
        self.values.insert(26, v as u32);
        self
    }

    pub fn unit_POWER4(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&26) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_POWER5(mut self, v: i32) -> Self {
        self.header_set(27);
        self.values.insert(27, v as u32);
        self
    }

    pub fn unit_POWER5(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&27) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(28);
        self.values.insert(28, v as u32);
        self
    }

    pub fn unit_MAXHEALTH(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&28) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(29);
        self.values.insert(29, v as u32);
        self
    }

    pub fn unit_MAXPOWER1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&29) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(30);
        self.values.insert(30, v as u32);
        self
    }

    pub fn unit_MAXPOWER2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&30) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(31);
        self.values.insert(31, v as u32);
        self
    }

    pub fn unit_MAXPOWER3(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&31) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(32);
        self.values.insert(32, v as u32);
        self
    }

    pub fn unit_MAXPOWER4(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&32) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(33);
        self.values.insert(33, v as u32);
        self
    }

    pub fn unit_MAXPOWER5(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&33) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_LEVEL(mut self, v: i32) -> Self {
        self.header_set(34);
        self.values.insert(34, v as u32);
        self
    }

    pub fn unit_LEVEL(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&34) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(35);
        self.values.insert(35, v as u32);
        self
    }

    pub fn unit_FACTIONTEMPLATE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&35) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(36);
        self.values.insert(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn unit_BYTES_0(&self) -> Option<(Race, Class, Gender, Power)> {
        if let Some(v) = self.values.get(&36) {
            let v = v.to_le_bytes();
            let (race, class, gender, power) = (v[0], v[1], v[2], v[3]);
            Some((race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(mut self, v: i32) -> Self {
        self.header_set(37);
        self.values.insert(37, v as u32);
        self
    }

    pub fn unit_VIRTUAL_ITEM_SLOT_DISPLAY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&37) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(40);
        self.values.insert(40, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn unit_VIRTUAL_ITEM_INFO(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&40) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_FLAGS(mut self, v: i32) -> Self {
        self.header_set(46);
        self.values.insert(46, v as u32);
        self
    }

    pub fn unit_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&46) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_AURA(mut self, v: i32) -> Self {
        self.header_set(47);
        self.values.insert(47, v as u32);
        self
    }

    pub fn unit_AURA(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&47) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_AURAFLAGS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(95);
        self.values.insert(95, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn unit_AURAFLAGS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&95) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURALEVELS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(101);
        self.values.insert(101, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn unit_AURALEVELS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&101) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURAAPPLICATIONS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(113);
        self.values.insert(113, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn unit_AURAAPPLICATIONS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&113) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURASTATE(mut self, v: i32) -> Self {
        self.header_set(125);
        self.values.insert(125, v as u32);
        self
    }

    pub fn unit_AURASTATE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&125) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(126);
        self.values.insert(126, v as u32);
        self
    }

    pub fn unit_BASEATTACKTIME(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&126) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(128);
        self.values.insert(128, v as u32);
        self
    }

    pub fn unit_RANGEDATTACKTIME(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&128) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_BOUNDINGRADIUS(mut self, v: f32) -> Self {
        self.header_set(129);
        self.values.insert(129, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_BOUNDINGRADIUS(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&129) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_COMBATREACH(mut self, v: f32) -> Self {
        self.header_set(130);
        self.values.insert(130, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_COMBATREACH(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&130) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(131);
        self.values.insert(131, v as u32);
        self
    }

    pub fn unit_DISPLAYID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&131) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(132);
        self.values.insert(132, v as u32);
        self
    }

    pub fn unit_NATIVEDISPLAYID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&132) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(133);
        self.values.insert(133, v as u32);
        self
    }

    pub fn unit_MOUNTDISPLAYID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&133) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MINDAMAGE(mut self, v: f32) -> Self {
        self.header_set(134);
        self.values.insert(134, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MINDAMAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&134) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_MAXDAMAGE(mut self, v: f32) -> Self {
        self.header_set(135);
        self.values.insert(135, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MAXDAMAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&135) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_MINOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(136);
        self.values.insert(136, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MINOFFHANDDAMAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&136) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(137);
        self.values.insert(137, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MAXOFFHANDDAMAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&137) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(138);
        self.values.insert(138, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn unit_BYTES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&138) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_PETNUMBER(mut self, v: i32) -> Self {
        self.header_set(139);
        self.values.insert(139, v as u32);
        self
    }

    pub fn unit_PETNUMBER(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&139) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(140);
        self.values.insert(140, v as u32);
        self
    }

    pub fn unit_PET_NAME_TIMESTAMP(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&140) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(141);
        self.values.insert(141, v as u32);
        self
    }

    pub fn unit_PETEXPERIENCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&141) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(142);
        self.values.insert(142, v as u32);
        self
    }

    pub fn unit_PETNEXTLEVELEXP(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&142) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(143);
        self.values.insert(143, v as u32);
        self
    }

    pub fn unit_DYNAMIC_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&143) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(144);
        self.values.insert(144, v as u32);
        self
    }

    pub fn unit_CHANNEL_SPELL(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&144) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(145);
        self.values.insert(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MOD_CAST_SPEED(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&145) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(146);
        self.values.insert(146, v as u32);
        self
    }

    pub fn unit_CREATED_BY_SPELL(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&146) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(147);
        self.values.insert(147, v as u32);
        self
    }

    pub fn unit_NPC_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&147) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(148);
        self.values.insert(148, v as u32);
        self
    }

    pub fn unit_NPC_EMOTESTATE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&148) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_TRAINING_POINTS(mut self, a: u16, b: u16) -> Self {
        self.header_set(149);
        self.values.insert(149, (a as u32) << 16 | b as u32);
        self
    }

    pub fn unit_TRAINING_POINTS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&149) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(150);
        self.values.insert(150, v as u32);
        self
    }

    pub fn unit_STRENGTH(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&150) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_AGILITY(mut self, v: i32) -> Self {
        self.header_set(151);
        self.values.insert(151, v as u32);
        self
    }

    pub fn unit_AGILITY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&151) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_STAMINA(mut self, v: i32) -> Self {
        self.header_set(152);
        self.values.insert(152, v as u32);
        self
    }

    pub fn unit_STAMINA(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&152) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(153);
        self.values.insert(153, v as u32);
        self
    }

    pub fn unit_INTELLECT(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&153) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(154);
        self.values.insert(154, v as u32);
        self
    }

    pub fn unit_SPIRIT(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&154) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_NORMAL_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(155);
        self.values.insert(155, v as u32);
        self
    }

    pub fn unit_NORMAL_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&155) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_HOLY_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(156);
        self.values.insert(156, v as u32);
        self
    }

    pub fn unit_HOLY_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&156) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_FIRE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(157);
        self.values.insert(157, v as u32);
        self
    }

    pub fn unit_FIRE_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&157) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_NATURE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(158);
        self.values.insert(158, v as u32);
        self
    }

    pub fn unit_NATURE_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&158) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_FROST_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(159);
        self.values.insert(159, v as u32);
        self
    }

    pub fn unit_FROST_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&159) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_SHADOW_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(160);
        self.values.insert(160, v as u32);
        self
    }

    pub fn unit_SHADOW_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&160) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_ARCANE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(161);
        self.values.insert(161, v as u32);
        self
    }

    pub fn unit_ARCANE_RESISTANCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&161) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(162);
        self.values.insert(162, v as u32);
        self
    }

    pub fn unit_BASE_MANA(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&162) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(163);
        self.values.insert(163, v as u32);
        self
    }

    pub fn unit_BASE_HEALTH(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&163) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_BYTES_2(mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) -> Self {
        self.header_set(164);
        self.values.insert(164, u32::from_le_bytes([facial_hair, unknown, bank_bag_slots, rested_state]));
        self
    }

    pub fn unit_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&164) {
            let v = v.to_le_bytes();
            let (facial_hair, unknown, bank_bag_slots, rested_state) = (v[0], v[1], v[2], v[3]);
            Some((facial_hair.try_into().unwrap(), unknown.try_into().unwrap(), bank_bag_slots.try_into().unwrap(), rested_state.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(165);
        self.values.insert(165, v as u32);
        self
    }

    pub fn unit_ATTACK_POWER(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&165) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(166);
        self.values.insert(166, (a as u32) << 16 | b as u32);
        self
    }

    pub fn unit_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&166) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(167);
        self.values.insert(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&167) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(168);
        self.values.insert(168, v as u32);
        self
    }

    pub fn unit_RANGED_ATTACK_POWER(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&168) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(169);
        self.values.insert(169, (a as u32) << 16 | b as u32);
        self
    }

    pub fn unit_RANGED_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&169) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(170);
        self.values.insert(170, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_RANGED_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&170) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_MINRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(171);
        self.values.insert(171, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MINRANGEDDAMAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&171) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_MAXRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(172);
        self.values.insert(172, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_MAXRANGEDDAMAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&172) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_unit_POWER_COST_MODIFIER(mut self, v: i32) -> Self {
        self.header_set(173);
        self.values.insert(173, v as u32);
        self
    }

    pub fn unit_POWER_COST_MODIFIER(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&173) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(180);
        self.values.insert(180, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn unit_POWER_COST_MULTIPLIER(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&180) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_player_DUEL_ARBITER(mut self, v: Guid) -> Self {
        self.header_set(188);
        self.header_set(189);
        self.values.insert(188, v.guid() as u32);
        self.values.insert(189, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_DUEL_ARBITER(&self) -> Option<Guid> {
        let lower = self.values.get(&188);
        let upper = self.values.get(&189);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_FLAGS(mut self, v: i32) -> Self {
        self.header_set(190);
        self.values.insert(190, v as u32);
        self
    }

    pub fn player_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&190) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_GUILDID(mut self, v: i32) -> Self {
        self.header_set(191);
        self.values.insert(191, v as u32);
        self
    }

    pub fn player_GUILDID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&191) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_GUILDRANK(mut self, v: i32) -> Self {
        self.header_set(192);
        self.values.insert(192, v as u32);
        self
    }

    pub fn player_GUILDRANK(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&192) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FEATURES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(193);
        self.values.insert(193, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn player_FEATURES(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&193) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(194);
        self.values.insert(194, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn player_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&194) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_BYTES_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(195);
        self.values.insert(195, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn player_BYTES_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&195) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_DUEL_TEAM(mut self, v: i32) -> Self {
        self.header_set(196);
        self.values.insert(196, v as u32);
        self
    }

    pub fn player_DUEL_TEAM(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&196) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_GUILD_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(197);
        self.values.insert(197, v as u32);
        self
    }

    pub fn player_GUILD_TIMESTAMP(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&197) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_1_1(mut self, v: i32) -> Self {
        self.header_set(198);
        self.values.insert(198, v as u32);
        self
    }

    pub fn player_QUEST_LOG_1_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&198) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_1_2(mut self, v: i32) -> Self {
        self.header_set(199);
        self.values.insert(199, v as u32);
        self
    }

    pub fn player_QUEST_LOG_1_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&199) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_2_1(mut self, v: i32) -> Self {
        self.header_set(201);
        self.values.insert(201, v as u32);
        self
    }

    pub fn player_QUEST_LOG_2_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&201) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_2_2(mut self, v: i32) -> Self {
        self.header_set(202);
        self.values.insert(202, v as u32);
        self
    }

    pub fn player_QUEST_LOG_2_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&202) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_3_1(mut self, v: i32) -> Self {
        self.header_set(204);
        self.values.insert(204, v as u32);
        self
    }

    pub fn player_QUEST_LOG_3_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&204) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_3_2(mut self, v: i32) -> Self {
        self.header_set(205);
        self.values.insert(205, v as u32);
        self
    }

    pub fn player_QUEST_LOG_3_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&205) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_4_1(mut self, v: i32) -> Self {
        self.header_set(207);
        self.values.insert(207, v as u32);
        self
    }

    pub fn player_QUEST_LOG_4_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&207) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_4_2(mut self, v: i32) -> Self {
        self.header_set(208);
        self.values.insert(208, v as u32);
        self
    }

    pub fn player_QUEST_LOG_4_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&208) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_5_1(mut self, v: i32) -> Self {
        self.header_set(210);
        self.values.insert(210, v as u32);
        self
    }

    pub fn player_QUEST_LOG_5_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&210) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_5_2(mut self, v: i32) -> Self {
        self.header_set(211);
        self.values.insert(211, v as u32);
        self
    }

    pub fn player_QUEST_LOG_5_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&211) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_6_1(mut self, v: i32) -> Self {
        self.header_set(213);
        self.values.insert(213, v as u32);
        self
    }

    pub fn player_QUEST_LOG_6_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&213) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_6_2(mut self, v: i32) -> Self {
        self.header_set(214);
        self.values.insert(214, v as u32);
        self
    }

    pub fn player_QUEST_LOG_6_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&214) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_7_1(mut self, v: i32) -> Self {
        self.header_set(216);
        self.values.insert(216, v as u32);
        self
    }

    pub fn player_QUEST_LOG_7_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&216) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_7_2(mut self, v: i32) -> Self {
        self.header_set(217);
        self.values.insert(217, v as u32);
        self
    }

    pub fn player_QUEST_LOG_7_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&217) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_8_1(mut self, v: i32) -> Self {
        self.header_set(219);
        self.values.insert(219, v as u32);
        self
    }

    pub fn player_QUEST_LOG_8_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&219) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_8_2(mut self, v: i32) -> Self {
        self.header_set(220);
        self.values.insert(220, v as u32);
        self
    }

    pub fn player_QUEST_LOG_8_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&220) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_9_1(mut self, v: i32) -> Self {
        self.header_set(222);
        self.values.insert(222, v as u32);
        self
    }

    pub fn player_QUEST_LOG_9_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&222) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_9_2(mut self, v: i32) -> Self {
        self.header_set(223);
        self.values.insert(223, v as u32);
        self
    }

    pub fn player_QUEST_LOG_9_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&223) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_10_1(mut self, v: i32) -> Self {
        self.header_set(225);
        self.values.insert(225, v as u32);
        self
    }

    pub fn player_QUEST_LOG_10_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&225) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_10_2(mut self, v: i32) -> Self {
        self.header_set(226);
        self.values.insert(226, v as u32);
        self
    }

    pub fn player_QUEST_LOG_10_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&226) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_11_1(mut self, v: i32) -> Self {
        self.header_set(228);
        self.values.insert(228, v as u32);
        self
    }

    pub fn player_QUEST_LOG_11_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&228) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_11_2(mut self, v: i32) -> Self {
        self.header_set(229);
        self.values.insert(229, v as u32);
        self
    }

    pub fn player_QUEST_LOG_11_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&229) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_12_1(mut self, v: i32) -> Self {
        self.header_set(231);
        self.values.insert(231, v as u32);
        self
    }

    pub fn player_QUEST_LOG_12_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&231) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_12_2(mut self, v: i32) -> Self {
        self.header_set(232);
        self.values.insert(232, v as u32);
        self
    }

    pub fn player_QUEST_LOG_12_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&232) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_13_1(mut self, v: i32) -> Self {
        self.header_set(234);
        self.values.insert(234, v as u32);
        self
    }

    pub fn player_QUEST_LOG_13_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&234) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_13_2(mut self, v: i32) -> Self {
        self.header_set(235);
        self.values.insert(235, v as u32);
        self
    }

    pub fn player_QUEST_LOG_13_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&235) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_14_1(mut self, v: i32) -> Self {
        self.header_set(237);
        self.values.insert(237, v as u32);
        self
    }

    pub fn player_QUEST_LOG_14_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&237) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_14_2(mut self, v: i32) -> Self {
        self.header_set(238);
        self.values.insert(238, v as u32);
        self
    }

    pub fn player_QUEST_LOG_14_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&238) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_15_1(mut self, v: i32) -> Self {
        self.header_set(240);
        self.values.insert(240, v as u32);
        self
    }

    pub fn player_QUEST_LOG_15_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&240) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_15_2(mut self, v: i32) -> Self {
        self.header_set(241);
        self.values.insert(241, v as u32);
        self
    }

    pub fn player_QUEST_LOG_15_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&241) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_16_1(mut self, v: i32) -> Self {
        self.header_set(243);
        self.values.insert(243, v as u32);
        self
    }

    pub fn player_QUEST_LOG_16_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&243) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_16_2(mut self, v: i32) -> Self {
        self.header_set(244);
        self.values.insert(244, v as u32);
        self
    }

    pub fn player_QUEST_LOG_16_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&244) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_17_1(mut self, v: i32) -> Self {
        self.header_set(246);
        self.values.insert(246, v as u32);
        self
    }

    pub fn player_QUEST_LOG_17_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&246) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_17_2(mut self, v: i32) -> Self {
        self.header_set(247);
        self.values.insert(247, v as u32);
        self
    }

    pub fn player_QUEST_LOG_17_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&247) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_18_1(mut self, v: i32) -> Self {
        self.header_set(249);
        self.values.insert(249, v as u32);
        self
    }

    pub fn player_QUEST_LOG_18_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&249) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_18_2(mut self, v: i32) -> Self {
        self.header_set(250);
        self.values.insert(250, v as u32);
        self
    }

    pub fn player_QUEST_LOG_18_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&250) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_19_1(mut self, v: i32) -> Self {
        self.header_set(252);
        self.values.insert(252, v as u32);
        self
    }

    pub fn player_QUEST_LOG_19_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&252) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_19_2(mut self, v: i32) -> Self {
        self.header_set(253);
        self.values.insert(253, v as u32);
        self
    }

    pub fn player_QUEST_LOG_19_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&253) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_20_1(mut self, v: i32) -> Self {
        self.header_set(255);
        self.values.insert(255, v as u32);
        self
    }

    pub fn player_QUEST_LOG_20_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&255) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_20_2(mut self, v: i32) -> Self {
        self.header_set(256);
        self.values.insert(256, v as u32);
        self
    }

    pub fn player_QUEST_LOG_20_2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&256) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_1_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(258);
        self.header_set(259);
        self.values.insert(258, v.guid() as u32);
        self.values.insert(259, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_1_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&258);
        let upper = self.values.get(&259);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_1_0(mut self, v: i32) -> Self {
        self.header_set(260);
        self.values.insert(260, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_1_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&260) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_1_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(268);
        self.values.insert(268, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_1_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&268) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_2_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(270);
        self.header_set(271);
        self.values.insert(270, v.guid() as u32);
        self.values.insert(271, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_2_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&270);
        let upper = self.values.get(&271);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_2_0(mut self, v: i32) -> Self {
        self.header_set(272);
        self.values.insert(272, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_2_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&272) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_2_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(280);
        self.values.insert(280, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_2_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&280) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_3_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(282);
        self.header_set(283);
        self.values.insert(282, v.guid() as u32);
        self.values.insert(283, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_3_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&282);
        let upper = self.values.get(&283);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_3_0(mut self, v: i32) -> Self {
        self.header_set(284);
        self.values.insert(284, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_3_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&284) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_3_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(292);
        self.values.insert(292, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_3_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&292) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_4_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(294);
        self.header_set(295);
        self.values.insert(294, v.guid() as u32);
        self.values.insert(295, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_4_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&294);
        let upper = self.values.get(&295);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_4_0(mut self, v: i32) -> Self {
        self.header_set(296);
        self.values.insert(296, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_4_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&296) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_4_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(304);
        self.values.insert(304, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_4_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&304) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_5_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(306);
        self.header_set(307);
        self.values.insert(306, v.guid() as u32);
        self.values.insert(307, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_5_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&306);
        let upper = self.values.get(&307);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_5_0(mut self, v: i32) -> Self {
        self.header_set(308);
        self.values.insert(308, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_5_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&308) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_5_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(316);
        self.values.insert(316, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_5_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&316) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_6_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(318);
        self.header_set(319);
        self.values.insert(318, v.guid() as u32);
        self.values.insert(319, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_6_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&318);
        let upper = self.values.get(&319);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_6_0(mut self, v: i32) -> Self {
        self.header_set(320);
        self.values.insert(320, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_6_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&320) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_6_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(328);
        self.values.insert(328, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_6_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&328) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_7_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(330);
        self.header_set(331);
        self.values.insert(330, v.guid() as u32);
        self.values.insert(331, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_7_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&330);
        let upper = self.values.get(&331);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_7_0(mut self, v: i32) -> Self {
        self.header_set(332);
        self.values.insert(332, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_7_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&332) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_7_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(340);
        self.values.insert(340, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_7_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&340) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_8_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(342);
        self.header_set(343);
        self.values.insert(342, v.guid() as u32);
        self.values.insert(343, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_8_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&342);
        let upper = self.values.get(&343);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_8_0(mut self, v: i32) -> Self {
        self.header_set(344);
        self.values.insert(344, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_8_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&344) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_8_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(352);
        self.values.insert(352, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_8_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&352) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_9_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(354);
        self.header_set(355);
        self.values.insert(354, v.guid() as u32);
        self.values.insert(355, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_9_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&354);
        let upper = self.values.get(&355);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_9_0(mut self, v: i32) -> Self {
        self.header_set(356);
        self.values.insert(356, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_9_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&356) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_9_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(364);
        self.values.insert(364, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_9_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&364) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_10_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(366);
        self.header_set(367);
        self.values.insert(366, v.guid() as u32);
        self.values.insert(367, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_10_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&366);
        let upper = self.values.get(&367);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_10_0(mut self, v: i32) -> Self {
        self.header_set(368);
        self.values.insert(368, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_10_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&368) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_10_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(376);
        self.values.insert(376, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_10_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&376) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_11_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(378);
        self.header_set(379);
        self.values.insert(378, v.guid() as u32);
        self.values.insert(379, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_11_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&378);
        let upper = self.values.get(&379);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_11_0(mut self, v: i32) -> Self {
        self.header_set(380);
        self.values.insert(380, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_11_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&380) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_11_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(388);
        self.values.insert(388, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_11_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&388) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_12_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(390);
        self.header_set(391);
        self.values.insert(390, v.guid() as u32);
        self.values.insert(391, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_12_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&390);
        let upper = self.values.get(&391);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_12_0(mut self, v: i32) -> Self {
        self.header_set(392);
        self.values.insert(392, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_12_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&392) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_12_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(400);
        self.values.insert(400, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_12_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&400) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_13_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(402);
        self.header_set(403);
        self.values.insert(402, v.guid() as u32);
        self.values.insert(403, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_13_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&402);
        let upper = self.values.get(&403);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_13_0(mut self, v: i32) -> Self {
        self.header_set(404);
        self.values.insert(404, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_13_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&404) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_13_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(412);
        self.values.insert(412, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_13_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&412) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_14_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(414);
        self.header_set(415);
        self.values.insert(414, v.guid() as u32);
        self.values.insert(415, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_14_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&414);
        let upper = self.values.get(&415);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_14_0(mut self, v: i32) -> Self {
        self.header_set(416);
        self.values.insert(416, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_14_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&416) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_14_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(424);
        self.values.insert(424, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_14_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&424) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_15_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(426);
        self.header_set(427);
        self.values.insert(426, v.guid() as u32);
        self.values.insert(427, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_15_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&426);
        let upper = self.values.get(&427);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_15_0(mut self, v: i32) -> Self {
        self.header_set(428);
        self.values.insert(428, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_15_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&428) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_15_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(436);
        self.values.insert(436, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_15_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&436) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_16_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(438);
        self.header_set(439);
        self.values.insert(438, v.guid() as u32);
        self.values.insert(439, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_16_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&438);
        let upper = self.values.get(&439);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_16_0(mut self, v: i32) -> Self {
        self.header_set(440);
        self.values.insert(440, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_16_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&440) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_16_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(448);
        self.values.insert(448, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_16_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&448) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_17_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(450);
        self.header_set(451);
        self.values.insert(450, v.guid() as u32);
        self.values.insert(451, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_17_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&450);
        let upper = self.values.get(&451);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_17_0(mut self, v: i32) -> Self {
        self.header_set(452);
        self.values.insert(452, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_17_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&452) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_17_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(460);
        self.values.insert(460, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_17_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&460) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_18_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(462);
        self.header_set(463);
        self.values.insert(462, v.guid() as u32);
        self.values.insert(463, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_18_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&462);
        let upper = self.values.get(&463);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_18_0(mut self, v: i32) -> Self {
        self.header_set(464);
        self.values.insert(464, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_18_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&464) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_18_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(472);
        self.values.insert(472, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_18_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&472) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_19_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(474);
        self.header_set(475);
        self.values.insert(474, v.guid() as u32);
        self.values.insert(475, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_19_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&474);
        let upper = self.values.get(&475);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_19_0(mut self, v: i32) -> Self {
        self.header_set(476);
        self.values.insert(476, v as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_19_0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&476) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_19_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(484);
        self.values.insert(484, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_VISIBLE_ITEM_19_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&484) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_INV_SLOT_HEAD(mut self, v: Guid) -> Self {
        self.header_set(486);
        self.header_set(487);
        self.values.insert(486, v.guid() as u32);
        self.values.insert(487, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_FIELD_INV_SLOT_HEAD(&self) -> Option<Guid> {
        let lower = self.values.get(&486);
        let upper = self.values.get(&487);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_PACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(532);
        self.header_set(533);
        self.values.insert(532, v.guid() as u32);
        self.values.insert(533, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_FIELD_PACK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&532);
        let upper = self.values.get(&533);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_BANK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(564);
        self.header_set(565);
        self.values.insert(564, v.guid() as u32);
        self.values.insert(565, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_FIELD_BANK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&564);
        let upper = self.values.get(&565);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(612);
        self.header_set(613);
        self.values.insert(612, v.guid() as u32);
        self.values.insert(613, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_FIELD_BANKBAG_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&612);
        let upper = self.values.get(&613);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(624);
        self.header_set(625);
        self.values.insert(624, v.guid() as u32);
        self.values.insert(625, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&624);
        let upper = self.values.get(&625);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_KEYRING_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(648);
        self.header_set(649);
        self.values.insert(648, v.guid() as u32);
        self.values.insert(649, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_FIELD_KEYRING_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&648);
        let upper = self.values.get(&649);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_FARSIGHT(mut self, v: Guid) -> Self {
        self.header_set(712);
        self.header_set(713);
        self.values.insert(712, v.guid() as u32);
        self.values.insert(713, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_FARSIGHT(&self) -> Option<Guid> {
        let lower = self.values.get(&712);
        let upper = self.values.get(&713);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_COMBO_TARGET(mut self, v: Guid) -> Self {
        self.header_set(714);
        self.header_set(715);
        self.values.insert(714, v.guid() as u32);
        self.values.insert(715, (v.guid() >> 32) as u32);
        self
    }

    pub fn player_FIELD_COMBO_TARGET(&self) -> Option<Guid> {
        let lower = self.values.get(&714);
        let upper = self.values.get(&715);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_player_XP(mut self, v: i32) -> Self {
        self.header_set(716);
        self.values.insert(716, v as u32);
        self
    }

    pub fn player_XP(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&716) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_NEXT_LEVEL_XP(mut self, v: i32) -> Self {
        self.header_set(717);
        self.values.insert(717, v as u32);
        self
    }

    pub fn player_NEXT_LEVEL_XP(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&717) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_SKILL_INFO_1_1(mut self, a: u16, b: u16) -> Self {
        self.header_set(718);
        self.values.insert(718, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_SKILL_INFO_1_1(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&718) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_CHARACTER_POINTS1(mut self, v: i32) -> Self {
        self.header_set(1102);
        self.values.insert(1102, v as u32);
        self
    }

    pub fn player_CHARACTER_POINTS1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1102) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_CHARACTER_POINTS2(mut self, v: i32) -> Self {
        self.header_set(1103);
        self.values.insert(1103, v as u32);
        self
    }

    pub fn player_CHARACTER_POINTS2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1103) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_TRACK_CREATURES(mut self, v: i32) -> Self {
        self.header_set(1104);
        self.values.insert(1104, v as u32);
        self
    }

    pub fn player_TRACK_CREATURES(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1104) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_TRACK_RESOURCES(mut self, v: i32) -> Self {
        self.header_set(1105);
        self.values.insert(1105, v as u32);
        self
    }

    pub fn player_TRACK_RESOURCES(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1105) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_BLOCK_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1106);
        self.values.insert(1106, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn player_BLOCK_PERCENTAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&1106) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_player_DODGE_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1107);
        self.values.insert(1107, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn player_DODGE_PERCENTAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&1107) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_player_PARRY_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1108);
        self.values.insert(1108, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn player_PARRY_PERCENTAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&1108) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_player_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1109);
        self.values.insert(1109, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn player_CRIT_PERCENTAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&1109) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_player_RANGED_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1110);
        self.values.insert(1110, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn player_RANGED_CRIT_PERCENTAGE(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&1110) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_player_EXPLORED_ZONES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1111);
        self.values.insert(1111, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn player_EXPLORED_ZONES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1111) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_REST_STATE_EXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(1175);
        self.values.insert(1175, v as u32);
        self
    }

    pub fn player_REST_STATE_EXPERIENCE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1175) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_COINAGE(mut self, v: i32) -> Self {
        self.header_set(1176);
        self.values.insert(1176, v as u32);
        self
    }

    pub fn player_FIELD_COINAGE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1176) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_POSSTAT0(mut self, v: i32) -> Self {
        self.header_set(1177);
        self.values.insert(1177, v as u32);
        self
    }

    pub fn player_FIELD_POSSTAT0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1177) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_POSSTAT1(mut self, v: i32) -> Self {
        self.header_set(1178);
        self.values.insert(1178, v as u32);
        self
    }

    pub fn player_FIELD_POSSTAT1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1178) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_POSSTAT2(mut self, v: i32) -> Self {
        self.header_set(1179);
        self.values.insert(1179, v as u32);
        self
    }

    pub fn player_FIELD_POSSTAT2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1179) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_POSSTAT3(mut self, v: i32) -> Self {
        self.header_set(1180);
        self.values.insert(1180, v as u32);
        self
    }

    pub fn player_FIELD_POSSTAT3(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1180) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_POSSTAT4(mut self, v: i32) -> Self {
        self.header_set(1181);
        self.values.insert(1181, v as u32);
        self
    }

    pub fn player_FIELD_POSSTAT4(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1181) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_NEGSTAT0(mut self, v: i32) -> Self {
        self.header_set(1182);
        self.values.insert(1182, v as u32);
        self
    }

    pub fn player_FIELD_NEGSTAT0(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1182) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_NEGSTAT1(mut self, v: i32) -> Self {
        self.header_set(1183);
        self.values.insert(1183, v as u32);
        self
    }

    pub fn player_FIELD_NEGSTAT1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1183) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_NEGSTAT2(mut self, v: i32) -> Self {
        self.header_set(1184);
        self.values.insert(1184, v as u32);
        self
    }

    pub fn player_FIELD_NEGSTAT2(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1184) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_NEGSTAT3(mut self, v: i32) -> Self {
        self.header_set(1185);
        self.values.insert(1185, v as u32);
        self
    }

    pub fn player_FIELD_NEGSTAT3(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1185) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_NEGSTAT4(mut self, v: i32) -> Self {
        self.header_set(1186);
        self.values.insert(1186, v as u32);
        self
    }

    pub fn player_FIELD_NEGSTAT4(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1186) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_RESISTANCEBUFFMODSPOSITIVE(mut self, v: i32) -> Self {
        self.header_set(1187);
        self.values.insert(1187, v as u32);
        self
    }

    pub fn player_FIELD_RESISTANCEBUFFMODSPOSITIVE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1187) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_RESISTANCEBUFFMODSNEGATIVE(mut self, v: i32) -> Self {
        self.header_set(1194);
        self.values.insert(1194, v as u32);
        self
    }

    pub fn player_FIELD_RESISTANCEBUFFMODSNEGATIVE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1194) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_POS(mut self, v: i32) -> Self {
        self.header_set(1201);
        self.values.insert(1201, v as u32);
        self
    }

    pub fn player_FIELD_MOD_DAMAGE_DONE_POS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1201) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_NEG(mut self, v: i32) -> Self {
        self.header_set(1208);
        self.values.insert(1208, v as u32);
        self
    }

    pub fn player_FIELD_MOD_DAMAGE_DONE_NEG(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1208) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_PCT(mut self, v: i32) -> Self {
        self.header_set(1215);
        self.values.insert(1215, v as u32);
        self
    }

    pub fn player_FIELD_MOD_DAMAGE_DONE_PCT(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1215) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1222);
        self.values.insert(1222, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn player_FIELD_BYTES(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1222) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_AMMO_ID(mut self, v: i32) -> Self {
        self.header_set(1223);
        self.values.insert(1223, v as u32);
        self
    }

    pub fn player_AMMO_ID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1223) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_SELF_RES_SPELL(mut self, v: i32) -> Self {
        self.header_set(1224);
        self.values.insert(1224, v as u32);
        self
    }

    pub fn player_SELF_RES_SPELL(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1224) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_PVP_MEDALS(mut self, v: i32) -> Self {
        self.header_set(1225);
        self.values.insert(1225, v as u32);
        self
    }

    pub fn player_FIELD_PVP_MEDALS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1225) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_BUYBACK_PRICE_1(mut self, v: i32) -> Self {
        self.header_set(1226);
        self.values.insert(1226, v as u32);
        self
    }

    pub fn player_FIELD_BUYBACK_PRICE_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1226) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_BUYBACK_TIMESTAMP_1(mut self, v: i32) -> Self {
        self.header_set(1238);
        self.values.insert(1238, v as u32);
        self
    }

    pub fn player_FIELD_BUYBACK_TIMESTAMP_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1238) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_SESSION_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1250);
        self.values.insert(1250, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_FIELD_SESSION_KILLS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1250) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_YESTERDAY_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1251);
        self.values.insert(1251, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_FIELD_YESTERDAY_KILLS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1251) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_LAST_WEEK_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1252);
        self.values.insert(1252, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_FIELD_LAST_WEEK_KILLS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1252) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_THIS_WEEK_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1253);
        self.values.insert(1253, (a as u32) << 16 | b as u32);
        self
    }

    pub fn player_FIELD_THIS_WEEK_KILLS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1253) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_THIS_WEEK_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1254);
        self.values.insert(1254, v as u32);
        self
    }

    pub fn player_FIELD_THIS_WEEK_CONTRIBUTION(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1254) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_LIFETIME_HONORBALE_KILLS(mut self, v: i32) -> Self {
        self.header_set(1255);
        self.values.insert(1255, v as u32);
        self
    }

    pub fn player_FIELD_LIFETIME_HONORBALE_KILLS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1255) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_LIFETIME_DISHONORBALE_KILLS(mut self, v: i32) -> Self {
        self.header_set(1256);
        self.values.insert(1256, v as u32);
        self
    }

    pub fn player_FIELD_LIFETIME_DISHONORBALE_KILLS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1256) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_YESTERDAY_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1257);
        self.values.insert(1257, v as u32);
        self
    }

    pub fn player_FIELD_YESTERDAY_CONTRIBUTION(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1257) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_LAST_WEEK_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1258);
        self.values.insert(1258, v as u32);
        self
    }

    pub fn player_FIELD_LAST_WEEK_CONTRIBUTION(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1258) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_LAST_WEEK_RANK(mut self, v: i32) -> Self {
        self.header_set(1259);
        self.values.insert(1259, v as u32);
        self
    }

    pub fn player_FIELD_LAST_WEEK_RANK(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1259) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_BYTES2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1260);
        self.values.insert(1260, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn player_FIELD_BYTES2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1260) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_WATCHED_FACTION_INDEX(mut self, v: i32) -> Self {
        self.header_set(1261);
        self.values.insert(1261, v as u32);
        self
    }

    pub fn player_FIELD_WATCHED_FACTION_INDEX(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1261) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_player_FIELD_COMBAT_RATING_1(mut self, v: i32) -> Self {
        self.header_set(1262);
        self.values.insert(1262, v as u32);
        self
    }

    pub fn player_FIELD_COMBAT_RATING_1(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&1262) {
            Some(*v as i32)
        } else {
            None
        }
    }

}

impl UpdateGameObject {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&3) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&4) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_gameobject_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn gameobject_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_gameobject_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(8);
        self.values.insert(8, v as u32);
        self
    }

    pub fn gameobject_DISPLAYID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&8) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_gameobject_FLAGS(mut self, v: i32) -> Self {
        self.header_set(9);
        self.values.insert(9, v as u32);
        self
    }

    pub fn gameobject_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&9) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_gameobject_ROTATION(mut self, v: f32) -> Self {
        self.header_set(10);
        self.values.insert(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn gameobject_ROTATION(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&10) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_gameobject_STATE(mut self, v: i32) -> Self {
        self.header_set(14);
        self.values.insert(14, v as u32);
        self
    }

    pub fn gameobject_STATE(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&14) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_gameobject_POS_X(mut self, v: f32) -> Self {
        self.header_set(15);
        self.values.insert(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn gameobject_POS_X(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&15) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_gameobject_POS_Y(mut self, v: f32) -> Self {
        self.header_set(16);
        self.values.insert(16, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn gameobject_POS_Y(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&16) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_gameobject_POS_Z(mut self, v: f32) -> Self {
        self.header_set(17);
        self.values.insert(17, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn gameobject_POS_Z(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&17) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_gameobject_FACING(mut self, v: f32) -> Self {
        self.header_set(18);
        self.values.insert(18, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn gameobject_FACING(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&18) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_gameobject_DYN_FLAGS(mut self, v: i32) -> Self {
        self.header_set(19);
        self.values.insert(19, v as u32);
        self
    }

    pub fn gameobject_DYN_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&19) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_gameobject_FACTION(mut self, v: i32) -> Self {
        self.header_set(20);
        self.values.insert(20, v as u32);
        self
    }

    pub fn gameobject_FACTION(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&20) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_gameobject_TYPE_ID(mut self, v: i32) -> Self {
        self.header_set(21);
        self.values.insert(21, v as u32);
        self
    }

    pub fn gameobject_TYPE_ID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&21) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_gameobject_LEVEL(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn gameobject_LEVEL(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&22) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_gameobject_ARTKIT(mut self, v: i32) -> Self {
        self.header_set(23);
        self.values.insert(23, v as u32);
        self
    }

    pub fn gameobject_ARTKIT(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&23) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_gameobject_ANIMPROGRESS(mut self, v: i32) -> Self {
        self.header_set(24);
        self.values.insert(24, v as u32);
        self
    }

    pub fn gameobject_ANIMPROGRESS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&24) {
            Some(*v as i32)
        } else {
            None
        }
    }

}

impl UpdateDynamicObject {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&3) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&4) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_dynamicobject_CASTER(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn dynamicobject_CASTER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_dynamicobject_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(8);
        self.values.insert(8, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn dynamicobject_BYTES(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&8) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_dynamicobject_SPELLID(mut self, v: i32) -> Self {
        self.header_set(9);
        self.values.insert(9, v as u32);
        self
    }

    pub fn dynamicobject_SPELLID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&9) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_dynamicobject_RADIUS(mut self, v: f32) -> Self {
        self.header_set(10);
        self.values.insert(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn dynamicobject_RADIUS(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&10) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_dynamicobject_POS_X(mut self, v: f32) -> Self {
        self.header_set(11);
        self.values.insert(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn dynamicobject_POS_X(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&11) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_dynamicobject_POS_Y(mut self, v: f32) -> Self {
        self.header_set(12);
        self.values.insert(12, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn dynamicobject_POS_Y(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&12) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_dynamicobject_POS_Z(mut self, v: f32) -> Self {
        self.header_set(13);
        self.values.insert(13, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn dynamicobject_POS_Z(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&13) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_dynamicobject_FACING(mut self, v: f32) -> Self {
        self.header_set(14);
        self.values.insert(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn dynamicobject_FACING(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&14) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

}

impl UpdateCorpse {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&3) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&4) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_corpse_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn corpse_OWNER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        if let Some(lower) = lower {
            Some(Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
        } else {
            None
        }
    }

    pub fn set_corpse_FACING(mut self, v: f32) -> Self {
        self.header_set(8);
        self.values.insert(8, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn corpse_FACING(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&8) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_corpse_POS_X(mut self, v: f32) -> Self {
        self.header_set(9);
        self.values.insert(9, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn corpse_POS_X(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&9) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_corpse_POS_Y(mut self, v: f32) -> Self {
        self.header_set(10);
        self.values.insert(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn corpse_POS_Y(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&10) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_corpse_POS_Z(mut self, v: f32) -> Self {
        self.header_set(11);
        self.values.insert(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn corpse_POS_Z(&self) -> Option<f32> {
        if let Some(v) = self.values.get(&11) {
            Some(f32::from_le_bytes(v.to_le_bytes()))
        } else {
            None
        }
    }

    pub fn set_corpse_DISPLAY_ID(mut self, v: i32) -> Self {
        self.header_set(12);
        self.values.insert(12, v as u32);
        self
    }

    pub fn corpse_DISPLAY_ID(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&12) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_corpse_ITEM(mut self, v: i32) -> Self {
        self.header_set(13);
        self.values.insert(13, v as u32);
        self
    }

    pub fn corpse_ITEM(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&13) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_corpse_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(32);
        self.values.insert(32, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn corpse_BYTES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&32) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(33);
        self.values.insert(33, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn corpse_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&33) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_GUILD(mut self, v: i32) -> Self {
        self.header_set(34);
        self.values.insert(34, v as u32);
        self
    }

    pub fn corpse_GUILD(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&34) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_corpse_FLAGS(mut self, v: i32) -> Self {
        self.header_set(35);
        self.values.insert(35, v as u32);
        self
    }

    pub fn corpse_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&35) {
            Some(*v as i32)
        } else {
            None
        }
    }

    pub fn set_corpse_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(36);
        self.values.insert(36, v as u32);
        self
    }

    pub fn corpse_DYNAMIC_FLAGS(&self) -> Option<i32> {
        if let Some(v) = self.values.get(&36) {
            Some(*v as i32)
        } else {
            None
        }
    }

}

