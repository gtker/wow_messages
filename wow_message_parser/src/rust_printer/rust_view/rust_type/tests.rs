use crate::file_info::FileInfo;
use crate::parser::types::definer::{Definer, DefinerField, DefinerValue};
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::tags::{MemberTags, ObjectTags};
use crate::parser::types::ty::Type;
use crate::parser::types::version::{MajorWorldVersion, Version};
use crate::parser::types::IntegerType;
use crate::path_utils::workspace_directory;
use crate::rust_printer::rust_view::rust_type::{
    MonsterMoveSplineLayout, UnsupportedMonsterMoveSplineLayout,
};
use crate::rust_printer::DefinerType;

fn definition(name: &str, ty: Type) -> StructMember {
    StructMember::Definition(StructMemberDefinition::new(
        name.to_string(),
        ty,
        None,
        None,
        true,
        None,
        Default::default(),
    ))
}

fn spline_flag_type(name: &str) -> Type {
    let file_info = FileInfo::new(
        workspace_directory()
            .join("wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm"),
        1,
        1,
    );
    let fields = [("FLYING", "8192"), ("CATMULLROM", "262144")]
        .into_iter()
        .map(|(field_name, value)| {
            DefinerField::new(
                field_name,
                DefinerValue::from_str(value, name, field_name, &file_info),
                MemberTags::new(),
            )
        })
        .collect();
    let definer = Definer::new(
        name.to_string(),
        DefinerType::Flag,
        fields,
        IntegerType::U32,
        ObjectTags::new_with_version(Version::World(MajorWorldVersion::Wrath)),
        vec![],
        file_info,
    );

    Type::Flag {
        e: definer,
        upcast: None,
    }
}

fn spline_choice_with_discriminator(
    variable_name: &str,
    flags: &[&str],
    else_ifs: Vec<IfStatement>,
    original_ty: Type,
) -> IfStatement {
    IfStatement::new(
        variable_name.to_string(),
        Equation::BitwiseAnd {
            values: flags.iter().map(|flag| (*flag).to_string()).collect(),
        },
        vec![definition("full_splines", Type::FullMonsterMoveSpline)],
        else_ifs,
        vec![definition("splines", Type::MonsterMoveSplines)],
        original_ty,
        false,
    )
}

fn spline_choice(variable_name: &str, flags: &[&str], else_ifs: Vec<IfStatement>) -> IfStatement {
    spline_choice_with_discriminator(
        variable_name,
        flags,
        else_ifs,
        spline_flag_type("SplineFlag"),
    )
}

#[test]
fn recognizes_only_supported_wrath_spline_choice() {
    let statement = spline_choice("spline_flags", &["FLYING", "CATMULLROM"], vec![]);

    assert!(matches!(
        MonsterMoveSplineLayout::from_if_statement(&statement),
        Ok(Some(_))
    ));
}

#[test]
fn rejects_unhandled_spline_choice_shapes() {
    let renamed_discriminator = spline_choice("movement_flags", &["FLYING", "CATMULLROM"], vec![]);
    let wrong_discriminator = spline_choice_with_discriminator(
        "spline_flags",
        &["FLYING", "CATMULLROM"],
        vec![],
        Type::MonsterMoveSplines,
    );
    let wrong_flag_type = spline_choice_with_discriminator(
        "spline_flags",
        &["FLYING", "CATMULLROM"],
        vec![],
        spline_flag_type("OtherFlag"),
    );
    let extra_flag = spline_choice(
        "spline_flags",
        &["FLYING", "CATMULLROM", "PARABOLIC"],
        vec![],
    );
    let else_if = IfStatement::new(
        "spline_flags".to_string(),
        Equation::Equals {
            values: vec!["PARABOLIC".to_string()],
        },
        vec![definition("unused", Type::Integer(IntegerType::U8))],
        vec![],
        vec![],
        Type::MonsterMoveSplines,
        false,
    );
    let with_else_if = spline_choice("spline_flags", &["FLYING", "CATMULLROM"], vec![else_if]);
    let extra_branch_member = IfStatement::new(
        "spline_flags".to_string(),
        Equation::BitwiseAnd {
            values: vec!["FLYING".to_string(), "CATMULLROM".to_string()],
        },
        vec![
            definition("full_splines", Type::FullMonsterMoveSpline),
            definition("extra", Type::Integer(IntegerType::U8)),
        ],
        vec![],
        vec![definition("splines", Type::MonsterMoveSplines)],
        spline_flag_type("SplineFlag"),
        false,
    );

    assert!(matches!(
        MonsterMoveSplineLayout::from_if_statement(&renamed_discriminator),
        Ok(Some(_))
    ));
    assert!(matches!(
        MonsterMoveSplineLayout::from_if_statement(&wrong_discriminator),
        Err(UnsupportedMonsterMoveSplineLayout)
    ));
    assert!(matches!(
        MonsterMoveSplineLayout::from_if_statement(&wrong_flag_type),
        Err(UnsupportedMonsterMoveSplineLayout)
    ));
    assert!(matches!(
        MonsterMoveSplineLayout::from_if_statement(&extra_flag),
        Err(UnsupportedMonsterMoveSplineLayout)
    ));
    assert!(matches!(
        MonsterMoveSplineLayout::from_if_statement(&with_else_if),
        Err(UnsupportedMonsterMoveSplineLayout)
    ));
    assert!(matches!(
        MonsterMoveSplineLayout::from_if_statement(&extra_branch_member),
        Err(UnsupportedMonsterMoveSplineLayout)
    ));
}
