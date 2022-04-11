use std::io;
use std::io::{Read, Write};
use crate::Guid;

pub struct UpdateObject {
    pub guid: Guid,
    pub object_type: i32,
    pub entry: i32,
    pub scale: f32,
}

pub struct UpdateItem {
    owner: Guid,
    contained: Guid,
    creator: Guid,
    gift_creator: Guid,
    stack_count: i32,
    duration: i32,
    spell_charges: [i32; 5],
    flags: u32,
    enchantment: [i32; 21],
    property_seed: i32,
    random_properties_id: i32,
    item_text_id: i32,
    durability: i32,
    max_durability: i32,
}

pub struct UpdateContainer {
    number_of_slots: i32,
    slots: [Guid; 36],
}

pub struct UpdateUnit {
    charm: Guid,
    summon: Guid,
    charmed_by: Guid,
    summoned_by: Guid,
    created_by: Guid,
    target: Guid,
    persuaded: Guid,
    channel_object: Guid,
    health: i32,
    power1: i32,
    power2: i32,
    power3: i32,
    power4: i32,
    power5: i32,
    max_health: i32,
    max_power1: i32,
    max_power2: i32,
    max_power3: i32,
    max_power4: i32,
    max_power5: i32,
    level: i32,
    faction_template: i32,
    bytes_0: [u8; 4],
    item_slot_display: [i32; 3],
    item_info: [u8; 6 * 4],
    flags: u32,
    auras: [i32; 48],
    aura_flags: [u8; 6 * 4],
    aura_levels: [u8; 12 * 4],
    aura_applications: [u8; 12 * 4],
    aura_state: i32,
    base_attack_time: [i32; 2],
    // TODO Add missing
}

pub struct UpdateMask {
    pub object: Option<UpdateObject>,
    pub item: Option<UpdateItem>,
    pub container: Option<UpdateContainer>,
}

impl UpdateMask {
    pub fn read(r: &mut impl Read) -> Result<Self, io::Error> {
        todo!()
    }

    pub fn write(&self, w: &mut impl Write) -> Result<(), io::Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn most_minimal_example() {
        let b = [
           2_u8, // Amount of u32 mask blocks that will follow
           // Mask blocks
           7, 0, 64, 0,   16, 0, 0, 0,
           // End of mask blocks
           4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
           25, 0, 0, 0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
           100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
           1, // UNIT_FIELD_BYTES[0] // Race (Human)
           1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
           1, // UNIT_FIELD_BYTES[2] // Gender (Female)
           1, // UNIT_FIELD_BYTES[3] // Power (Rage)
        ];

    }

    #[test]
    fn small_example() {
        let b = [
           5_u8, // Amount of u32 mask blocks that will follow
           // Mask blocks
           23, 0, 64, 16,    28, 0, 0, 0,    0, 0, 0, 0,    0, 0, 0, 0,    24, 0, 0, 0,
           // End of mask blocks
           4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
           25, 0, 0, 0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
           0, 0, 128, 63, // Scale (1.0)
           100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
           100, 0, 0, 0, // UNIT_FIELD_MAXHEALTH (100)
           1, 0, 0, 0, // UNIT_FIELD_LEVEL (1)
           1, 0, 0, 0, // UNIT_FIELD_FACTIONTEMPLATE (1)
           1, // UNIT_FIELD_BYTES[0] // Race (Human)
           1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
           1, // UNIT_FIELD_BYTES[2] // Gender (Female)
           1, // UNIT_FIELD_BYTES[3] // Power (Rage)
           50, 0, 0, 0, // UNIT_FIELD_DISPLAYD (50, Human Female)
           50, 0, 0, 0, // UNIT_FIELD_NATIVEDISPLAYID (50, Human Female)
        ];
    }
}
