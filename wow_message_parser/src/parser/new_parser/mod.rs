#![allow(unused)]
use crate::file_info::FileInfo;
use crate::parser::types::container::ContainerType;
use crate::parser::types::definer::{DefinerField, DefinerValue};
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_definer::ParsedDefiner;
use crate::parser::types::parsed::parsed_if_statement::ParsedIfStatement;
use crate::parser::types::parsed::parsed_object::ParsedObjects;
use crate::parser::types::parsed::parsed_optional::ParsedOptionalStatement;
use crate::parser::types::parsed::parsed_struct_member::ParsedStructMember;
use crate::parser::types::parsed::parsed_tags::ParsedTags;
use crate::parser::types::parsed::parsed_test_case::ParsedTestCase;
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::IntegerType;
use crate::parser::{apply_tags, Commands};
use crate::rust_printer::DefinerType;
use std::iter::Peekable;
use std::path::{Path, PathBuf};
use std::slice::Iter;
use std::str::CharIndices;

pub(crate) fn parse_contents_new(contents: &str, filename: &Path) -> ParsedObjects {
    let file = parse_file(contents, filename.to_str().unwrap().to_string());

    let mut iter = file.iter().peekable();

    let filename = filename.to_path_buf();
    let commands = parse_commands(&mut iter, filename.clone());

    parse_body(commands, iter, filename)
}

fn parse_body(
    commands: Commands,
    mut iter: Peekable<Iter<Token>>,
    filename: PathBuf,
) -> ParsedObjects {
    let mut enums = Vec::new();
    let mut flags = Vec::new();
    let mut structs = Vec::new();
    let mut messages = Vec::new();
    let mut tests = Vec::new();

    let mut comments = Vec::new();

    loop {
        let next = iter.next();
        match next {
            Some(Token {
                ty: TokenTy::DescriptiveComment(c),
                ..
            }) => {
                comments.push(c.clone());
            }
            Some(Token {
                ty:
                    TokenTy::CLogin
                    | TokenTy::SLogin
                    | TokenTy::SMsg
                    | TokenTy::CMsg
                    | TokenTy::Struct
                    | TokenTy::Msg,
                ..
            }) => {
                if next.unwrap().ty == TokenTy::Struct {
                    structs.append(&mut parse_container(
                        &next.unwrap().ty,
                        &comments,
                        &mut iter,
                        filename.clone(),
                        &commands,
                    ));
                } else {
                    messages.append(&mut parse_container(
                        &next.unwrap().ty,
                        &comments,
                        &mut iter,
                        filename.clone(),
                        &commands,
                    ));
                }
                comments.clear();
            }
            Some(Token {
                ty: TokenTy::Enum | TokenTy::Flag,
                ..
            }) => {
                if next.unwrap().ty == TokenTy::Enum {
                    enums.push(parse_definer(
                        &next.unwrap().ty,
                        &comments,
                        &mut iter,
                        filename.clone(),
                        &commands,
                    ));
                } else {
                    flags.push(parse_definer(
                        &next.unwrap().ty,
                        &comments,
                        &mut iter,
                        filename.clone(),
                        &commands,
                    ));
                }
                comments.clear();
            }
            Some(Token {
                ty: TokenTy::Test, ..
            }) => {
                /*
                tests.push(parse_test(&next.unwrap().ty, &comments));
                 */
                comments.clear();
            }
            Some(_) => {
                panic!("invalid token {next:?}")
            }
            None => break,
        }
    }

    ParsedObjects::new(enums, flags, structs, messages, tests)
}

#[allow(unused)]
fn require_token(mut iter: &mut Peekable<Iter<Token>>, t: &TokenTy, reason: &str) -> usize {
    let next = iter.next();
    if let Some(Token { ty, end, .. }) = next {
        if t != ty {
            panic!("Invalid token {next:?} during {reason}");
        }
        *end
    } else {
        panic!("unexpected end of file");
    }
}

fn parse_definer(
    ty: &TokenTy,
    comments: &Vec<String>,
    iter: &mut Peekable<Iter<Token>>,
    filename: PathBuf,
    commands: &Commands,
) -> ParsedDefiner {
    let mut next = iter.next();
    let Some(Token {
        ty: TokenTy::Identifier(name),
        start,
        ..
    }) = next
    else {
        panic!("Invalid name for definer: {next:?}");
    };

    require_token(iter, &TokenTy::Colon, "enum");

    next = iter.next();
    let Some(Token {
        ty: TokenTy::Identifier(base_ty),
        ..
    }) = next
    else {
        panic!("Invalid type for definer: {next:?}");
    };

    require_token(iter, &TokenTy::OpenCurly, "enum");

    let temp_file_info = FileInfo::new(filename.clone(), *start, *start);
    let fields = parse_definer_members(iter, &name, &temp_file_info);

    let current = require_token(iter, &TokenTy::ClosingCurly, "enum");

    let (end, extras) = parse_object_extras(iter, name, &temp_file_info, true, current, commands);

    let file_info = FileInfo::new(filename, *start, end);
    let basic_type = IntegerType::from_str(base_ty, name, &file_info);
    ParsedDefiner::new(
        name,
        if *ty == TokenTy::Enum {
            DefinerType::Enum
        } else {
            DefinerType::Flag
        },
        fields,
        basic_type,
        extras.into_tags(name, &file_info, true),
        file_info,
    )
}

fn parse_definer_members(
    iter: &mut Peekable<Iter<Token>>,
    definer_name: &str,
    file_info: &FileInfo,
) -> Vec<DefinerField> {
    let mut fields = Vec::new();

    loop {
        if let Some(Token {
            ty: TokenTy::ClosingCurly,
            ..
        }) = iter.peek()
        {
            return fields;
        }
        let mut next = iter.next();

        let mut comments = Vec::new();
        while let Some(Token {
            ty: TokenTy::DescriptiveComment(c),
            ..
        }) = next
        {
            comments.push(c.clone());
            next = iter.next();
        }

        let Some(Token {
            ty: TokenTy::Identifier(name),
            ..
        }) = next
        else {
            panic!("Invalid type for definer member {next:?}");
        };

        require_token(iter, &TokenTy::Equals, "definer member");

        next = iter.next();
        let Some(Token {
            ty: TokenTy::Integer { value, original },
            ..
        }) = next
        else {
            panic!("Invalid type for definer member {next:?}");
        };

        next = iter.next();
        let kvs = match next {
            Some(Token {
                ty: TokenTy::SemiColon,
                ..
            }) => ParsedTags::new(),
            Some(Token {
                ty: TokenTy::OpenCurly,
                end,
                ..
            }) => parse_key_value_tags(iter, definer_name, file_info, true, *end).1,
            _ => panic!("invalid termination for definer value {next:?}"),
        };

        fields.push(DefinerField::new(
            name,
            DefinerValue::from_str(original, definer_name, name, file_info),
            kvs.into_member_tags(),
        ));
    }
}

fn parse_object_extras(
    iter: &mut Peekable<Iter<Token>>,
    ty_name: &str,
    file_info: &FileInfo,
    rust_base_type_default: bool,
    current: usize,
    commands: &Commands,
) -> (usize, ParsedTags) {
    let peek = iter.peek();

    match peek {
        Some(Token {
            ty: TokenTy::OpenCurly,
            ..
        }) => {
            let _ = iter.next();
            let (end, mut t) =
                parse_key_value_tags(iter, ty_name, file_info, rust_base_type_default, current);
            t.append(commands.tags.clone(), ty_name, file_info);
            (end, t)
        }
        _ => {
            let mut t = ParsedTags::new();
            t.append(commands.tags.clone(), ty_name, file_info);
            (current, t)
        }
    }
}

fn parse_key_value_tags(
    iter: &mut Peekable<Iter<Token>>,
    ty_name: &str,
    file_info: &FileInfo,
    rust_base_type_default: bool,
    current: usize,
) -> (usize, ParsedTags) {
    let mut tags = ParsedTags::new();

    loop {
        let mut next = iter.next();
        if let Some(Token {
            ty: TokenTy::ClosingCurly,
            end,
            ..
        }) = next
        {
            return (*end, tags);
        }

        let Some(Token {
            ty: TokenTy::Identifier(name),
            ..
        }) = next
        else {
            panic!("Invalid identifier for tags {next:?}");
        };

        require_token(iter, &TokenTy::Equals, "tags");

        next = iter.next();
        let Some(Token {
            ty: TokenTy::Value(value),
            ..
        }) = next
        else {
            panic!("Invalid value for tags {next:?}");
        };

        require_token(iter, &TokenTy::SemiColon, "tags");

        tags.insert(name, value, ty_name, file_info);
    }
}

fn parse_container(
    ty: &TokenTy,
    comments: &Vec<String>,
    iter: &mut Peekable<Iter<Token>>,
    filename: PathBuf,
    commands: &Commands,
) -> Vec<ParsedContainer> {
    let mut next = iter.next();
    let Some(Token {
        ty: TokenTy::Identifier(name),
        start,
        ..
    }) = next
    else {
        panic!("Invalid name for container: {next:?}");
    };

    let opcode = if *ty != TokenTy::Struct {
        require_token(iter, &TokenTy::Equals, "container");

        let Some(Token {
            ty: TokenTy::Integer { value, original },
            ..
        }) = next
        else {
            panic!("Invalid opcode for container: {next:?}");
        };

        match *ty {
            TokenTy::CLogin => ContainerType::CLogin(*value as u16),
            TokenTy::SLogin => ContainerType::SLogin(*value as u16),
            TokenTy::SMsg => ContainerType::SMsg(*value as u16),
            TokenTy::CMsg => ContainerType::CMsg(*value as u16),
            TokenTy::Msg => ContainerType::Msg(*value as u16),
            _ => unreachable!(""),
        }
    } else {
        ContainerType::Struct
    };

    require_token(iter, &TokenTy::OpenCurly, "container");

    let fields = parse_container_fields(iter);

    let current = require_token(iter, &TokenTy::ClosingCurly, "container");

    let temp_file_info = FileInfo::new(filename.clone(), *start, *start);
    let (end, extras) = parse_object_extras(iter, name, &temp_file_info, false, current, commands);

    let file_info = FileInfo::new(filename, *start, end);

    let mut objects = Vec::new();
    apply_tags(name, fields, extras, file_info, opcode, &mut objects);
    objects
}

fn parse_container_fields(iter: &mut Peekable<Iter<Token>>) -> Vec<ParsedStructMember> {
    let mut fields = vec![];

    loop {
        if let Some(Token {
            ty: TokenTy::ClosingCurly,
            ..
        }) = iter.peek()
        {
            return fields;
        }
        let mut next = iter.next();

        let mut comments = Vec::new();
        while let Some(Token {
            ty: TokenTy::DescriptiveComment(c),
            ..
        }) = next
        {
            comments.push(c.clone());
            next = iter.next();
        }

        match next {
            None => unreachable!(),
            Some(Token {
                ty: TokenTy::If, ..
            }) => fields.push(parse_if_statement()),
            Some(Token {
                ty: TokenTy::Optional,
                ..
            }) => fields.push(parse_optional_statement()),
            _ => fields.push(parse_definition()),
        }
    }
}

fn parse_if_statement() -> ParsedStructMember {
    todo!()
}

fn parse_optional_statement() -> ParsedStructMember {
    todo!()
}

fn parse_definition() -> ParsedStructMember {
    todo!()
}

fn parse_test(ty: &TokenTy, comments: &Vec<String>, iter: &mut Iter<Token>) -> ParsedTestCase {
    todo!()
}

fn parse_commands(iter: &mut Peekable<Iter<Token>>, filename: PathBuf) -> Commands {
    let mut tags = ParsedTags::new();

    while let Some(Token {
        ty: TokenTy::HashMark,
        ..
    }) = iter.peek()
    {
        let mut token = iter.next();
        token = iter.next();
        let Some(Token {
            ty: TokenTy::Identifier(command),
            ..
        }) = token
        else {
            panic!("invalid token after #");
        };
        token = iter.next();

        match command.as_ref() {
            "tag_all" => {}
            _ => {
                panic!("invalid command {command}")
            }
        }

        let Some(Token {
            ty: TokenTy::Identifier(key),
            ..
        }) = token
        else {
            panic!("invalid token for #tag_all");
        };
        token = iter.next();

        let Some(Token {
            ty: TokenTy::Value(value),
            ..
        }) = token
        else {
            panic!("invalid token after #tag_all key");
        };
        token = iter.next();

        tags.insert(
            key.as_str(),
            value.as_str(),
            "tag_all",
            &FileInfo::new(filename.clone(), 0, 0),
        );

        if !matches!(
            token,
            Some(Token {
                ty: TokenTy::SemiColon,
                ..
            })
        ) {
            panic!("Missing ; for #tag_all")
        }
    }

    Commands::new(tags)
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Token {
    ty: TokenTy,
    filename: String,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum TokenTy {
    HashMark,
    Dash,
    SemiColon,
    Comma,
    Colon,
    OpenParens,
    ClosingParens,
    OpenCurly,
    ClosingCurly,
    OpenSquareBracket,
    ClosingSquareBracket,
    Equals,
    DoubleEquals,
    NotEquals,
    BitwiseAnd,
    BitwiseOr,
    Slash,
    And,
    Or,
    If,
    Optional,
    Else,
    Enum,
    Flag,
    Struct,
    CLogin,
    SLogin,
    SMsg,
    CMsg,
    Msg,
    Test,
    Identifier(String),
    Value(String),
    LineComment(String),
    BlockComment(String),
    DescriptiveComment(String),
    Integer { value: i128, original: String },
    Float { original: String },
}

struct Tokenizer<'a> {
    contents: Peekable<CharIndices<'a>>,
    filename: String,
}

impl<'a> Tokenizer<'a> {
    pub fn new(contents: &'a str, filename: String) -> Self {
        Self {
            contents: contents.char_indices().peekable(),
            filename,
        }
    }

    fn peek_next_char(&mut self) -> Option<(usize, char)> {
        self.contents.peek().cloned()
    }

    fn next_char(&mut self) -> Option<(usize, char)> {
        self.contents.next()
    }

    pub fn next(&mut self) -> Option<Token> {
        loop {
            let (mut start, mut char) = self.next_char()?;
            let mut end = start + 1;
            let filename = self.filename.clone();

            while char.is_whitespace() {
                (start, char) = self.next_char()?;
            }

            let mut next = self.peek_next_char();

            let mut chars = vec![];

            fn char_to_token(ch: char, next: Option<char>) -> Option<TokenTy> {
                match (ch, next) {
                    ('/', Some('*')) => None,
                    ('/', Some('/')) => None,
                    ('/', Some(_)) => Some(TokenTy::Slash),
                    ('=', Some('=')) => Some(TokenTy::DoubleEquals),
                    ('!', Some('=')) => Some(TokenTy::NotEquals),
                    ('&', Some('&')) => Some(TokenTy::And),
                    ('|', Some('"')) => Some(TokenTy::Or),
                    ('-', Some(c)) => {
                        if c.is_numeric() {
                            None
                        } else {
                            Some(TokenTy::Dash)
                        }
                    }
                    ('#', _) => Some(TokenTy::HashMark),
                    (':', _) => Some(TokenTy::Colon),
                    (';', _) => Some(TokenTy::SemiColon),
                    (',', _) => Some(TokenTy::Comma),
                    ('{', _) => Some(TokenTy::OpenCurly),
                    ('}', _) => Some(TokenTy::ClosingCurly),
                    ('[', _) => Some(TokenTy::OpenSquareBracket),
                    (']', _) => Some(TokenTy::ClosingSquareBracket),
                    ('(', _) => Some(TokenTy::OpenParens),
                    (')', _) => Some(TokenTy::ClosingParens),
                    ('=', _) => Some(TokenTy::Equals),
                    ('&', _) => Some(TokenTy::BitwiseAnd),
                    ('|', _) => Some(TokenTy::BitwiseOr),
                    _ => None,
                }
            }

            if let Some(ty) = char_to_token(char, next.map(|(_, c)| c)) {
                return Some(Token {
                    ty,
                    filename,
                    start,
                    end: start + 1,
                });
            } else if char == '/'
                && next.is_some()
                && (next.unwrap().1 == '*' || next.unwrap().1 == '/')
            {
                (end, char) = next.unwrap();
                next = self.peek_next_char();
                if char == '/' {
                    (end, char) = next.unwrap();
                    next = self.peek_next_char();
                    if char == '/' {
                        loop {
                            if next.is_none() {
                                panic!("Unterminated value at {}:{start}", self.filename);
                            }

                            (end, char) = self.next_char().unwrap();
                            if char == '\n' {
                                break;
                            }

                            chars.push(char);

                            next = self.peek_next_char();
                        }

                        return Some(Token {
                            ty: TokenTy::DescriptiveComment(chars.iter().collect()),
                            filename,
                            start,
                            end,
                        });
                    } else {
                        loop {
                            if next.is_none() {
                                panic!("Unterminated value at {}:{start}", self.filename);
                            }

                            (end, char) = self.next_char().unwrap();
                            if char == '\n' {
                                break;
                            }

                            next = self.peek_next_char();
                        }
                        continue;
                    }
                } else {
                    loop {
                        if next.is_none() {
                            panic!("Unterminated value at {}:{start}", self.filename);
                        }

                        if let ('*', Some((_, '/'))) = (char, next) {
                            break;
                        }

                        (end, char) = self.next_char().unwrap();

                        chars.push(char);

                        next = self.peek_next_char();
                    }

                    return Some(Token {
                        ty: TokenTy::Value(chars.iter().collect()),
                        filename,
                        start,
                        end,
                    });
                }
            } else if char == '"' {
                loop {
                    if next.is_none() {
                        panic!("Unterminated value at {}:{start}", self.filename);
                    }

                    (end, char) = self.next_char().unwrap();
                    if char == '"' {
                        break;
                    }

                    chars.push(char);

                    next = self.peek_next_char();
                }

                return Some(Token {
                    ty: TokenTy::Value(chars.iter().collect()),
                    filename,
                    start,
                    end,
                });
            } else if char.is_numeric() || char == '-' {
                let negative = if char == '-' {
                    (end, char) = self.next_char().unwrap();
                    next = self.peek_next_char();
                    true
                } else {
                    false
                };
                let mut original = vec![];

                let base = if let Some((_, n)) = next {
                    if n == 'x' || n == 'b' {
                        original.push(char);
                        (end, char) = self.next_char().unwrap();
                        next = self.peek_next_char();
                        original.push(char);
                        (end, char) = self.next_char().unwrap();
                        next = self.peek_next_char();
                        if n == 'x' {
                            16
                        } else {
                            2
                        }
                    } else {
                        10
                    }
                } else {
                    10
                };

                loop {
                    let Some((_, n)) = next else {
                        panic!("Unterminated value at {}:{start}", self.filename);
                    };

                    chars.push(char);

                    if !(n.is_ascii_hexdigit() || n == '.') {
                        break;
                    }

                    (end, char) = self.next_char().unwrap();

                    next = self.peek_next_char();
                }

                if negative {
                    chars.insert(0, '-');
                }
                let number = chars.iter().collect::<String>();
                let mut original = original.iter().collect::<String>();
                original += number.as_str();

                return if number.contains(".") {
                    let _ = number.parse::<f64>().unwrap();
                    Some(Token {
                        ty: TokenTy::Float { original },
                        filename,
                        start,
                        end,
                    })
                } else {
                    let value = i128::from_str_radix(&number, base).unwrap();
                    Some(Token {
                        ty: TokenTy::Integer { value, original },
                        filename,
                        start,
                        end,
                    })
                };
            } else if char.is_alphabetic() || char == '_' {
                while char.is_alphanumeric() || char == '_' {
                    if next.is_none() {
                        panic!("Unterminated value at {}:{start}", self.filename);
                    }

                    chars.push(char);
                    (end, char) = self.next_char().unwrap();

                    next = self.peek_next_char();
                }

                let identifier = chars.iter().collect::<String>();

                return Some(Token {
                    ty: match identifier.as_ref() {
                        "if" => TokenTy::If,
                        "optional" => TokenTy::Optional,
                        "else" => TokenTy::Else,
                        "struct" => TokenTy::Struct,
                        "clogin" => TokenTy::CLogin,
                        "slogin" => TokenTy::SLogin,
                        "smsg" => TokenTy::SMsg,
                        "cmsg" => TokenTy::CMsg,
                        "msg" => TokenTy::Msg,
                        "enum" => TokenTy::Enum,
                        "flag" => TokenTy::Flag,
                        _ => TokenTy::Identifier(identifier),
                    },
                    filename,
                    start,
                    end,
                });
            }

            panic!("unknown token: '{char}' in wow_message_parser/{filename}:{end}")
        }
    }
}

fn parse_file(contents: &str, filename: String) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(contents, filename);
    let mut v = vec![];

    while let Some(token) = tokenizer.next() {
        v.push(token);
    }

    v
}

#[cfg(test)]
mod test {
    use crate::parser::new_parser::{parse_contents_new, parse_file, Token, TokenTy};
    use std::fs::read_to_string;
    use walkdir::WalkDir;

    #[test]
    fn test_parser() {
        let dir = "wowm/login/".to_string();

        for file in WalkDir::new(dir).into_iter().filter_map(|a| a.ok()) {
            if !file.file_type().is_file() {
                continue;
            }

            let filename = file.into_path();
            let file = read_to_string(&filename).unwrap();
            dbg!(parse_contents_new(&file, &filename));
        }
    }

    #[test]
    fn simple_commands() {
        let file = "\n\n#tag_all rust_base_type \"true\";\n\n#tag_all paste_versions \"1.12 2.4.3 3.3.5\";";

        let filename = "testfilename.wowm".to_string();
        let tokens = parse_file(file, filename.clone());

        assert_eq!(
            tokens[0],
            Token {
                ty: TokenTy::HashMark,
                filename: filename.clone(),
                start: 2,
                end: 3,
            }
        );
        assert_eq!(
            tokens[1],
            Token {
                ty: TokenTy::Identifier("tag_all".to_string()),
                filename: filename.clone(),
                start: 3,
                end: 10,
            }
        );
        assert_eq!(
            tokens[2],
            Token {
                ty: TokenTy::Identifier("rust_base_type".to_string()),
                filename: filename.clone(),
                start: 11,
                end: 25,
            }
        );
        assert_eq!(
            tokens[3],
            Token {
                ty: TokenTy::Value("true".to_string()),
                filename: filename.clone(),
                start: 26,
                end: 31,
            }
        );
        assert_eq!(
            tokens[4],
            Token {
                ty: TokenTy::SemiColon,
                filename: filename.clone(),
                start: 32,
                end: 33,
            }
        );
        assert_eq!(
            tokens[5],
            Token {
                ty: TokenTy::HashMark,
                filename: filename.clone(),
                start: 35,
                end: 36,
            }
        );
        assert_eq!(
            tokens[6],
            Token {
                ty: TokenTy::Identifier("tag_all".to_string()),
                filename: filename.clone(),
                start: 36,
                end: 43,
            }
        );
        assert_eq!(
            tokens[7],
            Token {
                ty: TokenTy::Identifier("paste_versions".to_string()),
                filename: filename.clone(),
                start: 44,
                end: 58,
            }
        );
        assert_eq!(
            tokens[8],
            Token {
                ty: TokenTy::Value("1.12 2.4.3 3.3.5".to_string()),
                filename: filename.clone(),
                start: 59,
                end: 76,
            }
        );
        assert_eq!(
            tokens[9],
            Token {
                ty: TokenTy::SemiColon,
                filename: filename.clone(),
                start: 77,
                end: 78,
            }
        );
    }
}
