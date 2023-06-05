use crate::file_utils::overwrite_if_not_same_contents;
use crate::path_utils::update_mask_doc_file;
use crate::rust_printer::{
    tbc_fields, vanilla_fields, wrath_fields, MemberType, UfType, UpdateMaskType, Writer,
};
use std::fs::read_to_string;

fn print_specific_update_mask_doc(fields: &[MemberType], s: &mut Writer) {
    for &u in UpdateMaskType::all() {
        let name = u.as_str();
        s.wln(format!("Fields that all {name}s have:"));
        s.newline();

        s.wln("| Name | Offset | Size | Type |");
        s.wln("|------|--------|------|------|");
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

                s.wln(format!(
                    "|`{}_{}`| 0x{:04x?} | {} | {} |",
                    name.to_uppercase(),
                    field.name(),
                    field.offset(),
                    field.size(),
                    ty
                ));

                if field.offset() == 0 && field.ty() == UfType::Guid {
                    s.wln(format!(
                        "|`{}_TYPE`| 0x{:04x?} | {} | INT |",
                        name.to_uppercase(),
                        2,
                        1,
                    ));
                }
            }
        }

        s.newline();
        s.newline();
    }
}

pub(crate) fn print_update_mask_docs() {
    const LOOKUP_TABLE: &str = "## Lookup Table";
    let contents = read_to_string(update_mask_doc_file()).unwrap();

    let (previous, _) = contents.split_once(LOOKUP_TABLE).unwrap();

    let mut s = Writer::start_with(previous.to_string());

    s.wln(LOOKUP_TABLE);
    s.wln("### Version 1.12");
    s.newline();

    s.wln("Taken from [vmangos](https://github.com/vmangos/core/blob/4b2a5173b0ca4917dfe91aa7b87d84232fd7203c/src/game/Objects/UpdateFields_1_12_1.cpp#L5) with some modifications.");
    s.newline();

    print_specific_update_mask_doc(vanilla_fields::FIELDS, &mut s);

    s.wln("### Version 2.4.3");
    s.newline();
    s.wln("Taken from [mangosone](https://github.com/mangosone/server/blob/f441fc27a6430e79753bceb545f62fef90a79832/src/game/Object/UpdateFields.h#L30) with some modifications.");
    s.newline();

    print_specific_update_mask_doc(tbc_fields::FIELDS, &mut s);

    s.wln("### Version 3.3.5");
    s.newline();

    s.wln("Taken from [ArcEmu](https://github.com/arcemu/arcemu/blob/1cb2b5248d050cb6fe413d7c42dd1817994b6366/src/world/Game/Entities/Update/UpdateFields.h#L26) with some modifications.");
    s.newline();

    print_specific_update_mask_doc(wrath_fields::FIELDS, &mut s);

    overwrite_if_not_same_contents(s.inner(), &update_mask_doc_file());
}
