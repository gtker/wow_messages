use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::ty::Type;
use crate::rust_printer::structs::test_case_string;
use crate::rust_printer::structs::test_case_string::wlna;
use crate::rust_printer::Writer;

pub(crate) fn print_bytes(s: &mut Writer, e: &Container) {
    s.newline();
    s.wln("// Size/Opcode");

    if matches!(
        e.container_type(),
        ContainerType::SMsg(_) | ContainerType::CMsg(_)
    ) {
        let additional_size = match e.container_type() {
            ContainerType::CMsg(_) => 6,
            ContainerType::SMsg(_) => 4,
            _ => 0,
        };

        if let Some(size) = e.sizes().is_constant() {
            let size = size + additional_size;
            s.wln(format!("let [a, b] = {size}_u16.to_be_bytes();"));
        } else {
            s.wln(format!(
                "let [a, b] = (u16::try_from(self.size() + {additional_size}).unwrap()).to_be_bytes();"
            ));
        }

        test_case_string::wln(s, "    {a:#04X}, {b:#04X}, /* size */");
        let opcode = e.opcode();
        match e.container_type() {
            ContainerType::CMsg(_) => {
                s.wln(format!("let [a, b] = {opcode}_u16.to_le_bytes();"));
                test_case_string::wln(s, "    {a:#04X}, {b:#04X}, /* opcode */");
            }
            ContainerType::SMsg(_) => {
                s.wln(format!("let [a, b, c, d] = {opcode}_u32.to_le_bytes();"));
                test_case_string::wln(
                    s,
                    "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */",
                );
            }
            _ => {}
        }
    }

    print_bytes_member(s, e);

    s.newline();
}

fn print_bytes_member(s: &mut Writer, e: &Container) {
    s.wln("// Bytes");
    s.wln("let mut bytes: Vec<u8> = Vec::new();");
    s.wln("self.write_into_vec(&mut bytes).unwrap();");
    s.wln("let mut bytes = bytes.into_iter();");

    s.newline();

    let mut all_printed = true;

    if matches!(
        e.container_type(),
        ContainerType::CLogin(_) | ContainerType::SLogin(_)
    ) {
        wlna(s, "    {:#04X}, /* opcode */ ", "bytes.next().unwrap()");
    }

    for m in e.members() {
        if !print_bytes_struct_member(s, m) {
            all_printed = false;
            break;
        }
    }

    if !all_printed {
        s.body("for (i, b) in bytes.enumerate()", |s| {
            s.body("if i == 0", |s| {
                s.wln("write!(s, \"    \").unwrap();");
            });
            s.wln("write!(s, \"{b:#04X}, \").unwrap();");
        });
    }

    s.newline();
}

fn print_bytes_struct_member(s: &mut Writer, m: &StructMember) -> bool {
    match m {
        StructMember::Definition(d) => {
            if !print_bytes_struct_member_definition(s, d) {
                return false;
            }
        }
        StructMember::IfStatement(_) => {}
        StructMember::OptionalStatement(_) => {}
    }

    false
}

fn print_bytes_struct_member_definition(s: &mut Writer, d: &StructMemberDefinition) -> bool {
    let name = d.name();

    match d.ty() {
        Type::Guid
        | Type::Bool(_)
        | Type::Integer(_)
        | Type::Level
        | Type::Level16
        | Type::IpAddress
        | Type::FloatingPoint
        | Type::DateTime
        | Type::Seconds
        | Type::Milliseconds
        | Type::Level32
        | Type::Gold => {
            let size = d.ty().sizes().is_constant().unwrap();
            s.wln(format!(
                "crate::util::write_bytes(&mut s, &mut bytes, {size}, \"{name}\");"
            ));

            return true;
        }

        Type::Enum { e, upcast } | Type::Flag { e, upcast } => {
            let size = if let Some(upcast) = upcast {
                upcast.size()
            } else {
                e.ty().size()
            };

            s.wln(format!(
                "crate::util::write_bytes(&mut s, &mut bytes, {size}, \"{name}\");"
            ));

            return true;
        }

        Type::PackedGuid => {}

        Type::CString => {}
        Type::SizedCString => {}
        Type::String => {}
        Type::Array(_) => {}
        Type::Struct { .. } => {}

        Type::UpdateMask { .. }
        | Type::MonsterMoveSplines
        | Type::AuraMask
        | Type::AchievementDoneArray
        | Type::AchievementInProgressArray
        | Type::EnchantMask
        | Type::InspectTalentGearMask
        | Type::VariableItemRandomProperty
        | Type::AddonArray
        | Type::NamedGuid => {
            s.wln(format!(
                "panic!(\"unsupported type {} for variable '{name}'\");",
                d.ty().rust_str()
            ));
        }
    }

    false
}
