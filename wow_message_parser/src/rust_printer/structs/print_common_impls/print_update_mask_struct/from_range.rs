use crate::parser::types::array::ArraySize;
use crate::parser::types::container::UpdateMaskMember;
use crate::parser::types::ty::Type;
use crate::rust_printer::writer::Writer;

pub(crate) fn create_from_range(s: &mut Writer, words: &[Vec<UpdateMaskMember>]) {
    s.open_curly("pub(crate) fn from_range<'a>(mut range: impl Iterator<Item = (&'a u16, &'a u32)>) -> Option<Self>");

    for (i, word) in words.iter().enumerate() {
        match word.as_slice() {
            [word] => {
                let member = &word.member;
                let name = member.name();

                s.wln(format!("// index {i}: {name}"));

                match member.ty() {
                    Type::Guid => {
                        s.wln(format!("let (_, &{name}_lower) = range.next()?;"));
                        s.wln(format!("let (_, &{name}_upper) = range.next()?;"));
                        s.wln(format!(
                            "let {name} = crate::Guid::from_u32s({name}_lower, {name}_upper);"
                        ));
                    }

                    Type::DateTime | Type::Seconds | Type::Milliseconds => {
                        s.wln(format!("let (_, &{name}) = range.next()?;"));
                    }
                    Type::Integer(i) => {
                        assert_eq!(i.size(), 4);

                        s.wln(format!("let (_, &{name}) = range.next()?;"));
                    }
                    Type::Bool(i) => {
                        assert_eq!(i.size(), 4);
                    }

                    Type::FloatingPoint => {
                        s.wln(format!("let (_, &{name}) = range.next()?;"));
                        s.wln(format!(
                            "let {name} = f32::from_le_bytes({name}.to_le_bytes());"
                        ));
                    }
                    Type::Enum { .. } | Type::Flag { .. } => {
                        s.wln(format!("let (_, &{name}) = range.next()?;"));
                        s.wln(format!("let {name} = {name}.try_into().ok()?;"));
                    }

                    Type::Array(array) => match array.size() {
                        ArraySize::Fixed(size) => {
                            assert!(array.ty().sizes().is_constant().is_some());

                            s.wln(format!("let mut {name} = [0; {size}];"));

                            let inner_size = array.ty().sizes().maximum();
                            match inner_size {
                                4 => {
                                    for i in 0..size {
                                        s.wln(format!("let (_, &{name}_temp) = range.next()?;"));
                                        s.wln(format!("{name}[{i}] = {name}_temp;"));
                                    }
                                }
                                2 => {
                                    for i in (0..size).step_by(2) {
                                        s.wln(format!("let (_, &{name}_temp) = range.next()?;"));
                                        s.wln(format!("let ({name}_temp_a, {name}_temp_b) = crate::util::u32_to_u16s({name}_temp);"));
                                        s.wln(format!("{name}[{i}] = {name}_temp_a;"));
                                        let i = i + 1;
                                        s.wln(format!("{name}[{i}] = {name}_temp_b;"));
                                    }
                                }
                                _ => unimplemented!(),
                            }
                        }
                        ArraySize::Variable(_) | ArraySize::Endless => {
                            unreachable!("variable and endless in update mask struct")
                        }
                    },

                    Type::Gold | Type::Level | Type::Level16 | Type::Level32 | Type::IpAddress => {
                        unimplemented!()
                    }

                    Type::Population
                    | Type::AddonArray
                    | Type::String
                    | Type::CString
                    | Type::SizedCString
                    | Type::VariableItemRandomProperty
                    | Type::Struct { .. }
                    | Type::UpdateMask { .. }
                    | Type::MonsterMoveSplines
                    | Type::AuraMask
                    | Type::AchievementDoneArray
                    | Type::AchievementInProgressArray
                    | Type::EnchantMask
                    | Type::InspectTalentGearMask
                    | Type::PackedGuid
                    | Type::NamedGuid => unreachable!(),
                }
            }
            [first, second] => {
                let first_member = &first.member;
                let first_name = first_member.name();

                let second_member = &second.member;
                let second_name = second_member.name();

                s.wln(format!("// index {i}: {first_name} and {second_name}"));

                s.wln(format!("let (_, &{first_name}) = range.next()?;"));
                s.wln(format!(
                    "let ({second_name}, {first_name}) = crate::util::u32_to_u16s({first_name});"
                ));

                fn handle_short(s: &mut Writer, member: &UpdateMaskMember) {
                    let name = member.member.name();

                    match member.member.ty() {
                        Type::Integer(i) | Type::Bool(i) => {
                            assert_eq!(i.size(), 2);
                        }

                        Type::Enum { .. } | Type::Flag { .. } => {
                            s.wln(format!("let {name} = {name}.try_into().ok()?;"));
                        }

                        Type::Array(_) => unimplemented!(),

                        Type::Guid
                        | Type::DateTime
                        | Type::Seconds
                        | Type::Milliseconds
                        | Type::FloatingPoint
                        | Type::Gold
                        | Type::Level
                        | Type::Level16
                        | Type::Level32
                        | Type::IpAddress => {
                            unimplemented!()
                        }

                        Type::Population
                        | Type::AddonArray
                        | Type::String
                        | Type::CString
                        | Type::SizedCString
                        | Type::VariableItemRandomProperty
                        | Type::Struct { .. }
                        | Type::UpdateMask { .. }
                        | Type::MonsterMoveSplines
                        | Type::AuraMask
                        | Type::AchievementDoneArray
                        | Type::AchievementInProgressArray
                        | Type::EnchantMask
                        | Type::InspectTalentGearMask
                        | Type::PackedGuid
                        | Type::NamedGuid => unreachable!(),
                    }
                }

                handle_short(s, first);
                handle_short(s, second);
            }
            [] => continue,
            _ => unimplemented!(),
        }

        s.newline();
    }

    s.body_closing_with(
        "Some(Self",
        |s| {
            for word in words {
                for d in word {
                    if d.member.value().is_some() {
                        continue;
                    }

                    s.wln(format!("{},", d.member.name()));
                }
            }
        },
        ")",
    );

    s.closing_curly();
}
