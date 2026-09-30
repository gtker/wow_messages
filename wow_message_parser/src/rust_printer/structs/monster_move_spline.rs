use crate::parser::types::container::Container;
use crate::parser::types::ty::Type;

pub(crate) fn uses_wrath_monster_move_spline_encoding(container: &Container) -> bool {
    container
        .tags()
        .contains_wrath()
        && container
            .all_definitions_transitively()
            .iter()
            .any(|definition| {
                matches!(
                    definition.ty(),
                    Type::MonsterMoveSplines | Type::FullMonsterMoveSpline
                )
            })
}
