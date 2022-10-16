use std::collections::BTreeSet;

use crate::error_printer::{object_has_both_versions, object_has_no_versions};
use crate::file_info::FileInfo;
use crate::parser::types::tags::{MemberTags, TagString};
use crate::parser::types::version::{AllVersions, LoginVersion, WorldVersion};
use crate::{
    ObjectTags, COMMENT, COMPRESSED, DESCRIPTION, DISPLAY, LOGIN_VERSIONS, PASTE_VERSIONS,
    RUST_BASE_TYPE, SKIP_SERIALIZE, SKIP_STR, TEST_STR, UNIMPLEMENTED, VERSIONS,
};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub(crate) struct ParsedTags {
    login_versions: BTreeSet<LoginVersion>,
    world_versions: BTreeSet<WorldVersion>,
    description: Option<TagString>,
    compressed: Option<String>,
    comment: Option<TagString>,
    display: Option<String>,
    paste_versions: BTreeSet<WorldVersion>,

    skip_serialize: Option<bool>,
    is_test: Option<bool>,
    skip: Option<bool>,
    unimplemented: Option<bool>,
    rust_base_ty: Option<bool>,
}

impl ParsedTags {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn append(&mut self, mut t: ParsedTags) {
        self.login_versions.append(&mut t.login_versions);

        self.world_versions.append(&mut t.world_versions);

        if let Some(v) = t.description {
            self.description = Some(v);
        }

        if let Some(v) = t.compressed {
            self.compressed = Some(v);
        }

        if let Some(v) = t.comment {
            self.comment = Some(v);
        }

        if let Some(v) = t.display {
            self.display = Some(v);
        }

        self.paste_versions.append(&mut t.paste_versions);

        if let Some(v) = t.is_test {
            self.is_test = Some(v)
        }
        if let Some(v) = t.skip {
            self.skip = Some(v)
        }
        if let Some(v) = t.unimplemented {
            self.unimplemented = Some(v)
        }
        if let Some(v) = t.rust_base_ty {
            self.rust_base_ty = Some(v)
        }
    }

    pub(crate) fn into_tags(self, ty_name: &str, file_info: &FileInfo) -> ObjectTags {
        let all_versions = if !self.world_versions.is_empty() {
            if !self.login_versions.is_empty() {
                object_has_both_versions(ty_name, file_info);
            }
            AllVersions::World(self.world_versions)
        } else if !self.login_versions.is_empty() {
            if !self.world_versions.is_empty() {
                object_has_both_versions(ty_name, file_info);
            }
            AllVersions::Login(self.login_versions)
        } else {
            object_has_no_versions(ty_name, file_info)
        };

        ObjectTags::from_parsed(
            all_versions,
            self.description,
            self.compressed,
            self.comment,
            self.is_test,
            self.skip,
            self.unimplemented,
            self.rust_base_ty,
        )
    }

    pub(crate) fn into_member_tags(self) -> MemberTags {
        MemberTags::from_parsed(
            self.description,
            self.compressed,
            self.comment,
            self.display,
            self.skip_serialize,
        )
    }

    pub(crate) fn paste_versions(&self) -> impl Iterator<Item = WorldVersion> {
        self.paste_versions.clone().into_iter()
    }

    pub(crate) fn push_version(&mut self, version: WorldVersion) {
        self.world_versions.insert(version);
    }

    pub(crate) fn insert(&mut self, key: &str, value: &str) {
        if key == LOGIN_VERSIONS {
            for w in value.split_whitespace() {
                if let Ok(v) = w.parse::<u8>() {
                    if self.world_versions.get(&WorldVersion::All).is_none() {
                        self.login_versions.insert(LoginVersion::Specific(v));
                    }
                } else if w == "*" {
                    self.login_versions.clear();
                    self.login_versions.insert(LoginVersion::All);
                } else {
                    panic!("invalid value passed as login_logon_versions: '{}'", w);
                }
            }
        } else if key == VERSIONS {
            for w in value.split_whitespace() {
                if let Ok(v) = w.parse::<u8>() {
                    if self.world_versions.get(&WorldVersion::All).is_none() {
                        self.world_versions.insert(WorldVersion::Major(v));
                        continue;
                    }
                } else if w == "*" {
                    self.world_versions.clear();
                    self.world_versions.insert(WorldVersion::All);
                    continue;
                }

                let d: Vec<u8> = w.split('.').map(|a| a.parse::<u8>().unwrap()).collect();
                if self.world_versions.get(&WorldVersion::All).is_none() {
                    self.world_versions.insert(match d.len() {
                        2 => WorldVersion::Minor(d[0], d[1]),
                        3 => WorldVersion::Patch(d[0], d[1], d[2]),
                        4 => WorldVersion::Exact(d[0], d[1], d[2], u16::from(d[3])),
                        _ => panic!("incorrect world version string"),
                    });
                }
            }
        } else if key == PASTE_VERSIONS {
            for w in value.split_whitespace() {
                if let Ok(v) = w.parse::<u8>() {
                    self.paste_versions.insert(WorldVersion::Major(v));
                    continue;
                } else if w == "*" {
                    panic!(
                        "Got all version for paste_versions, this is not valid, {:#?}",
                        self
                    );
                }

                let d: Vec<u8> = w.split('.').map(|a| a.parse::<u8>().unwrap()).collect();
                self.paste_versions.insert(match d.len() {
                    2 => WorldVersion::Minor(d[0], d[1]),
                    3 => WorldVersion::Patch(d[0], d[1], d[2]),
                    4 => WorldVersion::Exact(d[0], d[1], d[2], u16::from(d[3])),
                    _ => panic!("incorrect world version string"),
                });
            }
        } else if key == DESCRIPTION {
            if let Some(desc) = &mut self.description {
                desc.add(value);
            } else {
                let mut t = TagString::new();
                t.add(value);
                self.description = Some(t);
            }
        } else if key == COMPRESSED {
            self.compressed = Some(value.to_owned());
        } else if key == SKIP_SERIALIZE {
            self.skip_serialize = Some(value.eq("true"))
        } else if key == COMMENT {
            if let Some(comment) = &mut self.comment {
                comment.add(value);
            } else {
                let mut t = TagString::new();
                t.add(value);
                self.comment = Some(t);
            }
        } else if key == DISPLAY {
            self.display = Some(value.to_string());
        } else if key == TEST_STR {
            self.is_test = Some(value.eq("true"));
        } else if key == SKIP_STR {
            self.skip = Some(value.eq("true"));
        } else if key == UNIMPLEMENTED {
            self.unimplemented = Some(value.eq("true"));
        } else if key == RUST_BASE_TYPE {
            self.rust_base_ty = Some(value.eq("true"));
        }
    }
}
