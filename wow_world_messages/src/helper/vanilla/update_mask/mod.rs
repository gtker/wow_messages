#[allow(clippy::missing_panics_doc)]
mod impls;
mod indices;

pub use indices::*;

use crate::helper::update_mask_common::{
    update_item, update_mask, CONTAINER, CORPSE, DYNAMICOBJECT, GAMEOBJECT, ITEM, PLAYER, UNIT,
};

update_item!(UpdateItem, UpdateItemBuilder, ITEM);
update_item!(UpdateContainer, UpdateContainerBuilder, ITEM | CONTAINER);
update_item!(UpdateUnit, UpdateUnitBuilder, UNIT);
update_item!(UpdatePlayer, UpdatePlayerBuilder, UNIT | PLAYER);
update_item!(UpdateGameObject, UpdateGameObjectBuilder, GAMEOBJECT);
update_item!(
    UpdateDynamicObject,
    UpdateDynamicObjectBuilder,
    DYNAMICOBJECT
);
update_item!(UpdateCorpse, UpdateCorpseBuilder, CORPSE);

update_mask!();

#[cfg(test)]
mod test {
    use crate::vanilla::update_mask::indices::SkillInfoIndex;
    use crate::vanilla::update_mask::UpdatePlayer;
    use crate::vanilla::{Class, Gender, Power, Race};
    use crate::vanilla::{SkillInfo, UpdateMask};
    use crate::Guid;
    use wow_world_base::vanilla::Skill;

    #[test]
    fn most_minimal_example() {
        let b = [
            2_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            7, 0, 64, 0, 16, 0, 0, 0, // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
            100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
            1, // UNIT_FIELD_BYTES[0] // Race (Human)
            1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
            1, // UNIT_FIELD_BYTES[2] // Gender (Female)
            1, // UNIT_FIELD_BYTES[3] // Power (Rage)
        ];

        let update_mask = UpdatePlayer::builder()
            .set_object_guid(Guid::new(4))
            .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_unit_health(100)
            .finalize();
        let update_mask = UpdateMask::Player(update_mask);

        assert_eq!(b.len(), update_mask.size());
        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn skill_info_in_out() {
        let b = [
            23_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            7, 0, 64, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 192, 1, 0, 4, 0, 0, 0, 0, 0, 0,
            0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
            100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
            1, // UNIT_FIELD_BYTES[0] // Race (Human)
            1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
            1, // UNIT_FIELD_BYTES[2] // Gender (Female)
            1, // UNIT_FIELD_BYTES[3] // Power (Rage)
            98, 0, 5, 0, // PLAYER_SKILL_INFO_1_1
            44, 1, 46, 1, // PLAYER_SKILL_INFO_1_2
            7, 0, 10, 0, // PLAYER_SKILL_INFO_1_3
        ];

        let skill_info = SkillInfo::new(Skill::LanguageCommon, 5, 300, 302, 7, 10);

        let update_mask = UpdateMask::Player(
            UpdatePlayer::builder()
                .set_object_guid(Guid::new(4))
                .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
                .set_unit_health(100)
                .set_player_skill_info(skill_info, SkillInfoIndex::Index0)
                .finalize(),
        );

        assert_eq!(b.len(), update_mask.size());

        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
        let update_mask = UpdateMask::read(&mut b.as_slice()).unwrap();
        match update_mask {
            UpdateMask::Player(p) => {
                assert_eq!(
                    p.player_skill_info(SkillInfoIndex::Index0).unwrap(),
                    skill_info
                );
                assert_eq!(p.player_skill_info(SkillInfoIndex::Index1), None)
            }
            _ => panic!(),
        }
    }

    #[test]
    fn reset_header_add_one() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
        ];

        let mut update_mask = UpdatePlayer::builder()
            .set_object_guid(Guid::new(4))
            .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_scale_x(1.0)
            .set_unit_health(100)
            .set_unit_maxhealth(100)
            .set_unit_level(1)
            .set_unit_factiontemplate(1)
            .set_unit_displayid(50)
            .set_unit_nativedisplayid(50)
            .finalize();
        update_mask.dirty_reset();

        update_mask.set_object_guid(4_u64.into());

        let update_mask = UpdateMask::Player(update_mask);
        assert_eq!(b.len(), update_mask.size());

        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn reset_header() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,
            // End of mask blocks
            // No value blocks
        ];

        let mut update_mask = UpdatePlayer::builder()
            .set_object_guid(Guid::new(4))
            .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_scale_x(1.0)
            .set_unit_health(100)
            .set_unit_maxhealth(100)
            .set_unit_level(1)
            .set_unit_factiontemplate(1)
            .set_unit_displayid(50)
            .set_unit_nativedisplayid(50)
            .finalize();

        update_mask.dirty_reset();
        let update_mask = UpdateMask::Player(update_mask);

        assert_eq!(b.len(), update_mask.size());
        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn small_example() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            23, 0, 64, 16, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0,
            // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
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

        let update_mask = UpdatePlayer::builder()
            .set_object_guid(Guid::new(4))
            .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_scale_x(1.0)
            .set_unit_health(100)
            .set_unit_maxhealth(100)
            .set_unit_level(1)
            .set_unit_factiontemplate(1)
            .set_unit_displayid(50)
            .set_unit_nativedisplayid(50)
            .finalize();
        let update_mask = UpdateMask::Player(update_mask);

        assert_eq!(b.len(), update_mask.size());
        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn reset_dirty_and_mark_full_dirty() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            23, 0, 64, 16, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0,
            // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
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

        let mut update_mask = UpdatePlayer::builder()
            .set_object_guid(Guid::new(4))
            .set_unit_bytes_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_scale_x(1.0)
            .set_unit_health(100)
            .set_unit_maxhealth(100)
            .set_unit_level(1)
            .set_unit_factiontemplate(1)
            .set_unit_displayid(50)
            .set_unit_nativedisplayid(50)
            .finalize();
        //Fully clear the dirty mask
        update_mask.dirty_reset();
        assert!(!update_mask.has_any_dirty_fields());

        //Mark completely dirty
        update_mask.mark_fully_dirty();
        assert!(update_mask.has_any_dirty_fields());

        let update_mask = UpdateMask::Player(update_mask);

        assert_eq!(b.len(), update_mask.size());
        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();

        //Output should match all previously written fields
        assert_eq!(b.as_slice(), v.as_slice());
    }
    #[test]
    fn update_mask_header_and_dirty_size() {
        //create update mask so header and mask  have size 1
        let mut update_mask = UpdatePlayer::builder()
            .set_object_guid(Guid::new(3))
            .finalize();
        //set a field that would need to increase the size of the header and dirty mask
        update_mask.set_unit_displayid(49);
        //check that the header and dirty mask are the same size
        assert_eq!(update_mask.dirty_mask.len(), update_mask.header.len());
    }
}
