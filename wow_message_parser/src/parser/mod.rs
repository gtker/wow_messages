use crate::parser::types::parsed::parsed_tags::ParsedTags;
use std::fs::read_to_string;
use std::path::Path;

use pest;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

use types::definer::DefinerField;

use crate::file_info::FileInfo;
use crate::parser::types::definer::DefinerValue;
use crate::parser::types::parsed::parsed_array::ParsedArray;
use crate::parser::types::parsed::parsed_if_statement::ParsedIfStatement;
use crate::parser::types::parsed::parsed_optional::ParsedOptionalStatement;
use crate::parser::types::parsed::parsed_struct_member::{
    ParsedStructMember, ParsedStructMemberDefinition,
};
use crate::parser::types::parsed::parsed_ty::ParsedType;
use crate::parser::types::IntegerType;
use crate::parser::utility::parse_value;
use crate::rust_printer::DefinerType;
use crate::{error_printer, ParsedObjects, UNIMPLEMENTED};
use types::container::ContainerType;
use types::if_statement::{Condition, Conditional};
use types::parsed::parsed_container::ParsedContainer;
use types::parsed::parsed_definer::ParsedDefiner;
use types::parsed::parsed_test_case::{ParsedTestCase, ParsedTestCaseMember, ParsedTestValue};

pub mod stats;
pub mod types;
pub mod utility;

#[derive(Parser)]
#[grammar = "auth.pest"]
pub(crate) struct AuthParser;

#[derive(Debug)]
pub(crate) struct Commands {
    tags: ParsedTags,
}

impl Commands {
    pub(crate) fn new(tags: ParsedTags) -> Self {
        Self { tags }
    }

    pub(crate) fn empty() -> Self {
        Self {
            tags: ParsedTags::new(),
        }
    }

    pub(crate) fn tags(&self) -> &ParsedTags {
        &self.tags
    }
}

pub(crate) fn parse_commands(t: Pair<Rule>) -> Commands {
    let t = t.into_inner();
    let mut tags = ParsedTags::new();

    for command in t {
        let mut command = command.into_inner();
        let command_ident = command.next().unwrap();
        match command_ident.as_str() {
            "tag_all" => {
                let key = command.next().unwrap();
                let value = command.next().unwrap();
                tags.insert(key.as_str(), value.as_str());
            }
            _ => unreachable!("invalid command"),
        }
    }

    Commands::new(tags)
}

pub(crate) fn parse_contents(contents: &str, filename: &Path) -> ParsedObjects {
    let file = AuthParser::parse(Rule::file, contents);
    let file = match file {
        Ok(mut f) => f.next().unwrap(),
        Err(e) => panic!("error parsing {}: {}", filename.to_str().unwrap(), e),
    };
    assert_eq!(file.as_rule(), Rule::file, "first scope was not file");

    let mut file = file.into_inner();

    let commands = file.find(|a| a.as_rule() == Rule::commands);
    let commands = match commands {
        Some(c) => parse_commands(c),
        None => Commands::empty(),
    };

    let mut statements = file
        .find(|a| a.as_rule() == Rule::statements)
        .expect("unable to find statements")
        .into_inner();

    parse_statements(&mut statements, commands.tags(), filename)
}

pub(crate) fn parse_file(filename: &Path) -> ParsedObjects {
    let contents = read_to_string(filename).expect("unable to read file");
    parse_contents(&contents, filename)
}

fn parse_statements(statements: &mut Pairs<Rule>, tags: &ParsedTags, path: &Path) -> ParsedObjects {
    let mut enums = Vec::new();
    let mut flags = Vec::new();
    let mut structs = Vec::new();
    let mut messages = Vec::new();
    let mut tests = Vec::new();
    let mut descriptive_comments = Vec::new();

    for statement in statements {
        let start_pos = statement.as_span().start_pos().line_col();
        let end_pos = statement.as_span().end_pos().line_col().0;
        let file_info = FileInfo::new(path.to_owned(), start_pos.0, end_pos);
        match statement.as_rule() {
            Rule::descriptive_comment => {
                let mut statement = statement.into_inner();
                let comment = statement.next().unwrap().as_str().trim();
                descriptive_comments.push(comment);
            }
            Rule::definer => {
                let mut statement = statement.into_inner();
                let keyword = statement.next().unwrap().as_str();

                match keyword {
                    "enum" => enums.push(parse_enum(
                        &mut statement,
                        tags,
                        file_info,
                        DefinerType::Enum,
                        &descriptive_comments,
                    )),
                    "flag" => flags.push(parse_enum(
                        &mut statement,
                        tags,
                        file_info,
                        DefinerType::Flag,
                        &descriptive_comments,
                    )),
                    _ => unreachable!("invalid keyword for definer"),
                }

                descriptive_comments.clear();
            }
            Rule::container => {
                let mut statement = statement.into_inner();
                let keyword = statement.next().unwrap().as_str();

                let (ty, objects) = match keyword {
                    "struct" => (ContainerType::Struct, &mut structs),
                    "clogin" => (ContainerType::CLogin(0), &mut messages),
                    "slogin" => (ContainerType::SLogin(0), &mut messages),
                    "cmsg" => (ContainerType::CMsg(0), &mut messages),
                    "smsg" => (ContainerType::SMsg(0), &mut messages),
                    "msg" => (ContainerType::Msg(0), &mut messages),
                    keyword => unreachable!("invalid keyword for container: '{}'", keyword),
                };
                parse_struct(
                    &mut statement,
                    tags,
                    ty,
                    file_info,
                    objects,
                    &descriptive_comments,
                );

                descriptive_comments.clear();
            }
            Rule::test => {
                let mut statement = statement.into_inner();
                tests.push(parse_test(
                    &mut statement,
                    tags,
                    file_info,
                    &descriptive_comments,
                ));

                descriptive_comments.clear();
            }
            _ => unreachable!("statements should only have definers or containers"),
        }
    }

    ParsedObjects::new(enums, flags, structs, messages, tests)
}

fn parse_test_values(m: Pair<Rule>, test_members: &mut Vec<ParsedTestCaseMember>) {
    let mut m = m.into_inner();
    let member_name = m.next().unwrap().as_str();
    let member_value = m.next().unwrap();

    let member_value = match member_value.as_rule() {
        Rule::sub_object => {
            let mut v = Vec::new();

            for m_inner in member_value.into_inner() {
                parse_test_values(m_inner, &mut v);
            }

            ParsedTestValue::Multiple(v)
        }
        Rule::array_of_sub_objects => {
            let mut multiples = Vec::new();
            let member_value = member_value.into_inner();

            for sub_object in member_value {
                let mut v = Vec::new();

                for m_inner in sub_object.into_inner() {
                    parse_test_values(m_inner, &mut v);
                }

                multiples.push(v);
            }

            ParsedTestValue::ArrayOfMultiple(multiples)
        }
        _ => ParsedTestValue::Single(member_value.as_str().to_owned()),
    };

    let mut extra_kvs = m.find(|a| a.as_rule() == Rule::object_key_values);
    let kvs = parse_object_key_values(&mut extra_kvs, &ParsedTags::new());

    test_members.push(ParsedTestCaseMember::new(
        member_name,
        member_value,
        kvs.into_member_tags(),
    ));
}

fn parse_test(
    t: &mut Pairs<Rule>,
    tags: &ParsedTags,
    file_info: FileInfo,
    descriptive_comments: &[&str],
) -> ParsedTestCase {
    let name = t.next().unwrap().as_str();
    let members = t.next().unwrap().into_inner();

    let mut test_members = Vec::new();
    for m in members {
        parse_test_values(m, &mut test_members);
    }

    let mut raw_bytes = Vec::new();
    let array_values = t.next().unwrap().into_inner();
    for r in array_values {
        raw_bytes.push(parse_value(r.as_str()).unwrap() as u8);
    }

    let mut extra_kvs = t.find(|a| a.as_rule() == Rule::object_key_values);
    let mut kvs = parse_object_key_values(&mut extra_kvs, tags);
    kvs.add_descriptive_comments(descriptive_comments);

    ParsedTestCase::new(
        name,
        test_members,
        raw_bytes,
        kvs.into_tags(name, &file_info, false),
        file_info,
    )
}

fn parse_struct(
    t: &mut Pairs<Rule>,
    tags: &ParsedTags,
    container_type: ContainerType,
    file_info: FileInfo,
    objects: &mut Vec<ParsedContainer>,
    descriptive_comments: &[&str],
) {
    let identifier = t.next().unwrap().as_str();

    let opcode = t.clone().find(|a| a.as_rule() == Rule::value);
    let container_type = if let Some(opcode) = opcode {
        if let Some(v) = parse_value(opcode.as_str()) {
            match container_type {
                ContainerType::Struct => unreachable!("struct has opcode"),
                ContainerType::CLogin(_) => ContainerType::CLogin(v as u16),
                ContainerType::SLogin(_) => ContainerType::SLogin(v as u16),
                ContainerType::Msg(_) => ContainerType::Msg(v as u16),
                ContainerType::CMsg(_) => ContainerType::CMsg(v as u16),
                ContainerType::SMsg(_) => ContainerType::SMsg(v as u16),
            }
        } else {
            match container_type {
                ContainerType::Struct => ContainerType::Struct,
                _ => unreachable!("non-struct missing opcode"),
            }
        }
    } else {
        match container_type {
            ContainerType::Struct => ContainerType::Struct,
            _ => unreachable!("non-struct missing opcode"),
        }
    };

    let container_members = t
        .find(|a| a.as_rule() == Rule::container_members)
        .unwrap_or_else(|| unreachable!("no members for struct {}", identifier))
        .into_inner();

    let mut members = Vec::new();

    for member in container_members {
        let unimplemented = member
            .clone()
            .into_inner()
            .any(|a| a.as_rule() == Rule::unimplemented);

        if unimplemented {
            let mut extra_kvs = t.find(|a| a.as_rule() == Rule::object_key_values);
            let mut kvs = parse_object_key_values(&mut extra_kvs, tags);
            kvs.insert(UNIMPLEMENTED, "true");
            let v = vec![unimplemented_member()];

            apply_tags(identifier, v, kvs, file_info, container_type, objects);
            return;
        }

        members.push(parse_struct_member(member, identifier, &file_info));
    }

    let mut extra_kvs = t.find(|a| a.as_rule() == Rule::object_key_values);
    let mut kvs = parse_object_key_values(&mut extra_kvs, tags);
    kvs.add_descriptive_comments(descriptive_comments);

    apply_tags(identifier, members, kvs, file_info, container_type, objects);
}

fn apply_tags(
    identifier: &str,
    members: Vec<ParsedStructMember>,
    tags: ParsedTags,
    file_info: FileInfo,
    container_type: ContainerType,
    objects: &mut Vec<ParsedContainer>,
) {
    if tags.paste_versions().next().is_none() {
        objects.push(ParsedContainer::new(
            identifier,
            members,
            tags.into_tags(identifier, &file_info, false),
            container_type,
            file_info,
        ));
    } else {
        for v in tags.paste_versions() {
            let mut tags = tags.clone();
            tags.push_version(v);

            objects.push(ParsedContainer::new(
                identifier,
                members.clone(),
                tags.into_tags(identifier, &file_info, false),
                container_type,
                file_info.clone(),
            ));
        }
    }
}

fn unimplemented_member() -> ParsedStructMember {
    ParsedStructMember::Definition(Box::new(ParsedStructMemberDefinition::new(
        UNIMPLEMENTED,
        ParsedType::Array(ParsedArray::new_unimplemented()),
        None,
        ParsedTags::new().into_member_tags(),
    )))
}

fn parse_struct_member(ts: Pair<Rule>, ty_name: &str, file_info: &FileInfo) -> ParsedStructMember {
    assert_eq!(ts.as_rule(), Rule::container_member);

    let mut ts = ts.into_inner();

    let mut descriptive_comments = Vec::new();

    let mut t = ts.next().unwrap();
    while matches!(t.as_rule(), Rule::descriptive_comment) {
        descriptive_comments.push(t.into_inner().as_str().trim());
        t = ts.next().unwrap();
    }

    match t.as_rule() {
        Rule::basic_container_item | Rule::complex_container_item => {
            let mut t = t.into_inner();
            let mut f = t
                .find(|a| a.as_rule() == Rule::container_identifier_type_and_value)
                .unwrap()
                .into_inner();

            let mut identifier_and_value = f
                .find(|a| a.as_rule() == Rule::container_identifier_and_type)
                .unwrap()
                .into_inner();

            let value = f.find(|a| a.as_rule() == Rule::value);
            let value = if let Some(value) = value {
                let value = value.as_str().into();
                Some(value)
            } else {
                None
            };

            let complex_key_value_pairs = t.filter(|a| a.as_rule() == Rule::complex_key_value_pair);
            let mut kvs = ParsedTags::new();
            for kv in complex_key_value_pairs {
                let mut kv = kv.into_inner();
                let identifier = kv.find(|a| a.as_rule() == Rule::identifier).unwrap();
                let text = kv.find(|a| a.as_rule() == Rule::text).unwrap();
                kvs.insert(identifier.as_str(), text.as_str());
            }
            kvs.add_descriptive_comments(&descriptive_comments);

            let mut container_type = identifier_and_value
                .find(|a| a.as_rule() == Rule::container_type)
                .unwrap()
                .into_inner();
            let next_possible = container_type.next().unwrap();
            let (container_type, upcasted_type) = match next_possible.as_rule() {
                Rule::upcasted_type => (
                    container_type.next().unwrap().as_str(),
                    Some(next_possible.into_inner().next().unwrap().as_str()),
                ),
                _ => (next_possible.as_str(), None),
            };

            let identifier = identifier_and_value
                .find(|a| a.as_rule() == Rule::identifier)
                .unwrap();

            let container_type = if let Some(ty) = upcasted_type {
                ParsedType::with_upcast(container_type, ty, ty_name, identifier.as_str(), file_info)
            } else {
                let compressed = kvs.compressed();
                ParsedType::from_str(container_type, compressed)
            };

            let s = ParsedStructMemberDefinition::new(
                identifier.as_str(),
                container_type,
                value,
                kvs.into_member_tags(),
            );

            ParsedStructMember::Definition(Box::new(s))
        }
        Rule::if_statement => {
            let mut f = t.into_inner();

            let (conditions, members) = parse_if_statement(&mut f, ty_name, file_info);

            let else_if_statement = f.clone().filter(|a| a.as_rule() == Rule::else_if_statement);
            let mut else_ifs = Vec::new();
            for statement in else_if_statement {
                let (conditions, members) =
                    parse_if_statement(&mut statement.into_inner(), ty_name, file_info);
                else_ifs.push(ParsedIfStatement::new(
                    Conditional::new(&conditions, ty_name, file_info),
                    members,
                    vec![],
                    vec![],
                ));
            }

            let else_statement = f.find(|a| a.as_rule() == Rule::else_statement);
            let else_statement_members = if let Some(else_statement) = else_statement {
                let else_statement = else_statement.into_inner();

                let mut v = Vec::new();

                for member in else_statement {
                    v.push(parse_struct_member(member, ty_name, file_info));
                }

                v
            } else {
                Vec::new()
            };

            let conditional = Conditional::new(&conditions, ty_name, file_info);
            ParsedStructMember::IfStatement(ParsedIfStatement::new(
                conditional,
                members,
                else_ifs,
                else_statement_members,
            ))
        }
        Rule::optional_statement => {
            let mut t = t.into_inner();
            let name = t.next().unwrap().as_str();
            let members = t.next().unwrap().into_inner();

            let mut v = Vec::new();

            for member in members {
                v.push(parse_struct_member(member, ty_name, file_info));
            }

            ParsedStructMember::OptionalStatement(ParsedOptionalStatement::new(name, v))
        }
        p => {
            unreachable!("invalid member for struct: {:#?}", p)
        }
    }
}

fn parse_if_statement(
    f: &mut Pairs<Rule>,
    ty_name: &str,
    file_info: &FileInfo,
) -> (Vec<Condition>, Vec<ParsedStructMember>) {
    let conditionals: Vec<Pair<Rule>> = f
        .clone()
        .filter(|a| a.as_rule() == Rule::if_statement_conditional)
        .collect();
    let mut conditions = Vec::new();
    for conditional in conditionals {
        let mut conditional = conditional.into_inner();
        let identifier = conditional
            .find(|a| a.as_rule() == Rule::identifier)
            .unwrap()
            .as_str();
        let operator = conditional
            .find(|a| a.as_rule() == Rule::if_statement_operator)
            .unwrap();
        let if_statement_value = conditional
            .find(|a| a.as_rule() == Rule::if_statement_value)
            .unwrap()
            .as_str();

        let condition = Condition::new(identifier, if_statement_value, operator.as_str().into());
        conditions.push(condition);
    }

    let container_members = f.find(|a| a.as_rule() == Rule::container_members);
    let members = if let Some(container_members) = container_members {
        let container_members = container_members.into_inner();

        let mut members = Vec::new();
        for m in container_members {
            members.push(parse_struct_member(m, ty_name, file_info));
        }

        members
    } else {
        Vec::new()
    };

    (conditions, members)
}

pub(crate) fn parse_enum(
    t: &mut Pairs<Rule>,
    tags: &ParsedTags,
    file_info: FileInfo,
    definer_ty: DefinerType,
    descriptive_comments: &[&str],
) -> ParsedDefiner {
    let ident = t.next().unwrap();
    assert_eq!(ident.as_rule(), Rule::identifier);

    let basic_type = t.next().unwrap();
    assert_eq!(basic_type.as_rule(), Rule::basic_type);
    let definer_members = t.next().unwrap().into_inner();

    let mut fields = Vec::new();

    for item in definer_members {
        let item_rule = item.as_rule();
        assert!(item_rule == Rule::basic_definer_item || item_rule == Rule::complex_definer_item);

        let mut item = item.into_inner();

        let mut first = item.next().unwrap();

        let mut comments = Vec::new();

        while matches!(first.as_rule(), Rule::descriptive_comment) {
            comments.push(first.into_inner().as_str().trim());
            first = item.next().unwrap();
        }

        let mut identifier_and_value = first.into_inner();

        let identifier = identifier_and_value.next().unwrap();
        let value = identifier_and_value.next().unwrap();

        let mut kvs = ParsedTags::new();
        while let Some(complex_key_value_pair) = item.next() {
            let mut complex_key_value_pair = complex_key_value_pair.into_inner();
            let identifier = complex_key_value_pair.next().unwrap();
            let text = complex_key_value_pair.next().unwrap();
            kvs.insert(identifier.as_str(), text.as_str());
        }
        kvs.add_descriptive_comments(&comments);

        fields.push(DefinerField::new(
            identifier.as_str(),
            DefinerValue::from_str(
                value.as_str(),
                ident.as_str(),
                identifier.as_str(),
                &file_info,
            ),
            kvs.into_member_tags(),
        ));
    }

    let mut extra_key_values = t.next();
    let mut extras = parse_object_key_values(&mut extra_key_values, tags);
    extras.append(tags.clone());
    extras.add_descriptive_comments(descriptive_comments);

    let basic_type = IntegerType::from_str(basic_type.as_str(), ident.as_str(), &file_info);

    if definer_ty == DefinerType::Flag && basic_type.is_signed() {
        error_printer::flag_with_signed_type(ident.as_str(), &file_info, basic_type);
    }

    ParsedDefiner::new(
        ident.as_str(),
        definer_ty,
        fields,
        basic_type,
        extras.into_tags(ident.as_str(), &file_info, true),
        file_info,
    )
}

fn parse_object_key_values(t: &mut Option<Pair<Rule>>, tags: &ParsedTags) -> ParsedTags {
    match t {
        None => tags.clone(),
        Some(s) => {
            let s = s.clone().into_inner();
            let mut v = ParsedTags::new();
            for kv in s {
                let mut kv = kv.into_inner();

                let key = kv.next().unwrap().as_str();
                let value = kv.next().unwrap().as_str();

                v.insert(key, value);
            }
            v.append(tags.clone());
            v
        }
    }
}
