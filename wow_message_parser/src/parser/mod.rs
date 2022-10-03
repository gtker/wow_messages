use std::fs::read_to_string;
use std::path::Path;

use pest;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

use types::definer::DefinerField;
use types::tags::{Tag, Tags};

use crate::file_info::FileInfo;
use crate::parser::types::parsed::parsed_if_statement::ParsedIfStatement;
use crate::parser::types::parsed::parsed_optional::ParsedOptionalStatement;
use crate::parser::types::parsed::parsed_struct_member::{
    ParsedStructMember, ParsedStructMemberDefinition,
};
use crate::parser::types::Array;
use crate::parser::utility::parse_value;
use crate::path_utils::path_to_fileinfo;
use crate::rust_printer::DefinerType;
use crate::{ParsedObjects, ENUM_SELF_VALUE_FIELD};
use types::container::ContainerType;
use types::definer::SelfValueDefinerField;
use types::if_statement::{Condition, Conditional};
use types::parsed::parsed_container::ParsedContainer;
use types::parsed::parsed_definer::ParsedDefiner;
use types::parsed::parsed_test_case::{ParsedTestCase, ParsedTestCaseMember, ParsedTestValue};
use types::ty::Type;

pub mod stats;
pub mod types;
pub mod utility;

#[derive(Parser)]
#[grammar = "auth.pest"]
pub(crate) struct AuthParser;

#[derive(Debug)]
pub(crate) struct Commands {
    tags: Tags,
}

impl Commands {
    pub(crate) fn new(tags: Tags) -> Self {
        Self { tags }
    }

    pub(crate) fn empty() -> Self {
        Self { tags: Tags::new() }
    }

    pub(crate) fn tags(&self) -> &Tags {
        &self.tags
    }
}

pub(crate) fn parse_commands(t: Pair<Rule>) -> Commands {
    let t = t.into_inner();
    let mut tags = Tags::new();

    for command in t {
        let mut command = command.into_inner();
        let command_ident = command.next().unwrap();
        match command_ident.as_str() {
            "tag_all" => {
                let key = command.next().unwrap();
                let value = command.next().unwrap();
                tags.push(Tag::new(key.as_str(), value.as_str()));
            }
            _ => panic!("invalid command"),
        }
    }

    Commands::new(tags)
}

pub(crate) fn parse_file(filename: &Path) -> ParsedObjects {
    let contents = read_to_string(filename).expect("unable to read file");
    let file = AuthParser::parse(Rule::file, &contents);
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
        .into_iter()
        .find(|a| a.as_rule() == Rule::statements)
        .expect("unable to find statements")
        .into_inner();

    let filename = path_to_fileinfo(filename);

    parse_statements(&mut statements, commands.tags(), &filename)
}

fn parse_statements(statements: &mut Pairs<Rule>, tags: &Tags, filename: &str) -> ParsedObjects {
    let mut enums = Vec::new();
    let mut flags = Vec::new();
    let mut structs = Vec::new();
    let mut messages = Vec::new();
    let mut tests = Vec::new();

    for statement in statements {
        let start_pos = statement.as_span().start_pos().line_col();
        let file_info = FileInfo::new(filename, start_pos);
        match statement.as_rule() {
            Rule::definer => {
                let mut statement = statement.into_inner();
                let keyword = statement.next().unwrap().as_str();

                match keyword {
                    "enum" => enums.push(parse_enum(
                        &mut statement,
                        tags,
                        file_info,
                        DefinerType::Enum,
                    )),
                    "flag" => flags.push(parse_enum(
                        &mut statement,
                        tags,
                        file_info,
                        DefinerType::Flag,
                    )),
                    _ => panic!("invalid keyword for definer"),
                }
            }
            Rule::container => {
                let mut statement = statement.into_inner();
                let keyword = statement.next().unwrap().as_str();

                match keyword {
                    "struct" => structs.push(parse_struct(
                        &mut statement,
                        tags,
                        ContainerType::Struct,
                        file_info,
                    )),
                    "clogin" => {
                        messages.push(parse_struct(
                            &mut statement,
                            tags,
                            ContainerType::CLogin(0),
                            file_info,
                        ));
                    }
                    "slogin" => {
                        let s =
                            parse_struct(&mut statement, tags, ContainerType::SLogin(0), file_info);
                        messages.push(s);
                    }
                    "cmsg" => {
                        messages.push(parse_struct(
                            &mut statement,
                            tags,
                            ContainerType::CMsg(0),
                            file_info,
                        ));
                    }
                    "smsg" => {
                        messages.push(parse_struct(
                            &mut statement,
                            tags,
                            ContainerType::SMsg(0),
                            file_info,
                        ));
                    }
                    "msg" => {
                        messages.push(parse_struct(
                            &mut statement,
                            tags,
                            ContainerType::Msg(0),
                            file_info,
                        ));
                    }
                    keyword => panic!("invalid keyword for container: '{}'", keyword),
                }
            }
            Rule::test => {
                let mut statement = statement.into_inner();
                tests.push(parse_test(&mut statement, tags, file_info));
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
    let kvs = parse_object_key_values(&mut extra_kvs, &Tags::new());

    test_members.push(ParsedTestCaseMember::new(member_name, member_value, kvs));
}

fn parse_test(t: &mut Pairs<Rule>, tags: &Tags, file_info: FileInfo) -> ParsedTestCase {
    let name = t.next().unwrap().as_str();
    let members = t.next().unwrap().into_inner();

    let mut test_members = Vec::new();
    for m in members {
        parse_test_values(m, &mut test_members);
    }

    let mut raw_bytes = Vec::new();
    let raw_values = t.next().unwrap().into_inner();
    for r in raw_values {
        raw_bytes.push(parse_value(r.as_str()).unwrap() as u8);
    }

    let mut extra_kvs = t.find(|a| a.as_rule() == Rule::object_key_values);
    let kvs = parse_object_key_values(&mut extra_kvs, tags);

    ParsedTestCase::new(name, test_members, raw_bytes, kvs, file_info)
}

fn parse_struct(
    t: &mut Pairs<Rule>,
    tags: &Tags,
    container_type: ContainerType,
    file_info: FileInfo,
) -> ParsedContainer {
    let identifier = t.next().unwrap().as_str();

    let opcode = t.clone().find(|a| a.as_rule() == Rule::value);
    let container_type = if let Some(opcode) = opcode {
        if let Some(v) = parse_value(opcode.as_str()) {
            match container_type {
                ContainerType::Struct => panic!("struct has opcode"),
                ContainerType::CLogin(_) => ContainerType::CLogin(v as u16),
                ContainerType::SLogin(_) => ContainerType::SLogin(v as u16),
                ContainerType::Msg(_) => ContainerType::Msg(v as u16),
                ContainerType::CMsg(_) => ContainerType::CMsg(v as u16),
                ContainerType::SMsg(_) => ContainerType::SMsg(v as u16),
            }
        } else {
            match container_type {
                ContainerType::Struct => ContainerType::Struct,
                _ => panic!("non-struct missing opcode"),
            }
        }
    } else {
        match container_type {
            ContainerType::Struct => ContainerType::Struct,
            _ => panic!("non-struct missing opcode"),
        }
    };

    let container_members = t
        .find(|a| a.as_rule() == Rule::container_members)
        .unwrap_or_else(|| panic!("no members for struct {}", identifier))
        .into_inner();

    let mut members = Vec::new();

    for member in container_members {
        let member = member.into_inner().next().unwrap();
        if matches!(member.as_rule(), Rule::unimplemented) {
            let mut extra_kvs = t.find(|a| a.as_rule() == Rule::object_key_values);
            let mut kvs = parse_object_key_values(&mut extra_kvs, tags);
            kvs.push(Tag::new("unimplemented", "true"));
            let v = vec![unimplemented_member()];

            return ParsedContainer::new(identifier, v, kvs, container_type, file_info);
        }
        members.push(parse_struct_member(member));
    }

    let mut extra_kvs = t.find(|a| a.as_rule() == Rule::object_key_values);
    let kvs = parse_object_key_values(&mut extra_kvs, tags);

    ParsedContainer::new(identifier, members, kvs, container_type, file_info)
}

fn unimplemented_member() -> ParsedStructMember {
    ParsedStructMember::Definition(ParsedStructMemberDefinition::new(
        "unimplemented",
        Type::Array(Array::new_unimplemented()),
        None,
        Tags::new(),
    ))
}

fn parse_struct_member(mut t: Pair<Rule>) -> ParsedStructMember {
    if t.as_rule() == Rule::container_member {
        t = t.into_inner().next().unwrap();
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
            let mut kvs = Tags::new();
            for kv in complex_key_value_pairs {
                let mut kv = kv.into_inner();
                let identifier = kv.find(|a| a.as_rule() == Rule::identifier).unwrap();
                let text = kv.find(|a| a.as_rule() == Rule::text).unwrap();
                kvs.push(Tag::new(identifier.as_str(), text.as_str()));
            }

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

            let container_type = if let Some(ty) = upcasted_type {
                Type::with_upcast(container_type, ty)
            } else {
                Type::from_str(container_type)
            };

            let identifier = identifier_and_value
                .find(|a| a.as_rule() == Rule::identifier)
                .unwrap();

            let s =
                ParsedStructMemberDefinition::new(identifier.as_str(), container_type, value, kvs);

            ParsedStructMember::Definition(s)
        }
        Rule::if_statement => {
            let mut f = t.into_inner();

            let (conditions, members) = parse_if_statement(&mut f);

            let else_if_statement = f.clone().filter(|a| a.as_rule() == Rule::else_if_statement);
            let mut else_ifs = Vec::new();
            for statement in else_if_statement {
                let (conditions, members) = parse_if_statement(&mut statement.into_inner());
                else_ifs.push(ParsedIfStatement::new(
                    Conditional::new(&conditions),
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
                    v.push(parse_struct_member(member));
                }

                v
            } else {
                Vec::new()
            };

            let conditional = Conditional::new(&conditions);
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
                v.push(parse_struct_member(member));
            }

            ParsedStructMember::OptionalStatement(ParsedOptionalStatement::new(name, v))
        }
        p => {
            unreachable!("invalid member for struct: {:#?}", p)
        }
    }
}

fn parse_if_statement(f: &mut Pairs<Rule>) -> (Vec<Condition>, Vec<ParsedStructMember>) {
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
            members.push(parse_struct_member(m));
        }

        members
    } else {
        Vec::new()
    };

    (conditions, members)
}

pub(crate) fn parse_enum(
    t: &mut Pairs<Rule>,
    tags: &Tags,
    file_info: FileInfo,
    definer_ty: DefinerType,
) -> ParsedDefiner {
    let ident = t.next().unwrap();
    assert_eq!(ident.as_rule(), Rule::identifier);

    let basic_type = t.next().unwrap();
    assert_eq!(basic_type.as_rule(), Rule::basic_type);
    let definer_members = t.next().unwrap().into_inner();

    let mut fields = Vec::new();

    let mut self_value = None;

    for item in definer_members {
        let item_rule = item.as_rule();
        assert!(item_rule == Rule::basic_definer_item || item_rule == Rule::complex_definer_item);
        let mut complex_key_values = item.clone().into_inner();
        let mut item = item.into_inner().next().unwrap().into_inner();
        let identifier = item.next().unwrap();
        let value = item.next().unwrap();

        let mut kvs = Tags::new();
        if item_rule == Rule::complex_definer_item {
            let _ = complex_key_values.next().unwrap();
            let item = complex_key_values;
            for kv in item {
                let mut inner = kv.into_inner();
                let ident = inner.next().unwrap().as_str();
                let text = inner.next().unwrap().as_str();

                kvs.push(Tag::new(ident, text));
            }
        }

        if value.as_str().contains(ENUM_SELF_VALUE_FIELD) {
            if self_value.is_some() {
                panic!("only one self value allowed")
            }
            self_value = Some(SelfValueDefinerField::new(identifier.as_str(), kvs));
        } else {
            fields.push(DefinerField::new(
                identifier.as_str(),
                value.as_str().into(),
                kvs,
            ));
        }
    }

    let mut extra_key_values = t.next();
    let mut extras = parse_object_key_values(&mut extra_key_values, tags);
    extras.append(tags);

    ParsedDefiner::new(
        ident.as_str(),
        definer_ty,
        fields,
        basic_type.as_str().into(),
        self_value,
        extras,
        file_info,
    )
}

fn parse_object_key_values(t: &mut Option<Pair<Rule>>, tags: &Tags) -> Tags {
    match t {
        None => tags.clone(),
        Some(s) => {
            let s = s.clone().into_inner();
            let mut v = Tags::new();
            for kv in s {
                let mut kv = kv.into_inner();

                let key = kv.next().unwrap().as_str();
                let value = kv.next().unwrap().as_str();

                v.append_or_insert(key, value);
            }
            v.append(tags);
            v
        }
    }
}
