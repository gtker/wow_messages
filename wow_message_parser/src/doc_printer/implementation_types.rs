use crate::file_utils::overwrite_autogenerate_if_insert_between_not_same;
use crate::parser::types::container::Container;
use crate::parser::types::objects::Objects;
use crate::parser::types::ty::Type;
use crate::parser::types::IntegerType;
use crate::path_utils::{doc_impl_login_path, doc_impl_world_path, doc_lang_spec_path};
use crate::rust_printer::writer::{Writer, WriterTable};
use std::collections::BTreeSet;
use std::path::Path;

fn print_implementation_type(table: &mut WriterTable<3>, ty: &Type) {
    let name = format!("`{}`", ty.str());

    match ty {
        Type::Integer(i) => {
            if let IntegerType::U48 = i {
                table.add_row([
                        name.as_ref(),
                        "`u32` followed by `u16`. Work around for having control flow from two different variables that simplifies implementations.",
                        format!("`unsigned int` followed by `unsigned short`").as_ref(),
                    ]);
                return;
            }

            let bits = i.size() as u32 * 8;
            let c_type = i.c_str();
            let (min, max) = (i.smallest_value(), i.largest_value());

            table.add_row([
                name,
                format!("Unsigned {bits} bit integer. Min value {min}, max value {max}."),
                format!("`{c_type}`"),
            ]);
        }
        Type::Bool(i) => {
            if let IntegerType::U48 = i {
                unreachable!("u48 for bool")
            }

            let bits = i.size() as u32;
            let c_type = i.c_str();

            table.add_row([
                name,
                format!("Unsigned {bits} bit integer. 0 means `false` and all other values mean `true`."),
                format!("`{c_type}`"),
            ]);
        }
        Type::Guid => {
            table.add_row([
                name,
                format!("Unsigned 8 bit integer. Can be replaced with a `u64`."),
                format!("`unsigned long long`"),
            ]);
        }
        Type::PackedGuid => table.add_row([
            name.as_ref(),
            "Guid sent in the \"packed\" format. See [PackedGuid](../types/packed-guid.md).",
            "-",
        ]),
        Type::NamedGuid => {
            table.add_row([
                name.as_ref(),
                "A `Guid` (`u64`) followed by a `CString` if the value of the `Guid` is not `0`.",
                "-",
            ]);
        }
        Type::DateTime => {
            table.add_row([
                name.as_ref(),
                "`u32` in a special format. See [DateTime](../types/datetime.md).",
                "`unsigned int`",
            ]);
        }
        Type::FloatingPoint => {
            table.add_row([name.as_ref(), "Floating point value of 4 bytes.", "`f32`"]);
        }
        Type::CString => {
            table.add_row([
                name.as_ref(),
                "UTF-8 string type that is terminated by a zero byte value.",
                "`char*`",
            ]);
        }
        Type::SizedCString => {
            table.add_row([
                name.as_ref(),
                "A `u32` field that determines the size of the string followed by a UTF-8 string type that is terminated by a zero byte value.",
                "`unsigned int` + `char*`",
            ]);
        }
        Type::String => {
            table.add_row([
                name.as_ref(),
                "UTF-8 string type of exactly length `len`.",
                "`unsigned char` + `char*`",
            ]);
        }
        Type::UpdateMask { .. } => {
            table.add_row([
                name.as_ref(),
                "Update values sent in a relatively complex format. See [UpdateMask](../types/update-mask.md).",
                "-",
            ]);
        }
        Type::MonsterMoveSplines => {
            table.add_row([
                name.as_ref(),
                "Array of positions. See [MonsterMoveSpline](../types/monster-move-spline).",
                "-",
            ]);
        }
        Type::AuraMask => {
            table.add_row([
                name.as_ref(),
                "Aura values sent using a mask. See [Masks](../types/aura-mask.md).",
                "-",
            ]);
        }
        Type::AchievementDoneArray => {
            table.add_row([
                name.as_ref(),
                "Array that terminates on a sentinel value. See [AchievementDoneArray](../types/achievement-done-array.md)",
                "-",
            ]);
        }
        Type::AchievementInProgressArray => {
            table.add_row([
                name.as_ref(),
                "Array that terminates on a sentinel value. See [AchievementInProgressArray](../types/achievement-done-array.md)",
                "-",
            ]);
        }
        Type::EnchantMask => {
            table.add_row([
                name.as_ref(),
                "Enchant values sent using a mask. See [Masks](../types/aura-mask.md).",
                "-",
            ]);
        }
        Type::InspectTalentGearMask => {
            table.add_row([
                name.as_ref(),
                "[InspectTalentGear](../docs/inspecttalentgear.md) values sent using a mask. See [Masks](../types/aura-mask.md).",
                "-",
            ]);
        }
        Type::VariableItemRandomProperty => {
            table.add_row([
                name.as_ref(),
                "A `u32` followed by another `u32` if the first value is not equal to `0`.",
                "-",
            ]);
        }
        Type::AddonArray => {
            table.add_row([
                name.as_ref(),
                "Array of [Addon](../docs/addon.md)s for TBC and Wrath that rely on externally knowing the amount of array members. See [AddonArray](../types/addon-array.md).",
                "-",
            ]);
        }

        Type::Population => {
            table.add_row([
                name.as_ref(),
                "`f32` with the special behavior that a value of `200` always means `GREEN_RECOMMENDED`, `400` always means `RED_FULL`, and `600` always means `BLUE_RECOMMENDED`.",
                "`float`",
            ]);
        }

        Type::IpAddress => {
            table.add_row([
                name.as_ref(),
                "Alias for **big endian** `u32`.",
                "`unsigned int`",
            ]);
        }
        Type::Level32 | Type::Seconds | Type::Milliseconds | Type::Gold => {
            table.add_row([name.as_ref(), "Alias for `u32`.", "`unsigned int`"]);
        }

        Type::Level => {
            table.add_row([name.as_ref(), "Alias for `u8`.", "`unsigned char`"]);
        }
        Type::Level16 => {
            table.add_row([name.as_ref(), "Alias for `u16`.", "`unsigned short`"]);
        }

        Type::Enum { .. } => {}
        Type::Flag { .. } => {}
        Type::Struct { .. } => {}
        Type::Array(_) => {}
    }
}

pub(crate) fn print_implementation_types(o: &Objects) {
    print_containers(o, |a| a.tags().has_login_version(), &doc_impl_login_path());
    print_containers(o, |a| a.tags().has_world_version(), &doc_impl_world_path());
    print_containers(o, |_| true, &doc_lang_spec_path());
}

fn print_containers(o: &Objects, filter: impl FnMut(&&Container) -> bool, path: &Path) {
    let mut s = Writer::new();
    s.newline();

    let mut types = BTreeSet::new();

    for e in o.all_containers().filter(filter) {
        for d in e.all_definitions() {
            types.insert(d.ty());
        }
    }
    let mut table = WriterTable::new(["Type", "Purpose", "C Name"]);

    for ty in types {
        print_implementation_type(&mut table, ty);
    }

    s.add_table(table);

    s.newline();

    overwrite_autogenerate_if_insert_between_not_same(
        path,
        "[comment]: # (AUTOGENERATED_FROM_HERE_NEXT_COMMENT)",
        "[comment]: # (AUTOGENERATED_UNTIL_HERE)",
        s.inner(),
    );
}
