use crate::file_utils::overwrite_if_not_same_contents;
use crate::path_utils::update_mask_doc_file;
use crate::rust_printer::{
    tbc_fields, vanilla_fields, wrath_fields, MemberType, UfType, UpdateMaskType,
};
use std::fmt::Write;
use std::fs::read_to_string;

fn print_specific_update_mask_doc(fields: &[MemberType], s: &mut String) {
    let update_types = [
        ("object", UpdateMaskType::Object),
        ("item", UpdateMaskType::Item),
        ("container", UpdateMaskType::Container),
        ("unit", UpdateMaskType::Unit),
        ("player", UpdateMaskType::Player),
        ("gameobject", UpdateMaskType::GameObject),
        ("dynamicobject", UpdateMaskType::DynamicObject),
        ("corpse", UpdateMaskType::Corpse),
    ];

    for (name, u) in update_types {
        writeln!(s, "Fields that all {name}s have:\n").unwrap();

        s.push_str("| Name | Offset | Size | Type |\n");
        s.push_str("|------|--------|------|------|\n");
        for field in fields {
            if field.object_ty() == u {
                let ty = match field.ty() {
                    UfType::Guid => "GUID",
                    UfType::Int => "INT",
                    UfType::Float => "FLOAT",
                    UfType::BytesWith(_, _, _, _) | UfType::Bytes => "BYTES",
                    UfType::TwoShort => "TWO_SHORT",
                    UfType::GuidEnumLookupArray { .. } | UfType::ArrayOfStruct { .. } => "CUSTOM",
                };

                writeln!(
                    s,
                    "|`{}_{}`| 0x{:04x?} | {} | {} |",
                    name.to_uppercase(),
                    field.name(),
                    field.offset(),
                    field.size(),
                    ty
                )
                .unwrap();

                if field.offset() == 0 && field.ty() == UfType::Guid {
                    writeln!(
                        s,
                        "|`{}_TYPE`| 0x{:04x?} | {} | INT |",
                        name.to_uppercase(),
                        2,
                        1,
                    )
                    .unwrap();
                }
            }
        }

        s.push_str("\n\n");
    }
}

pub(crate) fn print_update_mask_docs() {
    const LOOKUP_TABLE: &str = "## Lookup Table";
    let contents = read_to_string(update_mask_doc_file()).unwrap();

    let (s, _) = contents.split_once(LOOKUP_TABLE).unwrap();
    let mut s = s.to_string();
    s.push_str(LOOKUP_TABLE);
    s.push('\n');
    s.push_str("### Version 1.12\n\n");
    s.push_str("Taken from [vmangos](https://github.com/vmangos/core/blob/4b2a5173b0ca4917dfe91aa7b87d84232fd7203c/src/game/Objects/UpdateFields_1_12_1.cpp#L5) with some modifications.\n\n");

    print_specific_update_mask_doc(vanilla_fields::FIELDS, &mut s);

    s.push_str("### Version 2.4.3\n\n");
    s.push_str("Taken from [mangosone](https://github.com/mangosone/server/blob/f441fc27a6430e79753bceb545f62fef90a79832/src/game/Object/UpdateFields.h#L30) with some modifications.\n\n");

    print_specific_update_mask_doc(tbc_fields::FIELDS, &mut s);

    s.push_str("### Version 3.3.5\n\n");
    s.push_str("Taken from [ArcEmu](https://github.com/arcemu/arcemu/blob/1cb2b5248d050cb6fe413d7c42dd1817994b6366/src/world/Game/Entities/Update/UpdateFields.h#L26) with some modifications.\n\n");

    print_specific_update_mask_doc(wrath_fields::FIELDS, &mut s);

    overwrite_if_not_same_contents(&s, &update_mask_doc_file());
}
